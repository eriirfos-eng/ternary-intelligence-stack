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
    // ── First-party ──────────────────────────────────────────────────────────
    Ternlang,
    Anthropic,
    OpenAi,
    Google,
    Xai,
    // ── Inference cloud (all OpenAI-compatible) ───────────────────────────────
    Groq,
    Mistral,
    DeepSeek,
    Together,
    Fireworks,
    DeepInfra,
    OpenRouter,
    Perplexity,
    Cohere,
    Cerebras,
    Novita,
    SambaNova,
    NvidiaNim,
    // ── Regional foundation models (OpenAI-compatible) ───────────────────────
    Zhipu,
    MiniMax,
    Qwen,
    Moonshot,
    Qianfan,
    // ── Inference marketplaces ────────────────────────────────────────────────
    Chutes,
    // ── Enterprise cloud ─────────────────────────────────────────────────────
    Azure,
    Aws,
    // ── Aggregators ──────────────────────────────────────────────────────────
    HuggingFace,
    GitHub,
    // ── Local / offline ──────────────────────────────────────────────────────
    Ollama,
    LmStudio,
    // ── Generic OpenAI-compatible (user-configured base URL) ─────────────────
    OpenAiCompat,
}

impl LlmProvider {
    /// Returns `true` for every provider that speaks the OpenAI /v1/chat/completions wire format.
    pub fn is_openai_compat(self) -> bool {
        !matches!(self, Self::Anthropic | Self::Google | Self::Ternlang | Self::Aws)
    }

    pub fn default_base_url(&self) -> &'static str {
        match self {
            Self::Ternlang     => "https://api.ternlang.com",
            Self::Anthropic    => "https://api.anthropic.com",
            Self::OpenAi       => "https://api.openai.com",
            Self::Google       => "https://generativelanguage.googleapis.com",
            Self::Xai          => "https://api.x.ai",
            Self::Groq         => "https://api.groq.com/openai",
            Self::Mistral      => "https://api.mistral.ai",
            Self::DeepSeek     => "https://api.deepseek.com",
            Self::Together     => "https://api.together.xyz",
            Self::Fireworks    => "https://api.fireworks.ai/inference",
            Self::DeepInfra    => "https://api.deepinfra.com/v1/openai",
            Self::OpenRouter   => "https://openrouter.ai/api",
            Self::Perplexity   => "https://api.perplexity.ai",
            Self::Cohere       => "https://api.cohere.ai",
            Self::Cerebras     => "https://api.cerebras.ai",
            Self::Novita       => "https://api.novita.ai/v3/openai",
            Self::SambaNova    => "https://api.sambanova.ai",
            Self::NvidiaNim    => "https://integrate.api.nvidia.com",
            Self::Zhipu        => "https://open.bigmodel.cn/api/paas/v4",
            Self::MiniMax      => "https://api.minimax.chat/v1",
            Self::Qwen         => "https://dashscope.aliyuncs.com/compatible-mode/v1",
            Self::Moonshot     => "https://api.moonshot.cn",
            Self::Qianfan      => "https://aistudio.baidu.com/llm/lmapi/v3",
            Self::Chutes       => "https://llm.chutes.ai",
            Self::Azure        => "https://api.azure.com",
            Self::Aws          => "https://bedrock-runtime.us-east-1.amazonaws.com",
            Self::HuggingFace  => "https://api-inference.huggingface.co",
            Self::GitHub       => "https://models.inference.ai.azure.com",
            Self::Ollama       => "http://localhost:11434",
            Self::LmStudio     => "http://localhost:1234",
            Self::OpenAiCompat => "http://localhost:11434",
        }
    }

    pub fn api_path(&self) -> &'static str {
        match self {
            Self::Ternlang | Self::Anthropic => "/v1/messages",
            Self::Google => "/v1beta",
            Self::HuggingFace => "/models",
            // All OpenAI-compat providers share this path
            _ => "/v1/chat/completions",
        }
    }

    /// Canonical env-var name for this provider's API key (used for display / docs).
    pub fn env_var(self) -> &'static str {
        match self {
            Self::Ternlang     => "TERNLANG_API_KEY",
            Self::Anthropic    => "ANTHROPIC_API_KEY",
            Self::OpenAi       => "OPENAI_API_KEY",
            Self::Google       => "GEMINI_API_KEY",
            Self::Xai          => "XAI_API_KEY",
            Self::Groq         => "GROQ_API_KEY",
            Self::Mistral      => "MISTRAL_API_KEY",
            Self::DeepSeek     => "DEEPSEEK_API_KEY",
            Self::Together     => "TOGETHER_API_KEY",
            Self::Fireworks    => "FIREWORKS_API_KEY",
            Self::DeepInfra    => "DEEPINFRA_API_KEY",
            Self::OpenRouter   => "OPENROUTER_API_KEY",
            Self::Perplexity   => "PERPLEXITY_API_KEY",
            Self::Cohere       => "COHERE_API_KEY",
            Self::Cerebras     => "CEREBRAS_API_KEY",
            Self::Novita       => "NOVITA_API_KEY",
            Self::SambaNova    => "SAMBANOVA_API_KEY",
            Self::NvidiaNim    => "NVIDIA_API_KEY",
            Self::Zhipu        => "ZHIPU_API_KEY",
            Self::MiniMax      => "MINIMAX_API_KEY",
            Self::Qwen         => "DASHSCOPE_API_KEY",
            Self::Moonshot     => "MOONSHOT_API_KEY",
            Self::Qianfan      => "QIANFAN_API_KEY",
            Self::Chutes       => "CHUTES_API_KEY",
            Self::Azure        => "AZURE_OPENAI_API_KEY",
            Self::Aws          => "AWS_ACCESS_KEY_ID",
            Self::HuggingFace  => "HUGGINGFACE_API_KEY",
            Self::GitHub       => "GITHUB_TOKEN",
            Self::Ollama       => "",
            Self::LmStudio     => "",
            Self::OpenAiCompat => "OPENAI_API_KEY",
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
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .connect_timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap_or_default();
        Self {
            provider: LlmProvider::Ternlang,
            base_url: DEFAULT_BASE_URL.to_string(),
            auth,
            http,
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
                // Key passed as header (not URL param) to keep it out of server logs and Referer headers
                request_url = format!("{}/v1beta/{}:generateContent", self.base_url.trim_end_matches('/'), model_id);
                translate_to_gemini(request)
            }
            LlmProvider::Anthropic => translate_to_anthropic(request),
            LlmProvider::Ternlang | LlmProvider::Aws => {
                serde_json::to_value(request).map_err(ApiError::from)?
            }
            _ if self.provider.is_openai_compat() => translate_to_openai(request),
            _ => serde_json::to_value(request).map_err(ApiError::from)?,
        };

        let mut request_builder = self
            .http
            .post(&request_url)
            .header("content-type", "application/json");

        if self.provider == LlmProvider::Anthropic {
            request_builder = request_builder.header("anthropic-version", "2023-06-01");
        }
        if self.provider == LlmProvider::Google {
            if let Some(key) = self.auth.api_key() {
                request_builder = request_builder.header("x-goog-api-key", key);
            }
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
            LlmProvider::Ternlang | LlmProvider::Aws => {
                serde_json::from_value::<MessageResponse>(response_json).map_err(ApiError::from)?
            }
            _ if self.provider.is_openai_compat() => translate_from_openai(response_json, &request.model),
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
        // Google and OpenAI-compat providers use different SSE formats from Anthropic's.
        // Buffer the full response and wrap it in synthetic stream events so the
        // typewriter effect works identically without a format-specific SSE parser.
        if self.provider == LlmProvider::Google || self.provider.is_openai_compat() {
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
            parser: SseParser::new(false),
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
                Ok(response) => match expect_success(response, self.provider).await {
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
                let base = self.base_url.trim_end_matches('/');
                let url = format!("{}/v1beta/models", base);
                let mut req = self.http.get(&url);
                if let Some(key) = self.auth.api_key() {
                    req = req.header("x-goog-api-key", key);
                }
                let res = req.send().await.map_err(ApiError::from)?;
                let json: serde_json::Value = res.json().await.map_err(ApiError::from)?;
                let mut models = vec![];
                if let Some(list) = json.get("models").and_then(|m| m.as_array()) {
                    for m in list {
                        if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                            let id = name.replace("models/", "");
                            if !models.contains(&id) { models.push(id); }
                        }
                    }
                }
                if models.is_empty() {
                    return Ok(curated_models(self.provider));
                }
                Ok(models)
            }
            LlmProvider::Anthropic => Ok(curated_models(self.provider)),
            LlmProvider::Ternlang => {
                // Try the live /v1/models endpoint with auth; fall back to curated list.
                let url = format!("{}/v1/models", self.base_url.trim_end_matches('/'));
                if let Ok(res) = self.auth.apply(self.provider, self.http.get(&url)).send().await {
                    if res.status().is_success() {
                        if let Ok(json) = res.json::<serde_json::Value>().await {
                            let mut models = vec![];
                            if let Some(list) = json.get("data").and_then(|m| m.as_array()) {
                                for m in list {
                                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                                        if !models.contains(&id.to_string()) {
                                            models.push(id.to_string());
                                        }
                                    }
                                }
                            }
                            if !models.is_empty() {
                                return Ok(models);
                            }
                        }
                    }
                }
                Ok(curated_models(self.provider))
            }
            LlmProvider::Ollama => {
                let base = self.base_url.trim_end_matches('/');
                // Try OpenAI-compat /v1/models first (Ollama 0.1.14+ supports it).
                if let Ok(res) = self.http.get(&format!("{}/v1/models", base)).send().await {
                    if res.status().is_success() {
                        if let Ok(json) = res.json::<serde_json::Value>().await {
                            let mut models = vec![];
                            if let Some(list) = json.get("data").and_then(|m| m.as_array()) {
                                for m in list {
                                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                                        if !models.contains(&id.to_string()) {
                                            models.push(id.to_string());
                                        }
                                    }
                                }
                            }
                            if !models.is_empty() {
                                return Ok(models);
                            }
                        }
                    }
                }
                // Fall back to Ollama native /api/tags — returns actually installed models.
                let res = self
                    .http
                    .get(&format!("{}/api/tags", base))
                    .send()
                    .await
                    .map_err(|e| ApiError::Config(format!(
                        "Ollama is not reachable at {} — is it running? ({})",
                        base, e
                    )))?;
                if !res.status().is_success() {
                    return Err(ApiError::Config(format!(
                        "Ollama returned HTTP {} from /api/tags — check that it is running",
                        res.status()
                    )));
                }
                let json: serde_json::Value = res.json().await.map_err(ApiError::from)?;
                let mut models = vec![];
                if let Some(list) = json.get("models").and_then(|m| m.as_array()) {
                    for m in list {
                        if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                            if !models.contains(&name.to_string()) {
                                models.push(name.to_string());
                            }
                        }
                    }
                }
                if models.is_empty() {
                    Err(ApiError::Config(
                        "Ollama is running but has no models installed. Run: ollama pull <model-name>".to_string()
                    ))
                } else {
                    Ok(models)
                }
            }
            _ if self.provider.is_openai_compat() => {
                let url = format!("{}/v1/models", self.base_url.trim_end_matches('/'));
                let res = self.auth.apply(self.provider, self.http.get(&url)).send().await.map_err(ApiError::from)?;
                if !res.status().is_success() {
                    return Ok(curated_models(self.provider));
                }
                let json: serde_json::Value = res.json().await.map_err(ApiError::from)?;
                let mut models = vec![];
                if let Some(list) = json.get("data").and_then(|m| m.as_array()) {
                    for m in list {
                        if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                            if !models.contains(&id.to_string()) { models.push(id.to_string()); }
                        }
                    }
                }
                if models.is_empty() {
                    return Ok(curated_models(self.provider));
                }
                Ok(models)
            }
            _ => Ok(curated_models(self.provider)),
        }
    }

    /// Returns display-annotated model strings for the TUI picker.
    /// Format: "model-id (annotation)" for known models, bare id otherwise.
    pub async fn list_models_for_display(&self) -> Vec<String> {
        let raw = self.list_remote_models().await.unwrap_or_default();
        raw.into_iter().map(|id| {
            if let Some(ann) = model_annotation(&id) {
                format!("{id} ({ann})")
            } else {
                id
            }
        }).collect()
    }

    pub async fn create_embeddings(&self, model: &str, input: &[String]) -> Result<Vec<Vec<f32>>, ApiError> {
        if self.provider.is_openai_compat() || self.provider == LlmProvider::Ternlang {
            let url = format!("{}/v1/embeddings", self.base_url.trim_end_matches('/'));
            let req = EmbeddingRequest {
                model: model.to_string(),
                input: input.to_vec(),
            };
            
            let res = self.auth.apply(self.provider, self.http.post(&url))
                .json(&req)
                .send()
                .await
                .map_err(ApiError::from)?;
            
            if !res.status().is_success() {
                let status = res.status();
                let body = res.text().await.unwrap_or_default();
                return Err(ApiError::ProviderError { status, body });
            }
            
            let data: EmbeddingResponse = res.json().await.map_err(ApiError::from)?;
            Ok(data.data.into_iter().map(|d| d.embedding).collect())
        } else {
            Err(ApiError::Config(format!("Embeddings not yet supported for provider {:?}", self.provider)))
        }
    }

    pub async fn exchange_oauth_code(
        &self,
        _config: OAuthConfig,
        _request: &OAuthTokenExchangeRequest,
    ) -> Result<RuntimeTokenSet, ApiError> {
        Err(ApiError::Config("OAuth token exchange is not yet implemented".to_string()))
    }

    /// Check crates.io for the latest version of albert-cli.
    pub async fn check_for_updates(&self) -> Result<Option<String>, ApiError> {
        let url = "https://crates.io/api/v1/crates/albert-cli";
        // Crates.io requires a User-Agent header
        let res = self.http.get(url)
            .header("User-Agent", "albert-cli (https://github.com/eriirfos-eng/ternary-intelligence-stack)")
            .send()
            .await
            .map_err(ApiError::from)?;
        
        if !res.status().is_success() {
            return Ok(None);
        }

        let json: serde_json::Value = res.json().await.map_err(ApiError::from)?;
        let max_version = json.get("crate")
            .and_then(|c| c.get("max_version"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(max_version)
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
            parser: SseParser::new(false),
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
        let content: Vec<serde_json::Value> = msg.content.iter().filter_map(|block| {
            match block {
                InputContentBlock::Text { text } => Some(json!({ "type": "text", "text": text })),
                InputContentBlock::ToolUse { id, name, input } => Some(json!({
                    "type": "tool_use", "id": id, "name": name, "input": input
                })),
                InputContentBlock::ToolResult { tool_use_id, content, is_error } => {
                    let text = content.iter().filter_map(|c| {
                        if let ToolResultContentBlock::Text { text } = c { Some(text.clone()) } else { None }
                    }).collect::<Vec<String>>().join("\n");
                    Some(json!({
                        "type": "tool_result", "tool_use_id": tool_use_id, "content": text, "is_error": is_error
                    }))
                }
                InputContentBlock::Image { media_type, data } => Some(json!({
                    "type": "image",
                    "source": { "type": "base64", "media_type": media_type, "data": data }
                })),
                InputContentBlock::Thinking { .. } => None, // Gemini-specific, skip for Anthropic
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
        let mut content_blocks: Vec<serde_json::Value> = vec![];
        let mut tool_calls: Vec<serde_json::Value> = vec![];

        for block in &msg.content {
            match block {
                InputContentBlock::Text { text } => {
                    content_blocks.push(json!({ "type": "text", "text": text }));
                }
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
                InputContentBlock::Image { media_type, data } => {
                    content_blocks.push(json!({
                        "type": "image_url",
                        "image_url": { "url": format!("data:{media_type};base64,{data}") }
                    }));
                }
                InputContentBlock::Thinking { .. } => {} // Gemini-specific, skip for OpenAI
            }
        }

        if !content_blocks.is_empty() || !tool_calls.is_empty() {
            let mut m = json!({ "role": msg.role });
            if content_blocks.len() == 1 {
                if let Some(t) = content_blocks[0].get("text").and_then(|v| v.as_str()) {
                    m["content"] = json!(t);
                } else {
                    m["content"] = json!(content_blocks);
                }
            } else if !content_blocks.is_empty() {
                m["content"] = json!(content_blocks);
            }
            if !tool_calls.is_empty() { m["tool_calls"] = json!(tool_calls); }
            messages.push(m);
        }
    }

    let mut body = json!({ "model": request.model, "messages": messages, "stream": request.stream });
    if let Some(max) = request.max_tokens {
        if request.model.starts_with("o1") || request.model.starts_with("o3") {
            body["max_completion_tokens"] = json!(max);
        } else {
            body["max_tokens"] = json!(max);
        }
    }
    if let Some(effort) = &request.reasoning_effort {
        if effort == "off" {
            // NVIDIA specific extension mapping for disabling thinking
            body["extra_body"] = json!({
                "chat_template_kwargs": { "enable_thinking": false }
            });
        } else {
            // Standard OpenAI format (valid if supported, often ignored if not)
            body["reasoning_effort"] = json!(effort);
            // NVIDIA specific extension mapping for enabling thinking
            body["extra_body"] = json!({
                "chat_template_kwargs": { "enable_thinking": true },
                "reasoning_budget": 4096 // Lowered to improve responsiveness while maintaining deep thinking
            });
        }
    }
    if let Some(tools) = &request.tools {
        body["tools"] = json!(tools.iter().map(|t| {
            json!({ "type": "function", "function": { "name": t.name, "description": t.description, "parameters": t.input_schema } })
        }).collect::<Vec<_>>());
    }
    body
}

/// Gemini only supports a subset of JSON Schema — strip/normalize fields it rejects.
fn strip_gemini_unsupported_schema_fields(schema: serde_json::Value) -> serde_json::Value {
    match schema {
        serde_json::Value::Object(mut map) => {
            map.remove("additionalProperties");
            // "type": ["string", "null"] → "type": "string" (Gemini requires a single type string)
            if let Some(serde_json::Value::Array(types)) = map.get("type") {
                let first = types.iter()
                    .find(|t| t.as_str() != Some("null"))
                    .or_else(|| types.first())
                    .cloned()
                    .unwrap_or(serde_json::Value::String("string".to_string()));
                map.insert("type".to_string(), first);
            }
            let cleaned = map.into_iter()
                .map(|(k, v)| (k, strip_gemini_unsupported_schema_fields(v)))
                .collect();
            serde_json::Value::Object(cleaned)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(strip_gemini_unsupported_schema_fields).collect())
        }
        other => other,
    }
}

fn translate_to_gemini(request: &MessageRequest) -> serde_json::Value {
    use serde_json::json;
    let contents: Vec<serde_json::Value> = request.messages.iter().map(|msg| {
        let role = if msg.role == "assistant" { "model" } else { "user" };
        // Track the last thoughtSignature seen so it can be forwarded to the
        // following functionCall — Gemini requires it on the call part itself.
        let mut pending_sig: Option<String> = None;
        let mut parts: Vec<serde_json::Value> = Vec::new();
        for block in &msg.content {
            match block {
                InputContentBlock::Text { text } => parts.push(json!({ "text": text })),
                InputContentBlock::ToolUse { name, input, .. } => {
                    let mut call = json!({ "name": name, "args": input });
                    if let Some(sig) = pending_sig.take() {
                        call["thoughtSignature"] = json!(sig);
                    }
                    parts.push(json!({ "functionCall": call }));
                }
                InputContentBlock::ToolResult { tool_use_id, content, .. } => {
                    let text = content.iter().filter_map(|c| {
                        if let ToolResultContentBlock::Text { text } = c { Some(text.clone()) } else { None }
                    }).collect::<Vec<String>>().join("\n");
                    parts.push(json!({ "functionResponse": { "name": tool_use_id, "response": { "result": text } } }));
                }
                InputContentBlock::Image { media_type, data } => {
                    parts.push(json!({ "inline_data": { "mime_type": media_type, "data": data } }));
                }
                InputContentBlock::Thinking { text, thought_signature } => {
                    if let Some(sig) = thought_signature {
                        pending_sig = Some(sig.clone());
                    }
                    let mut part = json!({ "thought": true, "text": text });
                    if let Some(sig) = thought_signature {
                        part["thoughtSignature"] = json!(sig);
                    }
                    parts.push(part);
                }
            }
        }
        json!({ "role": role, "parts": parts })
    }).collect();

    let mut body = json!({ "contents": contents });
    if let Some(system) = &request.system { body["systemInstruction"] = json!({ "parts": [{ "text": system }] }); }
    if let Some(tools) = &request.tools {
        let declarations: Vec<serde_json::Value> = tools.iter().map(|t| {
            json!({ "name": t.name, "description": t.description, "parameters": strip_gemini_unsupported_schema_fields(t.input_schema.clone()) })
        }).collect();
        body["tools"] = json!([{ "functionDeclarations": declarations }]);
    }
    if let Some(max) = request.max_tokens {
        body["generationConfig"] = json!({ "maxOutputTokens": max });
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
                if let Some(reasoning) = message.get("reasoning_content").or_else(|| message.get("reasoning")).and_then(|c| c.as_str()) {
                    if !reasoning.is_empty() {
                        content.push(OutputContentBlock::Thinking { text: reasoning.to_string(), thought_signature: None });
                    }
                }
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
                    let is_thought = part.get("thought").and_then(|t| t.as_bool()).unwrap_or(false);
                    if is_thought {
                        let text = part.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string();
                        let sig = part.get("thoughtSignature").and_then(|s| s.as_str()).map(String::from);
                        content.push(OutputContentBlock::Thinking { text, thought_signature: sig });
                        continue;
                    }
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

/// Curated fallback model list for providers that don't expose a /v1/models endpoint.
pub fn curated_models(provider: LlmProvider) -> Vec<String> {
    let list: &[&str] = match provider {
        LlmProvider::Ternlang => &["albert-moe-13"],
        LlmProvider::Anthropic => &[
            "claude-opus-4-7",
            "claude-sonnet-4-6",
            "claude-haiku-4-5-20251001",
            "claude-3-7-sonnet-latest",
            "claude-3-5-sonnet-latest",
            "claude-3-5-haiku-latest",
            "claude-3-opus-latest",
        ],
        LlmProvider::Google => &[
            // gemini 3.x
            "gemini-3.5-flash",
            "gemini-3.1-pro-preview-customtools",
            "gemini-3.1-pro-preview",
            "gemini-3.1-flash-lite",
            "gemini-3.1-flash-lite-preview",
            "gemini-3.1-flash-image-preview",
            "gemini-3.1-flash-tts-preview",
            "gemini-3-pro-preview",
            "gemini-3-pro-image-preview",
            "gemini-3-flash-preview",
            // gemini 2.5
            "gemini-2.5-pro",
            "gemini-2.5-flash",
            "gemini-2.5-flash-lite",
            "gemini-2.5-flash-image",
            "gemini-2.5-flash-preview-tts",
            "gemini-2.5-pro-preview-tts",
            "gemini-2.5-computer-use-preview-10-2025",
            // gemini 2.0
            "gemini-2.0-flash",
            "gemini-2.0-flash-001",
            "gemini-2.0-flash-lite",
            "gemini-2.0-flash-lite-001",
            // gemma
            "gemma-4-31b-it",
            "gemma-4-26b-a4b-it",
            // audio / multimodal
            "lyria-3-pro-preview",
            "lyria-3-clip-preview",
            // research / agentic
            "deep-research-max-preview-04-2026",
            "deep-research-pro-preview-12-2025",
            "deep-research-preview-04-2026",
            // robotics / experimental
            "gemini-robotics-er-1.6-preview",
            "gemini-robotics-er-1.5-preview",
            "nano-banana-pro-preview",
            "antigravity-preview-05-2026",
            // dynamic latest aliases
            "gemini-pro-latest",
            "gemini-flash-latest",
            "gemini-flash-lite-latest",
        ],
        LlmProvider::OpenAi => &[
            "gpt-4o",
            "gpt-4o-mini",
            "gpt-4-turbo",
            "o3",
            "o3-mini",
            "o1",
            "o1-mini",
            "o4-mini",
        ],
        LlmProvider::Xai => &["grok-3", "grok-3-mini", "grok-2"],
        LlmProvider::DeepSeek => &["deepseek-chat", "deepseek-reasoner"],
        LlmProvider::Mistral => &[
            "mistral-large-latest",
            "mistral-small-latest",
            "codestral-latest",
            "pixtral-large-latest",
        ],
        LlmProvider::OpenRouter => &[
            // x-ai
            "x-ai/grok-build-0.1",
            "x-ai/grok-4.3",
            "x-ai/grok-4.20-multi-agent",
            "x-ai/grok-4.20",
            // google
            "google/gemini-3.5-flash",
            "google/gemini-3.1-flash-lite",
            "google/gemini-3.1-flash-lite-preview",
            "google/gemini-3.1-flash-image-preview",
            "google/gemini-3.1-pro-preview-customtools",
            "google/gemini-3.1-pro-preview",
            "google/gemini-3-flash-preview",
            "google/gemini-3-pro-image-preview",
            "google/gemini-2.5-flash-image",
            "google/gemini-2.5-flash-lite-preview-09-2025",
            "google/gemini-2.5-flash-lite",
            "google/gemini-2.5-flash",
            "google/gemini-2.5-pro",
            "google/gemini-2.5-pro-preview",
            "google/gemini-2.5-pro-preview-05-06",
            "google/gemini-2.0-flash-lite-001",
            "google/gemini-2.0-flash-001",
            "google/lyria-3-pro-preview",
            "google/lyria-3-clip-preview",
            "google/gemma-4-26b-a4b-it:free",
            "google/gemma-4-26b-a4b-it",
            "google/gemma-4-31b-it:free",
            "google/gemma-4-31b-it",
            "google/gemma-3n-e4b-it",
            "google/gemma-3-4b-it",
            "google/gemma-3-12b-it",
            "google/gemma-3-27b-it",
            "google/gemma-2-27b-it",
            // anthropic
            "anthropic/claude-opus-4.7-fast",
            "anthropic/claude-opus-4.7",
            "anthropic/claude-opus-4.6-fast",
            "anthropic/claude-opus-4.6",
            "anthropic/claude-opus-4.5",
            "anthropic/claude-opus-4.1",
            "anthropic/claude-opus-4",
            "anthropic/claude-sonnet-4.6",
            "anthropic/claude-sonnet-4.5",
            "anthropic/claude-sonnet-4",
            "anthropic/claude-haiku-4.5",
            "anthropic/claude-3.5-haiku",
            "anthropic/claude-3-haiku",
            // openai
            "openai/gpt-5.5-pro",
            "openai/gpt-5.5",
            "openai/gpt-5.4-image-2",
            "openai/gpt-5.4-nano",
            "openai/gpt-5.4-mini",
            "openai/gpt-5.4-pro",
            "openai/gpt-5.4",
            "openai/gpt-5.3-chat",
            "openai/gpt-5.3-codex",
            "openai/gpt-5.2-codex",
            "openai/gpt-5.2-chat",
            "openai/gpt-5.2-pro",
            "openai/gpt-5.2",
            "openai/gpt-5.1-codex-max",
            "openai/gpt-5.1-codex-mini",
            "openai/gpt-5.1-codex",
            "openai/gpt-5.1-chat",
            "openai/gpt-5.1",
            "openai/gpt-5-image-mini",
            "openai/gpt-5-image",
            "openai/gpt-5-pro",
            "openai/gpt-5-codex",
            "openai/gpt-5-chat",
            "openai/gpt-5-mini",
            "openai/gpt-5-nano",
            "openai/gpt-5",
            "openai/gpt-4o-audio-preview",
            "openai/gpt-4o-mini-search-preview",
            "openai/gpt-4o-search-preview",
            "openai/gpt-4o-2024-11-20",
            "openai/gpt-4o-2024-08-06",
            "openai/gpt-4o-mini-2024-07-18",
            "openai/gpt-4o-mini",
            "openai/gpt-4o-2024-05-13",
            "openai/gpt-4o",
            "openai/gpt-4.1",
            "openai/gpt-4.1-mini",
            "openai/gpt-4.1-nano",
            "openai/gpt-4-turbo",
            "openai/gpt-4-turbo-preview",
            "openai/gpt-4-1106-preview",
            "openai/gpt-4-0314",
            "openai/gpt-4",
            "openai/gpt-chat-latest",
            "openai/gpt-audio",
            "openai/gpt-audio-mini",
            "openai/gpt-3.5-turbo-0613",
            "openai/gpt-3.5-turbo-instruct",
            "openai/gpt-3.5-turbo-16k",
            "openai/gpt-3.5-turbo",
            "openai/o4-mini-high",
            "openai/o4-mini-deep-research",
            "openai/o4-mini",
            "openai/o3-pro",
            "openai/o3-deep-research",
            "openai/o3-mini-high",
            "openai/o3-mini",
            "openai/o3",
            "openai/o1-pro",
            "openai/o1",
            "openai/gpt-oss-safeguard-20b",
            "openai/gpt-oss-120b:free",
            "openai/gpt-oss-120b",
            "openai/gpt-oss-20b:free",
            "openai/gpt-oss-20b",
            // deepseek
            "deepseek/deepseek-v4-pro",
            "deepseek/deepseek-v4-flash:free",
            "deepseek/deepseek-v4-flash",
            "deepseek/deepseek-v3.2-speciale",
            "deepseek/deepseek-v3.2",
            "deepseek/deepseek-v3.2-exp",
            "deepseek/deepseek-v3.1-terminus",
            "deepseek/deepseek-chat-v3.1",
            "deepseek/deepseek-r1-0528",
            "deepseek/deepseek-r1-distill-qwen-32b",
            "deepseek/deepseek-r1-distill-llama-70b",
            "deepseek/deepseek-r1",
            "deepseek/deepseek-chat",
            // meta-llama
            "meta-llama/llama-4-maverick",
            "meta-llama/llama-4-scout",
            "meta-llama/llama-guard-4-12b",
            "meta-llama/llama-guard-3-8b",
            "meta-llama/llama-3.3-70b-instruct:free",
            "meta-llama/llama-3.3-70b-instruct",
            "meta-llama/llama-3.2-11b-vision-instruct",
            "meta-llama/llama-3.2-3b-instruct:free",
            "meta-llama/llama-3.2-3b-instruct",
            "meta-llama/llama-3.2-1b-instruct",
            "meta-llama/llama-3.1-70b-instruct",
            "meta-llama/llama-3.1-8b-instruct",
            "meta-llama/llama-3-70b-instruct",
            "meta-llama/llama-3-8b-instruct",
            // qwen
            "qwen/qwen3.6-flash",
            "qwen/qwen3.6-35b-a3b",
            "qwen/qwen3.6-max-preview",
            "qwen/qwen3.6-27b",
            "qwen/qwen3.6-plus",
            "qwen/qwen3.5-plus-20260420",
            "qwen/qwen3.5-plus-02-15",
            "qwen/qwen3.5-flash-02-23",
            "qwen/qwen3.5-35b-a3b",
            "qwen/qwen3.5-27b",
            "qwen/qwen3.5-122b-a10b",
            "qwen/qwen3.5-9b",
            "qwen/qwen3-vl-235b-a22b-thinking",
            "qwen/qwen3-vl-235b-a22b-instruct",
            "qwen/qwen3-vl-32b-instruct",
            "qwen/qwen3-vl-30b-a3b-thinking",
            "qwen/qwen3-vl-30b-a3b-instruct",
            "qwen/qwen3-vl-8b-thinking",
            "qwen/qwen3-vl-8b-instruct",
            "qwen/qwen3-max-thinking",
            "qwen/qwen3-max",
            "qwen/qwen3-coder-next",
            "qwen/qwen3-coder-plus",
            "qwen/qwen3-coder-flash",
            "qwen/qwen3-coder-30b-a3b-instruct",
            "qwen/qwen3-coder:free",
            "qwen/qwen3-coder",
            "qwen/qwen3-next-80b-a3b-thinking",
            "qwen/qwen3-next-80b-a3b-instruct:free",
            "qwen/qwen3-next-80b-a3b-instruct",
            "qwen/qwen3-235b-a22b-thinking-2507",
            "qwen/qwen3-235b-a22b-2507",
            "qwen/qwen3-235b-a22b",
            "qwen/qwen3-30b-a3b-thinking-2507",
            "qwen/qwen3-30b-a3b-instruct-2507",
            "qwen/qwen3-30b-a3b",
            "qwen/qwen3-32b",
            "qwen/qwen3-14b",
            "qwen/qwen3-8b",
            "qwen/qwen-plus-2025-07-28:thinking",
            "qwen/qwen-plus-2025-07-28",
            "qwen/qwen-plus",
            "qwen/qwen2.5-vl-72b-instruct",
            "qwen/qwen2.5-coder-32b-instruct",
            "qwen/qwen2.5-72b-instruct",
            // mistralai
            "mistralai/mistral-medium-3-5",
            "mistralai/mistral-medium-3.1",
            "mistralai/mistral-medium-3",
            "mistralai/mistral-large-2512",
            "mistralai/mistral-large-2411",
            "mistralai/mistral-large-2407",
            "mistralai/mistral-large",
            "mistralai/mistral-small-3.2-24b-instruct",
            "mistralai/mistral-small-3.1-24b-instruct",
            "mistralai/mistral-small-2603",
            "mistralai/mistral-small-24b-instruct-2501",
            "mistralai/mistral-saba",
            "mistralai/mistral-nemo",
            "mistralai/mistral-7b-instruct-v0.1",
            "mistralai/ministral-14b-2512",
            "mistralai/ministral-8b-2512",
            "mistralai/ministral-3b-2512",
            "mistralai/mixtral-8x22b-instruct",
            "mistralai/codestral-2508",
            "mistralai/devstral-2512",
            "mistralai/devstral-medium",
            "mistralai/devstral-small",
            "mistralai/pixtral-large-2411",
            "mistralai/voxtral-small-24b-2507",
            // z-ai / zhipu
            "z-ai/glm-5.1",
            "z-ai/glm-5v-turbo",
            "z-ai/glm-5-turbo",
            "z-ai/glm-5",
            "z-ai/glm-4.7-flash",
            "z-ai/glm-4.7",
            "z-ai/glm-4.6v",
            "z-ai/glm-4.6",
            "z-ai/glm-4.5v",
            "z-ai/glm-4.5-air:free",
            "z-ai/glm-4.5-air",
            "z-ai/glm-4.5",
            "z-ai/glm-4-32b",
            // microsoft
            "microsoft/phi-4-mini-instruct",
            "microsoft/wizardlm-2-8x22b",
            // amazon
            "amazon/nova-2-lite-v1",
            "amazon/nova-premier-v1",
            "amazon/nova-pro-v1",
            "amazon/nova-lite-v1",
            "amazon/nova-micro-v1",
            // nvidia
            "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free",
            "nvidia/nemotron-3-super-120b-a12b:free",
            "nvidia/nemotron-3-super-120b-a12b",
            "nvidia/nemotron-3-nano-30b-a3b:free",
            "nvidia/nemotron-3-nano-30b-a3b",
            "nvidia/nemotron-nano-12b-v2-vl:free",
            "nvidia/nemotron-nano-9b-v2:free",
            "nvidia/nemotron-nano-9b-v2",
            "nvidia/llama-3.3-nemotron-super-49b-v1.5",
            // perplexity
            "perplexity/sonar-reasoning-pro",
            "perplexity/sonar-pro-search",
            "perplexity/sonar-pro",
            "perplexity/sonar-deep-research",
            "perplexity/sonar",
            // cohere
            "cohere/command-a",
            "cohere/command-r-plus-08-2024",
            "cohere/command-r-08-2024",
            "cohere/command-r7b-12-2024",
            // minimax
            "minimax/minimax-m2.7",
            "minimax/minimax-m2.5:free",
            "minimax/minimax-m2.5",
            "minimax/minimax-m2.1",
            "minimax/minimax-m2-her",
            "minimax/minimax-m2",
            "minimax/minimax-m1",
            "minimax/minimax-01",
            // moonshotai
            "moonshotai/kimi-k2.6",
            "moonshotai/kimi-k2.5",
            "moonshotai/kimi-k2-0905",
            "moonshotai/kimi-k2-thinking",
            "moonshotai/kimi-k2",
            // baidu
            "baidu/ernie-4.5-vl-424b-a47b",
            "baidu/ernie-4.5-300b-a47b",
            "baidu/ernie-4.5-vl-28b-a3b",
            "baidu/ernie-4.5-21b-a3b-thinking",
            "baidu/ernie-4.5-21b-a3b",
            "baidu/qianfan-ocr-fast",
            "baidu/cobuddy:free",
            // bytedance-seed
            "bytedance-seed/seed-2.0-lite",
            "bytedance-seed/seed-2.0-mini",
            "bytedance-seed/seed-1.6-flash",
            "bytedance-seed/seed-1.6",
            // bytedance
            "bytedance/ui-tars-1.5-7b",
            // tencent
            "tencent/hunyuan-a13b-instruct",
            "tencent/hy3-preview",
            // xiaomi
            "xiaomi/mimo-v2.5-pro",
            "xiaomi/mimo-v2.5",
            "xiaomi/mimo-v2-omni",
            "xiaomi/mimo-v2-pro",
            "xiaomi/mimo-v2-flash",
            // nousresearch
            "nousresearch/hermes-4-405b",
            "nousresearch/hermes-4-70b",
            "nousresearch/hermes-3-llama-3.1-405b:free",
            "nousresearch/hermes-3-llama-3.1-405b",
            "nousresearch/hermes-3-llama-3.1-70b",
            "nousresearch/hermes-2-pro-llama-3-8b",
            // arcee-ai
            "arcee-ai/trinity-large-thinking:free",
            "arcee-ai/trinity-large-thinking",
            "arcee-ai/trinity-large-preview",
            "arcee-ai/trinity-mini",
            "arcee-ai/spotlight",
            "arcee-ai/maestro-reasoning",
            "arcee-ai/virtuoso-large",
            "arcee-ai/coder-large",
            // liquid
            "liquid/lfm-2-24b-a2b",
            "liquid/lfm-2.5-1.2b-thinking:free",
            "liquid/lfm-2.5-1.2b-instruct:free",
            // ai21
            "ai21/jamba-large-1.7",
            // aion-labs
            "aion-labs/aion-2.0",
            "aion-labs/aion-1.0",
            "aion-labs/aion-1.0-mini",
            "aion-labs/aion-rp-llama-3.1-8b",
            // inception
            "inception/mercury-2",
            // morph
            "morph/morph-v3-large",
            "morph/morph-v3-fast",
            // writer
            "writer/palmyra-x5",
            // upstage
            "upstage/solar-pro-3",
            // rekaai
            "rekaai/reka-flash-3",
            "rekaai/reka-edge",
            // relace
            "relace/relace-apply-3",
            "relace/relace-search",
            // nex-agi
            "nex-agi/deepseek-v3.1-nex-n1",
            // prime-intellect
            "prime-intellect/intellect-3",
            // deepcogito
            "deepcogito/cogito-v2.1-671b",
            // allenai
            "allenai/olmo-3-32b-think",
            // ibm-granite
            "ibm-granite/granite-4.1-8b",
            "ibm-granite/granite-4.0-h-micro",
            // kwaipilot
            "kwaipilot/kat-coder-pro-v2",
            // switchpoint
            "switchpoint/router",
            // sao10k
            "sao10k/l3.1-70b-hanami-x1",
            "sao10k/l3.3-euryale-70b",
            "sao10k/l3.1-euryale-70b",
            "sao10k/l3-lunaris-8b",
            "sao10k/l3-euryale-70b",
            // inflection
            "inflection/inflection-3-productivity",
            "inflection/inflection-3-pi",
            // thedrummer
            "thedrummer/cydonia-24b-v4.1",
            "thedrummer/skyfall-36b-v2",
            "thedrummer/unslopnemo-12b",
            "thedrummer/rocinante-12b",
            // anthracite-org
            "anthracite-org/magnum-v4-72b",
            // cognitivecomputations
            "cognitivecomputations/dolphin-mistral-24b-venice-edition:free",
            // essentialai
            "essentialai/rnj-1-instruct",
            // alfredpros
            "alfredpros/codellama-7b-instruct-solidity",
            // alibaba
            "alibaba/tongyi-deepresearch-30b-a3b",
            // stepfun
            "stepfun/step-3.5-flash",
            // openrouter-native
            "openrouter/owl-alpha",
            "openrouter/pareto-code",
            "openrouter/bodybuilder",
            "openrouter/free",
            "openrouter/auto",
            // poolside
            "poolside/laguna-xs.2:free",
            "poolside/laguna-m.1:free",
            // inclusionai
            "inclusionai/ring-2.6-1t",
            "inclusionai/ling-2.6-1t",
            "inclusionai/ling-2.6-flash",
            // perceptron
            "perceptron/perceptron-mk1",
            // mancer / legacy
            "mancer/weaver",
            "undi95/remm-slerp-l2-13b",
            "gryphe/mythomax-l2-13b",
            // openrouter alias routes (~ prefix = dynamic latest pointer)
            "~anthropic/claude-haiku-latest",
            "~anthropic/claude-sonnet-latest",
            "~anthropic/claude-opus-latest",
            "~openai/gpt-mini-latest",
            "~openai/gpt-latest",
            "~google/gemini-flash-latest",
            "~google/gemini-pro-latest",
            "~moonshotai/kimi-latest",
        ],
        LlmProvider::Groq => &[
            "llama-3.3-70b-versatile",
            "llama-3.1-8b-instant",
            "llama-3.1-70b-versatile",
            "gemma2-9b-it",
            "mixtral-8x7b-32768",
        ],
        LlmProvider::Cohere => &["command-r-plus", "command-r", "command-r-08-2024"],
        LlmProvider::Perplexity => &["sonar-pro", "sonar", "sonar-reasoning"],
        LlmProvider::Together => &[
            "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo",
            "meta-llama/Llama-3.2-90B-Vision-Instruct-Turbo",
            "Qwen/Qwen2.5-72B-Instruct-Turbo",
        ],
        LlmProvider::Fireworks => &[
            "accounts/fireworks/models/llama-v3p1-70b-instruct",
            "accounts/fireworks/models/deepseek-r1",
            "accounts/fireworks/models/qwen2p5-72b-instruct",
        ],
        LlmProvider::Cerebras => &["llama3.3-70b", "llama3.1-8b"],
        LlmProvider::SambaNova => &[
            "Meta-Llama-3.3-70B-Instruct",
            "DeepSeek-R1-Distill-Llama-70B",
        ],
        LlmProvider::NvidiaNim => &[
            "nvidia/llama-3.1-nemotron-70b-instruct",
            "nvidia/nemotron-4-340b-instruct",
            "meta/llama-3.1-405b-instruct",
            "meta/llama-3.3-70b-instruct",
            "meta/llama-3.1-70b-instruct",
            "meta/llama-3.1-8b-instruct",
            "mistralai/mistral-large-2-instruct",
            "mistralai/mixtral-8x22b-instruct-v0.1",
            "deepseek-ai/deepseek-r1",
            "google/gemma-2-27b-it",
            "microsoft/phi-3-medium-128k-instruct",
            "qwen/qwen2-72b-instruct",
        ],
        LlmProvider::Zhipu => &[
            "zai/glm-5",
            "zai/glm-4.7",
            "zai/glm-4.7-flash",
            "zai/glm-4.5",
            "zai/glm-4.5-flash",
            "glm-4-plus",
            "glm-4-air",
        ],
        LlmProvider::MiniMax => &["MiniMax-Text-01", "abab6.5s-chat", "abab6.5g-chat"],
        LlmProvider::Qwen => &[
            "qwen-max",
            "qwen-plus",
            "qwen-turbo",
            "qwen2.5-72b-instruct",
            "qwq-32b",
            "qwen2.5-coder-32b-instruct",
        ],
        LlmProvider::Moonshot => &["moonshot-v1-128k", "moonshot-v1-32k", "moonshot-v1-8k", "kimi-k2"],
        LlmProvider::Qianfan => &["ernie-4.5-turbo-128k", "ernie-4.0-turbo-8k", "ernie-3.5-8k"],
        LlmProvider::Chutes => &[
            "deepseek-ai/DeepSeek-R1",
            "deepseek-ai/DeepSeek-V3",
            "Qwen/Qwen2.5-72B-Instruct",
        ],
        LlmProvider::HuggingFace => &[
            "meta-llama/Meta-Llama-3-8B-Instruct",
            "mistralai/Mistral-7B-Instruct-v0.3",
        ],
        LlmProvider::GitHub => &["gpt-4o", "gpt-4o-mini", "o3-mini"],
        _ => &[],
    };
    list.iter().map(|s| (*s).to_string()).collect()
}

/// Human-readable annotation for known model IDs.
pub fn model_annotation(id: &str) -> Option<&'static str> {
    match id {
        // Anthropic
        "claude-opus-4-7" | "claude-opus-4-5" => Some("Opus 4 · ctx 200k · top capability"),
        "claude-sonnet-4-6" | "claude-3-7-sonnet-latest" | "claude-3-5-sonnet-latest" => Some("Sonnet · ctx 200k · balanced"),
        "claude-haiku-4-5-20251001" | "claude-3-5-haiku-latest" => Some("Haiku · ctx 200k · fast & cheap"),
        "claude-3-opus-latest" => Some("Opus 3 · ctx 200k · legacy"),
        // Google
        "gemini-2.5-pro" => Some("ctx 1M · best reasoning · multimodal"),
        "gemini-2.5-flash" => Some("ctx 1M · fast · recommended"),
        "gemini-2.0-flash" => Some("ctx 1M · ultra-fast"),
        "gemini-1.5-pro" => Some("ctx 2M · legacy pro"),
        // OpenAI
        "gpt-4o" => Some("ctx 128k · multimodal · flagship"),
        "gpt-4o-mini" => Some("ctx 128k · cheap · fast"),
        "o3" => Some("ctx 200k · reasoning · top"),
        "o3-mini" | "o4-mini" => Some("ctx 200k · reasoning · fast"),
        "o1" => Some("ctx 200k · reasoning · legacy"),
        // xAI
        "grok-3" => Some("ctx 131k · flagship"),
        "grok-3-mini" => Some("ctx 131k · fast"),
        // DeepSeek
        "deepseek-chat" => Some("ctx 64k · DeepSeek-V3 · cost-efficient"),
        "deepseek-reasoner" => Some("ctx 64k · R1 · chain-of-thought"),
        // Mistral
        "mistral-large-latest" => Some("ctx 128k · flagship"),
        "codestral-latest" => Some("ctx 256k · code specialist"),
        // Zhipu / Z.AI
        "zai/glm-5" => Some("ctx 200k · reasoning · alias: GLM"),
        "zai/glm-4.7" => Some("ctx 128k · balanced"),
        "zai/glm-4.7-flash" => Some("ctx 128k · fast"),
        "zai/glm-4.5" => Some("ctx 128k · cost-efficient"),
        "zai/glm-4.5-flash" => Some("ctx 32k · ultra-fast"),
        // Qwen
        "qwen-max" => Some("ctx 32k · flagship"),
        "qwq-32b" => Some("ctx 32k · reasoning"),
        "qwen2.5-coder-32b-instruct" => Some("ctx 128k · code specialist"),
        // Moonshot / Kimi
        "kimi-k2" => Some("ctx 128k · Kimi K2.5"),
        "moonshot-v1-128k" => Some("ctx 128k · long context"),
        // Groq
        "llama-3.3-70b-versatile" => Some("ctx 128k · ultra-fast on Groq"),
        // NVIDIA NIM  (free tier = standard signup key; enterprise = NGC subscription required)
        "nvidia/llama-3.1-nemotron-70b-instruct" => Some("ctx 128k · free tier · NVIDIA flagship"),
        "nvidia/nemotron-4-340b-instruct"         => Some("ctx 4k · enterprise · 340B research"),
        "meta/llama-3.1-405b-instruct"            => Some("ctx 128k · enterprise · 405B largest"),
        "meta/llama-3.3-70b-instruct"             => Some("ctx 128k · free tier · latest Llama 3.3"),
        "meta/llama-3.1-70b-instruct"             => Some("ctx 128k · free tier · solid baseline"),
        "meta/llama-3.1-8b-instruct"              => Some("ctx 128k · free tier · fast · lightweight"),
        "mistralai/mistral-large-2-instruct"      => Some("ctx 128k · check tier · Mistral flagship"),
        "mistralai/mixtral-8x22b-instruct-v0.1"  => Some("ctx 64k · check tier · MoE reasoning"),
        "deepseek-ai/deepseek-r1"                 => Some("ctx 64k · check tier · chain-of-thought"),
        "google/gemma-2-27b-it"                   => Some("ctx 8k · free tier · Google open model"),
        "microsoft/phi-3-medium-128k-instruct"    => Some("ctx 128k · free tier · small but capable"),
        "qwen/qwen2-72b-instruct"                 => Some("ctx 32k · free tier · multilingual"),
        // Misc
        _ => None,
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

async fn expect_success(response: reqwest::Response, provider: LlmProvider) -> Result<reqwest::Response, ApiError> {
    if response.status().is_success() {
        return Ok(response);
    }
    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    // 401/403 are always key/auth problems.
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        return Err(ApiError::Auth(format!("HTTP {status} — check your {} API key", provider.env_var())));
    }

    // 404 on an OpenAI-compat provider almost always means the model isn't in the caller's plan/tier.
    if status == reqwest::StatusCode::NOT_FOUND && provider.is_openai_compat() {
        return Err(ApiError::Auth(format!(
            "model not accessible on your {} plan (404) — this model may require a paid tier. \
             Choose a different model or check integrate.api.nvidia.com (or the provider's docs) for your tier's supported models.",
            provider.env_var().trim_end_matches("_API_KEY")
        )));
    }

    Err(ApiError::Auth(format!("HTTP {status}: {}", body.chars().take(300).collect::<String>())))
}

pub fn resolve_startup_auth_source() -> Result<AuthSource, ApiError> {
    if let Some(api_key) = read_env_non_empty("TERNLANG_API_KEY")? {
        return Ok(AuthSource::ApiKey(api_key));
    }
    Ok(AuthSource::None)
}

/// Read the standard env var for `provider` and return the appropriate auth.
pub fn resolve_auth_for_provider(provider: LlmProvider) -> Result<AuthSource, ApiError> {
    // No-auth local providers
    if matches!(provider, LlmProvider::Ollama | LlmProvider::LmStudio | LlmProvider::OpenAiCompat) {
        return Ok(AuthSource::None);
    }
    let env_var = provider.env_var();
    let key = if provider == LlmProvider::Google {
        // Google accepts either GEMINI_API_KEY or GOOGLE_API_KEY
        read_env_non_empty("GEMINI_API_KEY").ok().flatten()
            .or_else(|| read_env_non_empty("GOOGLE_API_KEY").ok().flatten())
    } else if env_var.is_empty() {
        None
    } else {
        read_env_non_empty(env_var)?
    };
    Ok(key.map_or(AuthSource::None, AuthSource::ApiKey))
}

/// Scan well-known env vars and return the first available (provider, default-model) pair.
/// Returns None if no recognised key is set (Ollama/LM Studio local are not detected here).
pub fn detect_provider_and_model_from_env() -> Option<(LlmProvider, &'static str)> {
    let env_set = |var: &str| std::env::var(var).ok().filter(|v| !v.is_empty()).is_some();
    if env_set("ANTHROPIC_API_KEY") {
        return Some((LlmProvider::Anthropic, "claude-sonnet-4-6"));
    }
    if env_set("GEMINI_API_KEY") || env_set("GOOGLE_API_KEY") {
        return Some((LlmProvider::Google, "gemini-2.5-flash"));
    }
    if env_set("OPENAI_API_KEY") {
        return Some((LlmProvider::OpenAi, "gpt-4o-mini"));
    }
    if env_set("XAI_API_KEY") {
        return Some((LlmProvider::Xai, "grok-3-mini"));
    }
    if env_set("GROQ_API_KEY") {
        return Some((LlmProvider::Groq, "llama-3.3-70b-versatile"));
    }
    if env_set("MISTRAL_API_KEY") {
        return Some((LlmProvider::Mistral, "mistral-large-latest"));
    }
    if env_set("DEEPSEEK_API_KEY") {
        return Some((LlmProvider::DeepSeek, "deepseek-chat"));
    }
    if env_set("TOGETHER_API_KEY") {
        return Some((LlmProvider::Together, "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo"));
    }
    if env_set("OPENROUTER_API_KEY") {
        return Some((LlmProvider::OpenRouter, "openai/gpt-4o-mini"));
    }
    if env_set("PERPLEXITY_API_KEY") {
        return Some((LlmProvider::Perplexity, "sonar-pro"));
    }
    if env_set("FIREWORKS_API_KEY") {
        return Some((LlmProvider::Fireworks, "accounts/fireworks/models/llama-v3p1-70b-instruct"));
    }
    if env_set("COHERE_API_KEY") {
        return Some((LlmProvider::Cohere, "command-r-plus"));
    }
    if env_set("CEREBRAS_API_KEY") {
        return Some((LlmProvider::Cerebras, "llama3.3-70b"));
    }
    if env_set("NOVITA_API_KEY") {
        return Some((LlmProvider::Novita, "meta-llama/llama-3.1-70b-instruct"));
    }
    if env_set("SAMBANOVA_API_KEY") {
        return Some((LlmProvider::SambaNova, "Meta-Llama-3.3-70B-Instruct"));
    }
    if env_set("NVIDIA_API_KEY") {
        return Some((LlmProvider::NvidiaNim, "nvidia/llama-3.1-nemotron-70b-instruct"));
    }
    if env_set("HUGGINGFACE_API_KEY") {
        return Some((LlmProvider::HuggingFace, "meta-llama/Meta-Llama-3-8B-Instruct"));
    }
    if env_set("GITHUB_TOKEN") {
        return Some((LlmProvider::GitHub, "gpt-4o-mini"));
    }
    None
}

#[derive(serde::Deserialize)]
pub struct OAuthConfig {}

pub fn translate_openai_chunk_to_event(chunk: serde_json::Value) -> Option<StreamEvent> {
    use crate::types::*;
    // VERY simplified translation, as a full robust mapper takes lots of logic
    if let Some(choices) = chunk.get("choices").and_then(|c| c.as_array()) {
        if let Some(choice) = choices.first() {
            if let Some(delta) = choice.get("delta") {
                if let Some(text) = delta.get("content").and_then(|c| c.as_str()) {
                    if !text.is_empty() {
                        return Some(StreamEvent::ContentBlockDelta(ContentBlockDeltaEvent {
                            index: 0,
                            delta: ContentBlockDelta::TextDelta { text: text.to_string() }
                        }));
                    }
                }
                if let Some(reasoning) = delta.get("reasoning_content").or_else(|| delta.get("reasoning")).and_then(|c| c.as_str()) {
                    if !reasoning.is_empty() {
                        return Some(StreamEvent::ContentBlockDelta(ContentBlockDeltaEvent {
                            index: 0, // In reality, we'd manage blocks but 0 is okay for MVP
                            delta: ContentBlockDelta::ReasoningDelta { text: reasoning.to_string() }
                        }));
                    }
                }
            }
        }
    }
    None
}
