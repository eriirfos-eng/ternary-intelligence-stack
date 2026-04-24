use crate::error::ApiError;
use crate::sse::SseParser;
use crate::types::*;
use std::collections::VecDeque;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.ternlang.com";
const REQUEST_ID_HEADER: &str = "x-request-id";
const ALT_REQUEST_ID_HEADER: &str = "request-id";
const DEFAULT_INITIAL_BACKOFF: Duration = Duration::from_millis(500);
const DEFAULT_MAX_BACKOFF: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LlmProvider {
    Ternlang,
    Anthropic,
    OpenAi,
    HuggingFace,
    Google,
    Azure,
    Aws,
    Ollama,
    Xai,
}

impl LlmProvider {
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Self::Ternlang => "https://api.ternlang.com",
            Self::Anthropic => "https://api.anthropic.com",
            Self::OpenAi => "https://api.openai.com",
            Self::HuggingFace => "https://api-inference.huggingface.co",
            Self::Google => "https://generativelanguage.googleapis.com",
            Self::Azure => "https://api.azure.com",
            Self::Aws => "https://bedrock-runtime.us-east-1.amazonaws.com",
            Self::Ollama => "http://localhost:11434",
            Self::Xai => "https://api.x.ai",
        }
    }

    pub fn api_path(&self) -> &'static str {
        match self {
            Self::Ternlang => "/v1/messages",
            Self::Anthropic => "/v1/messages",
            Self::OpenAi => "/v1/chat/completions",
            Self::HuggingFace => "/models",
            Self::Google => "/v1beta",
            Self::Ollama => "/v1/chat/completions",
            Self::Xai => "/v1/chat/completions",
            _ => "/v1/messages",
        }
    }
}

#[derive(Clone)]
pub struct TernlangClient {
    pub provider: LlmProvider,
    pub base_url: String,
    pub auth: AuthSource,
    pub http: reqwest::Client,
    pub max_retries: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

impl TernlangClient {
    pub fn from_auth(auth: AuthSource) -> Self {
        Self {
            provider: LlmProvider::Ternlang,
            base_url: DEFAULT_BASE_URL.to_string(),
            auth,
            http: reqwest::Client::new(),
            max_retries: 3,
            initial_backoff: DEFAULT_INITIAL_BACKOFF,
            max_backoff: DEFAULT_MAX_BACKOFF,
        }
    }

    pub fn from_env() -> Result<Self, ApiError> {
        Ok(Self::from_auth(AuthSource::from_env_or_saved()?).with_base_url(read_base_url()))
    }

    #[must_use]
    pub fn with_auth_source(mut self, auth: AuthSource) -> Self {
        self.auth = auth;
        self
    }

    #[must_use]
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    #[must_use]
    pub fn with_provider(mut self, provider: LlmProvider) -> Self {
        self.provider = provider;
        if self.base_url == DEFAULT_BASE_URL {
            self.base_url = provider.default_base_url().to_string();
        }
        self
    }

    async fn send_raw_request(
        &self,
        request: &MessageRequest,
    ) -> Result<reqwest::Response, ApiError> {
        let path = self.provider.api_path();
        let mut request_url = format!("{}/{}", self.base_url.trim_end_matches('/'), path.trim_start_matches('/'));

        let body = match self.provider {
            LlmProvider::Google => {
                let model_id = if request.model.starts_with("models/") {
                    request.model.clone()
                } else {
                    format!("models/{}", request.model)
                };
                let base = format!("{}/v1beta/{}:generateContent", self.base_url.trim_end_matches('/'), model_id);
                request_url = if let Some(key) = self.auth.api_key() {
                    format!("{}?key={}", base, key)
                } else {
                    base
                };
                translate_to_gemini(request)
            }
            LlmProvider::Anthropic => translate_to_anthropic(request),
            LlmProvider::OpenAi | LlmProvider::Ollama | LlmProvider::Xai => translate_to_openai(request),
            _ => serde_json::to_value(request).map_err(ApiError::from)?,
        };

        let mut request_builder = self
            .http
            .post(&request_url)
            .header("content-type", "application/json");

        if self.provider == LlmProvider::Anthropic {
            request_builder = request_builder.header("anthropic-version", "2023-06-01");
        }

        let request_builder = self.auth.apply(self.provider, request_builder);

        request_builder.json(&body).send().await.map_err(ApiError::from)
    }

    pub async fn send_message(
        &self,
        request: &MessageRequest,
    ) -> Result<MessageResponse, ApiError> {
        let request = MessageRequest {
            stream: false,
            ..request.clone()
        };
        let response = self.send_with_retry(&request).await?;
        let request_id = request_id_from_headers(response.headers());
        let response_json = response
            .json::<serde_json::Value>()
            .await
            .map_err(ApiError::from)?;
        
        let mut final_response = match self.provider {
            LlmProvider::Google => translate_from_gemini(response_json, &request.model),
            LlmProvider::Anthropic => translate_from_anthropic(response_json, &request.model),
            LlmProvider::OpenAi | LlmProvider::Ollama | LlmProvider::Xai => translate_from_openai(response_json, &request.model),
            _ => serde_json::from_value::<MessageResponse>(response_json).map_err(ApiError::from)?,
        };

        if final_response.request_id.is_none() {
            final_response.request_id = request_id;
        }
        Ok(final_response)
    }

    pub async fn stream_message(
        &mut self,
        request: &MessageRequest,
    ) -> Result<MessageStream, ApiError> {
        // Gemini SSE format differs from Anthropic's — use non-streaming and wrap events
        if self.provider == LlmProvider::Google {
            let non_stream_req = MessageRequest { stream: false, ..request.clone() };
            let buffered = self.send_message(&non_stream_req).await?;
            return Ok(MessageStream::from_buffered_response(buffered));
        }
        let response = self
            .send_with_retry(&request.clone().with_streaming())
            .await?;
        Ok(MessageStream {
            _request_id: request_id_from_headers(response.headers()),
            response: Some(response),
            parser: SseParser::new(),
            pending: VecDeque::new(),
            done: false,
        })
    }

    async fn send_with_retry(
        &self,
        request: &MessageRequest,
    ) -> Result<reqwest::Response, ApiError> {
        let mut attempts = 0;
        let mut last_error: Option<ApiError>;

        loop {
            attempts += 1;
            match self.send_raw_request(request).await {
                Ok(response) => match expect_success(response).await {
                    Ok(response) => return Ok(response),
                    Err(error) if error.is_retryable() && attempts <= self.max_retries => {
                        last_error = Some(error);
                    }
                    Err(error) => return Err(error),
                },
                Err(error) if error.is_retryable() && attempts <= self.max_retries => {
                    last_error = Some(error);
                }
                Err(error) => return Err(error),
            }

            if attempts > self.max_retries {
                break;
            }

            tokio::time::sleep(self.backoff_for_attempt(attempts)?).await;
        }

        Err(ApiError::RetriesExhausted {
            attempts,
            last_error: Box::new(last_error.unwrap_or(ApiError::Auth("Max retries exceeded without error capture".to_string()))),
        })
    }

    fn backoff_for_attempt(&self, attempt: u32) -> Result<Duration, ApiError> {
        let multiplier = 2_u32.pow(attempt.saturating_sub(1));
        Ok(self
            .initial_backoff
            .checked_mul(multiplier)
            .map_or(self.max_backoff, |delay| delay.min(self.max_backoff)))
    }

    pub async fn list_remote_models(&self) -> Result<Vec<String>, ApiError> {
        match self.provider {
            LlmProvider::Google => {
                let url = format!("{}/v1beta/models?key={}", self.base_url.trim_end_matches('/'), self.auth.api_key().unwrap_or(""));
                let res = self.http.get(&url).send().await.map_err(ApiError::from)?;
                let json: serde_json::Value = res.json().await.map_err(ApiError::from)?;
                
                let mut models = vec![];
                if let Some(list) = json.get("models").and_then(|m| m.as_array()) {
                    for m in list {
                        if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                            models.push(name.replace("models/", ""));
                        }
                    }
                }
                Ok(models)
            }
            LlmProvider::OpenAi | LlmProvider::Ollama | LlmProvider::Xai => {
                let url = format!("{}/v1/models", self.base_url.trim_end_matches('/'));
                let res = self.auth.apply(self.provider, self.http.get(&url)).send().await.map_err(ApiError::from)?;
                let json: serde_json::Value = res.json().await.map_err(ApiError::from)?;
                
                let mut models = vec![];
                if let Some(list) = json.get("data").and_then(|m| m.as_array()) {
                    for m in list {
                        if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                            models.push(id.to_string());
                        }
                    }
                }
                Ok(models)
            }
            _ => Ok(vec![])
        }
    }

    pub async fn exchange_oauth_code(
        &self,
        _config: OAuthConfig,
        _request: &OAuthTokenExchangeRequest,
    ) -> Result<RuntimeTokenSet, ApiError> {
        Ok(RuntimeTokenSet {
            access_token: "dummy_token".to_string(),
            refresh_token: None,
            expires_at: None,
            scopes: vec![],
        })
    }
}

#[derive(Debug)]
pub struct MessageStream {
    _request_id: Option<String>,
    response: Option<reqwest::Response>,
    parser: SseParser,
    pending: VecDeque<StreamEvent>,
    done: bool,
}

impl MessageStream {
    fn from_buffered_response(response: MessageResponse) -> Self {
        let mut pending = VecDeque::new();
        pending.push_back(StreamEvent::MessageStart(MessageStartEvent {
            message: response.clone(),
        }));
        for (i, block) in response.content.iter().enumerate() {
            let index = i as u32;
            pending.push_back(StreamEvent::ContentBlockStart(ContentBlockStartEvent {
                index,
                content_block: block.clone(),
            }));
            if let OutputContentBlock::Text { text } = block {
                pending.push_back(StreamEvent::ContentBlockDelta(ContentBlockDeltaEvent {
                    index,
                    delta: ContentBlockDelta::TextDelta { text: text.clone() },
                }));
            }
            pending.push_back(StreamEvent::ContentBlockStop(ContentBlockStopEvent { index }));
        }
        pending.push_back(StreamEvent::MessageDelta(MessageDeltaEvent {
            delta: MessageDelta {
                stop_reason: response.stop_reason,
                stop_sequence: response.stop_sequence,
            },
            usage: response.usage,
        }));
        pending.push_back(StreamEvent::MessageStop(MessageStopEvent {}));
        Self {
            _request_id: None,
            response: None,
            parser: SseParser::new(),
            pending,
            done: true,
        }
    }

    pub async fn next_event(&mut self) -> Result<Option<StreamEvent>, ApiError> {
        loop {
            if let Some(event) = self.pending.pop_front() {
                return Ok(Some(event));
            }
            if self.done { return Ok(None); }
            match self.response.as_mut() {
                None => {
                    self.done = true;
                    return Ok(None);
                }
                Some(response) => match response.chunk().await? {
                    None => {
                        self.done = true;
                        return Ok(None);
                    }
                    Some(chunk) => {
                        self.pending.extend(self.parser.push(&chunk)?);
                    }
                },
            }
        }
    }
}

fn translate_to_anthropic(request: &MessageRequest) -> serde_json::Value {
    use serde_json::json;
    let messages: Vec<serde_json::Value> = request.messages.iter().map(|msg| {
        let content: Vec<serde_json::Value> = msg.content.iter().map(|block| {
            match block {
                InputContentBlock::Text { text } => json!({ "type": "text", "text": text }),
                InputContentBlock::ToolUse { id, name, input } => json!({
                    "type": "tool_use", "id": id, "name": name, "input": input
                }),
                InputContentBlock::ToolResult { tool_use_id, content, is_error } => {
                    let text = content.iter().filter_map(|c| {
                        if let ToolResultContentBlock::Text { text } = c { Some(text.clone()) } else { None }
                    }).collect::<Vec<String>>().join("\n");
                    json!({
                        "type": "tool_result", "tool_use_id": tool_use_id, "content": text, "is_error": is_error
                    })
                }
            }
        }).collect();
        json!({ "role": msg.role, "content": content })
    }).collect();

    let mut body = json!({
        "model": request.model,
        "messages": messages,
        "max_tokens": request.max_tokens.unwrap_or(4096),
        "stream": request.stream
    });
    if let Some(system) = &request.system { body["system"] = json!(system); }
    if let Some(tools) = &request.tools {
        body["tools"] = json!(tools.iter().map(|t| {
            json!({ "name": t.name, "description": t.description, "input_schema": t.input_schema })
        }).collect::<Vec<_>>());
    }
    body
}

fn translate_to_openai(request: &MessageRequest) -> serde_json::Value {
    use serde_json::json;
    let mut messages = vec![];
    if let Some(system) = &request.system { messages.push(json!({ "role": "system", "content": system })); }

    for msg in &request.messages {
        let mut content_text = String::new();
        let mut tool_calls = vec![];

        for block in &msg.content {
            match block {
                InputContentBlock::Text { text } => content_text.push_str(text),
                InputContentBlock::ToolUse { id, name, input } => {
                    tool_calls.push(json!({
                        "id": id, "type": "function", "function": { "name": name, "arguments": input.to_string() }
                    }));
                }
                InputContentBlock::ToolResult { tool_use_id, content, .. } => {
                    let text = content.iter().filter_map(|c| {
                        if let ToolResultContentBlock::Text { text } = c { Some(text.clone()) } else { None }
                    }).collect::<Vec<String>>().join("\n");
                    messages.push(json!({ "role": "tool", "tool_call_id": tool_use_id, "content": text }));
                }
            }
        }

        if !content_text.is_empty() || !tool_calls.is_empty() {
            let mut m = json!({ "role": msg.role });
            if !content_text.is_empty() { m["content"] = json!(content_text); }
            if !tool_calls.is_empty() { m["tool_calls"] = json!(tool_calls); }
            messages.push(m);
        }
    }

    let mut body = json!({ "model": request.model, "messages": messages, "stream": request.stream });
    if let Some(tools) = &request.tools {
        body["tools"] = json!(tools.iter().map(|t| {
            json!({ "type": "function", "function": { "name": t.name, "description": t.description, "parameters": t.input_schema } })
        }).collect::<Vec<_>>());
    }
    body
}

fn translate_to_gemini(request: &MessageRequest) -> serde_json::Value {
    use serde_json::json;
    let contents: Vec<serde_json::Value> = request.messages.iter().map(|msg| {
        let role = if msg.role == "assistant" { "model" } else { "user" };
        let parts: Vec<serde_json::Value> = msg.content.iter().map(|block| {
            match block {
                InputContentBlock::Text { text } => json!({ "text": text }),
                InputContentBlock::ToolUse { name, input, .. } => json!({ "functionCall": { "name": name, "args": input } }),
                InputContentBlock::ToolResult { tool_use_id, content, .. } => {
                    let text = content.iter().filter_map(|c| {
                        if let ToolResultContentBlock::Text { text } = c { Some(text.clone()) } else { None }
                    }).collect::<Vec<String>>().join("\n");
                    json!({ "functionResponse": { "name": tool_use_id, "response": { "result": text } } })
                }
            }
        }).collect();
        json!({ "role": role, "parts": parts })
    }).collect();

    let mut body = json!({ "contents": contents });
    if let Some(system) = &request.system { body["systemInstruction"] = json!({ "parts": [{ "text": system }] }); }
    if let Some(tools) = &request.tools {
        let declarations: Vec<serde_json::Value> = tools.iter().map(|t| {
            json!({ "name": t.name, "description": t.description, "parameters": t.input_schema })
        }).collect();
        body["tools"] = json!([{ "functionDeclarations": declarations }]);
    }
    body
}

fn translate_from_anthropic(response: serde_json::Value, model: &str) -> MessageResponse {
    let mut content = vec![];
    if let Some(blocks) = response.get("content").and_then(|c| c.as_array()) {
        for block in blocks {
            match block.get("type").and_then(|t| t.as_str()) {
                Some("text") => if let Some(text) = block.get("text").and_then(|t| t.as_str()) {
                    content.push(OutputContentBlock::Text { text: text.to_string() });
                },
                Some("tool_use") => if let (Some(id), Some(name), Some(input)) = (
                    block.get("id").and_then(|i| i.as_str()),
                    block.get("name").and_then(|n| n.as_str()),
                    block.get("input")
                ) {
                    content.push(OutputContentBlock::ToolUse { id: id.to_string(), name: name.to_string(), input: input.clone() });
                },
                _ => {}
            }
        }
    }
    let mut usage = Usage { input_tokens: 0, cache_creation_input_tokens: 0, cache_read_input_tokens: 0, output_tokens: 0 };
    if let Some(u) = response.get("usage") {
        usage.input_tokens = u.get("input_tokens").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
        usage.output_tokens = u.get("output_tokens").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
    }
    MessageResponse {
        id: response.get("id").and_then(|i| i.as_str()).unwrap_or("anthropic-response").to_string(),
        kind: "message".to_string(), role: "assistant".to_string(), content, model: model.to_string(),
        stop_reason: response.get("stop_reason").and_then(|s| s.as_str()).map(|s| s.to_string()),
        stop_sequence: None, usage, request_id: None,
    }
}

fn translate_from_openai(response: serde_json::Value, model: &str) -> MessageResponse {
    let mut content = vec![];
    if let Some(choices) = response.get("choices").and_then(|c| c.as_array()) {
        if let Some(choice) = choices.first() {
            if let Some(message) = choice.get("message") {
                if let Some(text) = message.get("content").and_then(|c| c.as_str()) {
                    content.push(OutputContentBlock::Text { text: text.to_string() });
                }
                if let Some(tool_calls) = message.get("tool_calls").and_then(|t| t.as_array()) {
                    for call in tool_calls {
                        if let (Some(id), Some(name), Some(args_str)) = (
                            call.get("id").and_then(|i| i.as_str()),
                            call.get("function").and_then(|f| f.get("name")).and_then(|n| n.as_str()),
                            call.get("function").and_then(|f| f.get("arguments")).and_then(|a| a.as_str())
                        ) {
                            if let Ok(args) = serde_json::from_str(args_str) {
                                content.push(OutputContentBlock::ToolUse { id: id.to_string(), name: name.to_string(), input: args });
                            }
                        }
                    }
                }
            }
        }
    }
    let mut usage = Usage { input_tokens: 0, cache_creation_input_tokens: 0, cache_read_input_tokens: 0, output_tokens: 0 };
    if let Some(u) = response.get("usage") {
        usage.input_tokens = u.get("prompt_tokens").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
        usage.output_tokens = u.get("completion_tokens").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
    }
    MessageResponse {
        id: response.get("id").and_then(|i| i.as_str()).unwrap_or("openai-response").to_string(),
        kind: "message".to_string(), role: "assistant".to_string(), content, model: model.to_string(),
        stop_reason: Some("end_turn".to_string()), stop_sequence: None, usage, request_id: None,
    }
}

fn translate_from_gemini(response: serde_json::Value, model: &str) -> MessageResponse {
    let mut content = vec![];
    if let Some(candidates) = response.get("candidates").and_then(|c| c.as_array()) {
        if let Some(candidate) = candidates.first() {
            if let Some(parts) = candidate.get("content").and_then(|c| c.get("parts")).and_then(|p| p.as_array()) {
                for part in parts {
                    if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                        content.push(OutputContentBlock::Text { text: text.to_string() });
                    }
                    if let Some(call) = part.get("functionCall") {
                        if let (Some(name), Some(args)) = (call.get("name").and_then(|n| n.as_str()), call.get("args")) {
                            content.push(OutputContentBlock::ToolUse { id: name.to_string(), name: name.to_string(), input: args.clone() });
                        }
                    }
                }
            }
        }
    }
    let mut usage = Usage { input_tokens: 0, cache_creation_input_tokens: 0, cache_read_input_tokens: 0, output_tokens: 0 };
    if let Some(u) = response.get("usageMetadata") {
        usage.input_tokens = u.get("promptTokenCount").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
        usage.output_tokens = u.get("candidatesTokenCount").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
    }
    MessageResponse {
        id: "gemini-response".to_string(), kind: "message".to_string(), role: "assistant".to_string(),
        content, model: model.to_string(), stop_reason: Some("end_turn".to_string()),
        stop_sequence: None, usage, request_id: None,
    }
}

pub fn read_env_non_empty(key: &str) -> Result<Option<String>, ApiError> {
    match std::env::var(key) {
        Ok(value) if !value.is_empty() => Ok(Some(value)),
        Ok(_) | Err(std::env::VarError::NotPresent) => Ok(None),
        Err(error) => Err(ApiError::from(error)),
    }
}

pub fn read_base_url() -> String {
    std::env::var("TERNLANG_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
}

fn request_id_from_headers(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get(REQUEST_ID_HEADER)
        .or_else(|| headers.get(ALT_REQUEST_ID_HEADER))
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

async fn expect_success(response: reqwest::Response) -> Result<reqwest::Response, ApiError> {
    if response.status().is_success() {
        Ok(response)
    } else {
        Err(ApiError::Auth(format!("HTTP {}", response.status())))
    }
}

pub fn resolve_startup_auth_source() -> Result<AuthSource, ApiError> {
    if let Some(api_key) = read_env_non_empty("TERNLANG_API_KEY")? {
        return Ok(AuthSource::ApiKey(api_key));
    }
    Ok(AuthSource::None)
}

/// Read the standard env var for `provider` and return the appropriate auth.
pub fn resolve_auth_for_provider(provider: LlmProvider) -> Result<AuthSource, ApiError> {
    let key = match provider {
        LlmProvider::Anthropic => read_env_non_empty("ANTHROPIC_API_KEY")?,
        LlmProvider::Google => {
            let k = read_env_non_empty("GEMINI_API_KEY")?;
            if k.is_some() { k } else { read_env_non_empty("GOOGLE_API_KEY")? }
        }
        LlmProvider::OpenAi => read_env_non_empty("OPENAI_API_KEY")?,
        LlmProvider::Xai => read_env_non_empty("XAI_API_KEY")?,
        LlmProvider::HuggingFace => read_env_non_empty("HUGGINGFACE_API_KEY")?,
        LlmProvider::Ollama => return Ok(AuthSource::None),
        _ => read_env_non_empty("TERNLANG_API_KEY")?,
    };
    Ok(key.map_or(AuthSource::None, AuthSource::ApiKey))
}

/// Scan well-known env vars and return the first available (provider, default-model) pair.
/// Returns None if no recognised key is set (Ollama local is not detected here).
pub fn detect_provider_and_model_from_env() -> Option<(LlmProvider, &'static str)> {
    let env_set = |var: &str| std::env::var(var).ok().filter(|v| !v.is_empty()).is_some();
    if env_set("ANTHROPIC_API_KEY") {
        return Some((LlmProvider::Anthropic, "claude-sonnet-4-5"));
    }
    if env_set("GEMINI_API_KEY") || env_set("GOOGLE_API_KEY") {
        return Some((LlmProvider::Google, "gemini-2.0-flash"));
    }
    if env_set("OPENAI_API_KEY") {
        return Some((LlmProvider::OpenAi, "gpt-4o-mini"));
    }
    if env_set("XAI_API_KEY") {
        return Some((LlmProvider::Xai, "grok-2-1212"));
    }
    if env_set("HUGGINGFACE_API_KEY") {
        return Some((LlmProvider::HuggingFace, "meta-llama/Meta-Llama-3-8B-Instruct"));
    }
    None
}

#[derive(serde::Deserialize)]
pub struct OAuthConfig {}
