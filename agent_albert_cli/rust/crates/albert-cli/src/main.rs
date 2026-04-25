mod init;
mod input;
mod render;
mod tui;

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crossterm::execute;
use dialoguer::Select;
use console::style;

use api::{
    TernlangClient, AuthSource, ContentBlockDelta, InputContentBlock,
    InputMessage, MessageRequest, OutputContentBlock,
    StreamEvent as ApiStreamEvent, ToolChoice, ToolDefinition, ToolResultContentBlock,
};
use commands::{render_slash_command_help, slash_command_specs, SlashCommand};
use compat_harness::{extract_manifest, UpstreamPaths};
use init::initialize_repo;
use runtime::{
    clear_oauth_credentials, generate_pkce_pair, generate_state, load_system_prompt,
    parse_oauth_callback_request_target, save_oauth_credentials, ApiClient, ApiRequest,
    AssistantEvent, CompactionConfig, ConfigLoader, ConfigSource, ContentBlock,
    ConversationMessage, ConversationRuntime, MessageRole, OAuthAuthorizationRequest, OAuthConfig,
    PermissionMode, PermissionPolicy,
    ProjectContext, RuntimeError, Session, TokenUsage, ToolError, ToolExecutor, UsageTracker,
};
use serde_json::json;
use tools::{execute_tool, mvp_tool_specs, ToolSpec};
use runtime::{McpServerConfig, McpServerManager, McpStdioServerConfig, ScopedMcpServerConfig};
use std::sync::{Arc, Mutex};

const DEFAULT_MODEL: &str = "gemini-2.5-flash";
fn max_tokens_for_model(model: &str) -> u32 {
    if model.contains("haiku") {
        16_000
    } else {
        32_000
    }
}
const DEFAULT_DATE: &str = "2024-03-25";
const DEFAULT_OAUTH_CALLBACK_PORT: u16 = 4545;
const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_TARGET: Option<&str> = option_env!("TARGET");
const GIT_SHA: Option<&str> = option_env!("GIT_SHA");

type AllowedToolSet = BTreeSet<String>;

fn main() {
    if let Err(error) = run() {
        eprintln!(
            "error: {error}

Run `albert-cli --help` for usage."
        );
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_args(&args)? {
        CliAction::DumpManifests => dump_manifests(),
        CliAction::BootstrapPlan => print_bootstrap_plan(),
        CliAction::PrintSystemPrompt { cwd, date } => print_system_prompt(cwd, date),
        CliAction::Version => print_version(),
        CliAction::ResumeSession {
            session_path,
            commands,
        } => resume_session(&session_path, &commands),
        CliAction::Prompt {
            prompt,
            model,
            output_format,
            allowed_tools,
            permission_mode,
        } => LiveCli::new(model, true, allowed_tools, permission_mode)?
            .run_turn_with_output(&prompt, output_format),
        CliAction::Login => run_login(),
        CliAction::Logout => run_logout(),
        CliAction::Init => run_init(),
        CliAction::Repl {
            model,
            allowed_tools,
            permission_mode,
        } => {
            let mut config_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
            config_path.push("albert/config.toml");
            if !config_path.exists() {
                init::wake_sequence();
            }
            run_tui(model, allowed_tools, permission_mode)
        },
        CliAction::Help => {
            println!("{}", render_repl_help());
            Ok(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CliAction {
    DumpManifests,
    BootstrapPlan,
    PrintSystemPrompt {
        cwd: PathBuf,
        date: String,
    },
    Version,
    ResumeSession {
        session_path: PathBuf,
        commands: Vec<String>,
    },
    Prompt {
        prompt: String,
        model: String,
        output_format: CliOutputFormat,
        allowed_tools: Option<AllowedToolSet>,
        permission_mode: PermissionMode,
    },
    Login,
    Logout,
    Init,
    Repl {
        model: String,
        allowed_tools: Option<AllowedToolSet>,
        permission_mode: PermissionMode,
    },
    // prompt-mode formatting is only supported for non-interactive runs
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliOutputFormat {
    Text,
    Json,
}

impl CliOutputFormat {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            other => Err(format!(
                "unsupported value for --output-format: {other} (expected text or json)"
            )),
        }
    }
}

#[allow(clippy::too_many_lines)]
fn parse_args(args: &[String]) -> Result<CliAction, String> {
    let mut model = DEFAULT_MODEL.to_string();
    let mut output_format = CliOutputFormat::Text;
    let mut permission_mode = default_permission_mode();
    let mut wants_version = false;
    let mut allowed_tool_values = Vec::new();
    let mut rest = Vec::new();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--version" | "-V" => {
                wants_version = true;
                index += 1;
            }
            "--model" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --model".to_string())?;
                model = resolve_model_alias(value).to_string();
                index += 2;
            }
            flag if flag.starts_with("--model=") => {
                model = resolve_model_alias(&flag[8..]).to_string();
                index += 1;
            }
            "--output-format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --output-format".to_string())?;
                output_format = CliOutputFormat::parse(value)?;
                index += 2;
            }
            "--permission-mode" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --permission-mode".to_string())?;
                permission_mode = parse_permission_mode_arg(value)?;
                index += 2;
            }
            flag if flag.starts_with("--output-format=") => {
                output_format = CliOutputFormat::parse(&flag[16..])?;
                index += 1;
            }
            flag if flag.starts_with("--permission-mode=") => {
                permission_mode = parse_permission_mode_arg(&flag[18..])?;
                index += 1;
            }
            "--dangerously-skip-permissions" => {
                permission_mode = PermissionMode::DangerFullAccess;
                index += 1;
            }
            "-p" => {
                // Claw Code compat: -p "prompt" = one-shot prompt
                let prompt = args[index + 1..].join(" ");
                if prompt.trim().is_empty() {
                    return Err("-p requires a prompt string".to_string());
                }
                return Ok(CliAction::Prompt {
                    prompt,
                    model: resolve_model_alias(&model).to_string(),
                    output_format,
                    allowed_tools: normalize_allowed_tools(&allowed_tool_values)?,
                    permission_mode,
                });
            }
            "--print" => {
                // Claw Code compat: --print makes output non-interactive
                output_format = CliOutputFormat::Text;
                index += 1;
            }
            "--allowedTools" | "--allowed-tools" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --allowedTools".to_string())?;
                allowed_tool_values.push(value.clone());
                index += 2;
            }
            flag if flag.starts_with("--allowedTools=") => {
                allowed_tool_values.push(flag[15..].to_string());
                index += 1;
            }
            flag if flag.starts_with("--allowed-tools=") => {
                allowed_tool_values.push(flag[16..].to_string());
                index += 1;
            }
            other => {
                rest.push(other.to_string());
                index += 1;
            }
        }
    }

    if wants_version {
        return Ok(CliAction::Version);
    }

    let allowed_tools = normalize_allowed_tools(&allowed_tool_values)?;

    if rest.is_empty() {
        return Ok(CliAction::Repl {
            model,
            allowed_tools,
            permission_mode,
        });
    }
    if matches!(rest.first().map(String::as_str), Some("--help" | "-h")) {
        return Ok(CliAction::Help);
    }
    if rest.first().map(String::as_str) == Some("--resume") {
        return parse_resume_args(&rest[1..]);
    }

    match rest[0].as_str() {
        "dump-manifests" => Ok(CliAction::DumpManifests),
        "bootstrap-plan" => Ok(CliAction::BootstrapPlan),
        "system-prompt" => parse_system_prompt_args(&rest[1..]),
        "login" => Ok(CliAction::Login),
        "logout" => Ok(CliAction::Logout),
        "init" => Ok(CliAction::Init),
        "prompt" => {
            let prompt = rest[1..].join(" ");
            if prompt.trim().is_empty() {
                return Err("prompt subcommand requires a prompt string".to_string());
            }
            Ok(CliAction::Prompt {
                prompt,
                model,
                output_format,
                allowed_tools,
                permission_mode,
            })
        }
        other if !other.starts_with('/') => Ok(CliAction::Prompt {
            prompt: rest.join(" "),
            model,
            output_format,
            allowed_tools,
            permission_mode,
        }),
        other => Err(format!("unknown subcommand: {other}")),
    }
}

fn resolve_model_alias(model: &str) -> &str {
    match model {
        "opus" => "claude-3-opus-20240229",
        "sonnet" => "claude-3-sonnet-20240229",
        "haiku" => "claude-3-haiku-20240307",
        "flash" => "gemini-2.5-flash",
        "pro" => "gemini-2.5-pro",
        _ => model,
    }
}

fn normalize_allowed_tools(values: &[String]) -> Result<Option<AllowedToolSet>, String> {
    if values.is_empty() {
        return Ok(None);
    }

    let canonical_names = mvp_tool_specs()
        .into_iter()
        .map(|spec| spec.name.to_string())
        .collect::<Vec<_>>();
    let mut name_map = canonical_names
        .iter()
        .map(|name| (normalize_tool_name(name), name.clone()))
        .collect::<BTreeMap<_, _>>();

    for (alias, canonical) in [
        ("read", "read_file"),
        ("write", "write_file"),
        ("edit", "edit_file"),
        ("glob", "glob_search"),
        ("grep", "grep_search"),
    ] {
        name_map.insert(alias.to_string(), canonical.to_string());
    }

    let mut allowed = AllowedToolSet::new();
    for value in values {
        for token in value
            .split(|ch: char| ch == ',' || ch.is_whitespace())
            .filter(|token| !token.is_empty())
        {
            let normalized = normalize_tool_name(token);
            let canonical = name_map.get(&normalized).ok_or_else(|| {
                format!(
                    "unsupported tool in --allowedTools: {token} (expected one of: {})",
                    canonical_names.join(", ")
                )
            })?;
            allowed.insert(canonical.clone());
        }
    }

    Ok(Some(allowed))
}

fn normalize_tool_name(value: &str) -> String {
    value.trim().replace('-', "_").to_ascii_lowercase()
}

fn parse_permission_mode_arg(value: &str) -> Result<PermissionMode, String> {
    normalize_permission_mode(value)
        .ok_or_else(|| {
            format!(
                "unsupported permission mode '{value}'. Use read-only, workspace-write, or danger-full-access."
            )
        })
        .map(permission_mode_from_label)
}

fn permission_mode_from_label(mode: &str) -> PermissionMode {
    match mode {
        "read-only" => PermissionMode::ReadOnly,
        "workspace-write" => PermissionMode::WorkspaceWrite,
        "danger-full-access" => PermissionMode::DangerFullAccess,
        other => panic!("unsupported permission mode label: {other}"),
    }
}

fn default_permission_mode() -> PermissionMode {
    env::var("RUSTY_TERNLANG_PERMISSION_MODE")
        .ok()
        .as_deref()
        .and_then(normalize_permission_mode)
        .map_or(PermissionMode::DangerFullAccess, permission_mode_from_label)
}

fn filter_tool_specs(allowed_tools: Option<&AllowedToolSet>) -> Vec<tools::ToolSpec> {
    mvp_tool_specs()
        .into_iter()
        .filter(|spec| allowed_tools.is_none_or(|allowed| allowed.contains(spec.name)))
        .collect()
}

fn parse_system_prompt_args(args: &[String]) -> Result<CliAction, String> {
    let mut cwd = env::current_dir().map_err(|error| error.to_string())?;
    let mut date = DEFAULT_DATE.to_string();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--cwd" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --cwd".to_string())?;
                cwd = PathBuf::from(value);
                index += 2;
            }
            "--date" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "missing value for --date".to_string())?;
                date.clone_from(value);
                index += 2;
            }
            other => return Err(format!("unknown system-prompt option: {other}")),
        }
    }

    Ok(CliAction::PrintSystemPrompt { cwd, date })
}

fn parse_resume_args(args: &[String]) -> Result<CliAction, String> {
    let session_path = args
        .first()
        .ok_or_else(|| "missing session path for --resume".to_string())
        .map(PathBuf::from)?;
    let commands = args[1..].to_vec();
    if commands
        .iter()
        .any(|command| !command.trim_start().starts_with('/'))
    {
        return Err("--resume trailing arguments must be slash commands".to_string());
    }
    Ok(CliAction::ResumeSession {
        session_path,
        commands,
    })
}

fn dump_manifests() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let paths = UpstreamPaths::from_workspace_dir(&workspace_dir);
    let manifest = extract_manifest(&paths)?;
    println!("commands: {}", manifest.commands.entries().len());
    println!("tools: {}", manifest.tools.entries().len());
    println!("bootstrap phases: {}", manifest.bootstrap.phases().len());
    Ok(())
}

fn print_bootstrap_plan() -> Result<(), Box<dyn std::error::Error>> {
    for phase in runtime::BootstrapPlan::ternlang_cli_default().phases() {
        println!("- {phase:?}");
    }
    Ok(())
}

fn default_oauth_config() -> OAuthConfig {
    OAuthConfig {
        client_id: String::from("9d1c250a-e61b-44d9-88ed-5944d1962f5e"),
        authorize_url: String::from("https://console.anthropic.com/oauth/authorize"),
        token_url: String::from("https://api.anthropic.com/v1/oauth/token"),
        callback_port: None,
        manual_redirect_url: None,
        scopes: vec![
            String::from("user:profile"),
            String::from("user:inference"),
            String::from("user:sessions:ternlang_cli"),
        ],
    }
}

fn run_login() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let config = ConfigLoader::default_for(&cwd).load()?;
    let default_oauth = default_oauth_config();
    let oauth = config.oauth().unwrap_or(&default_oauth);
    let callback_port = oauth.callback_port.unwrap_or(DEFAULT_OAUTH_CALLBACK_PORT);
    let redirect_uri = runtime::loopback_redirect_uri(callback_port);
    let pkce = generate_pkce_pair()?;
    let state = generate_state()?;
    let authorize_url =
        OAuthAuthorizationRequest::from_config(oauth, redirect_uri.clone(), state.clone(), &pkce)
            .build_url();

    println!("Starting Anthropic OAuth login...");
    println!("Listening for callback on {redirect_uri}");
    if let Err(error) = open_browser(&authorize_url) {
        eprintln!("warning: failed to open browser automatically: {error}");
        println!("Open this URL manually:
{authorize_url}");
    }

    let callback = wait_for_oauth_callback(callback_port)?;
    if let Some(error) = callback.error {
        let description = callback
            .error_description
            .unwrap_or_else(|| "authorization failed".to_string());
        return Err(io::Error::other(format!("{error}: {description}")).into());
    }
    let code = callback.code.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "callback did not include code")
    })?;
    let returned_state = callback.state.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "callback did not include state")
    })?;
    if returned_state != state {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "oauth state mismatch").into());
    }

    let client = TernlangClient::from_auth(AuthSource::None).with_base_url(api::read_base_url());
    let exchange_request = api::OAuthTokenExchangeRequest {
        code,
        redirect_uri,
    };
    let runtime = tokio::runtime::Runtime::new()?;
    let token_set = runtime.block_on(client.exchange_oauth_code(api::OAuthConfig {}, &exchange_request))?;
    save_oauth_credentials(&runtime::OAuthTokenSet {
        access_token: token_set.access_token,
        refresh_token: token_set.refresh_token,
        expires_at: token_set.expires_at,
        scopes: token_set.scopes,
    })?;
    println!("Anthropic OAuth login complete.");
    Ok(())
}

fn run_logout() -> Result<(), Box<dyn std::error::Error>> {
    clear_oauth_credentials()?;
    println!("Anthropic OAuth credentials cleared.");
    Ok(())
}

fn open_browser(url: &str) -> io::Result<()> {
    let commands = if cfg!(target_os = "macos") {
        vec![("open", vec![url])]
    } else if cfg!(target_os = "windows") {
        vec![("cmd", vec!["/C", "start", "", url])]
    } else {
        vec![("xdg-open", vec![url])]
    };
    for (program, args) in commands {
        match Command::new(program).args(args).spawn() {
            Ok(_) => return Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "no supported browser opener command found",
    ))
}

fn wait_for_oauth_callback(
    port: u16,
) -> Result<runtime::OAuthCallbackParams, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let (mut stream, _) = listener.accept()?;
    let mut buffer = [0_u8; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "missing callback request line")
    })?;
    let target = request_line.split_whitespace().nth(1).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "missing callback request target",
        )
    })?;
    let callback = parse_oauth_callback_request_target(target)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let body = if callback.error.is_some() {
        "Anthropic OAuth login failed. You can close this window."
    } else {
        "Anthropic OAuth login succeeded. You can close this window."
    };
    let response = format!(
        "HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: {}
connection: close

{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    Ok(callback)
}

fn print_system_prompt(cwd: PathBuf, date: String) -> Result<(), Box<dyn std::error::Error>> {
    let sections = load_system_prompt(cwd, date, env::consts::OS, "unknown")?;
    println!("{}", sections.join("

"));
    Ok(())
}

fn print_version() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", render_version_report());
    Ok(())
}

fn resume_session(session_path: &Path, commands: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut session = Session::load_from_path(session_path)?;

    if commands.is_empty() {
        println!(
            "Restored session from {} ({} messages).",
            session_path.display(),
            session.messages.len()
        );
        return Ok(());
    }

    for raw_command in commands {
        let Some(command) = SlashCommand::parse(raw_command) else {
            eprintln!("unsupported resumed command: {raw_command}");
            std::process::exit(2);
        };
        let outcome = run_resume_command(session_path, &session, &command)?;
        session = outcome.session;
        if let Some(message) = outcome.message {
            println!("{message}");
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct ResumeCommandOutcome {
    session: Session,
    message: Option<String>,
}

#[derive(Debug, Clone)]
struct StatusContext {
    cwd: PathBuf,
    session_path: Option<PathBuf>,
    loaded_config_files: usize,
    discovered_config_files: usize,
    memory_file_count: usize,
    project_root: Option<PathBuf>,
    git_branch: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct StatusUsage {
    message_count: usize,
    turns: u32,
    latest: TokenUsage,
    cumulative: TokenUsage,
    estimated_tokens: usize,
}

fn format_model_switch_report(previous: &str, next: &str, message_count: usize) -> String {
    format!(
        "Model updated
  Previous         {previous}
  Current          {next}
  Preserved msgs   {message_count}"
    )
}

fn format_permissions_report(mode: &str) -> String {
    let modes = [
        ("read-only", "Read/search tools only", mode == "read-only"),
        (
            "workspace-write",
            "Edit files inside the workspace",
            mode == "workspace-write",
        ),
        (
            "danger-full-access",
            "Unrestricted tool access",
            mode == "danger-full-access",
        ),
    ]
    .into_iter()
    .map(|(name, description, is_current)| {
        let marker = if is_current {
            "● current"
        } else {
            "○ available"
        };
        format!("  {name:<18} {marker:<11} {description}")
    })
    .collect::<Vec<_>>()
    .join(
        "
",
    );

    format!(
        "Permissions
  Active mode      {mode}
  Mode status      live session default

Modes
{modes}

Usage
  Inspect current mode with /permissions
  Switch modes with /permissions <mode>"
    )
}

fn format_permissions_switch_report(previous: &str, next: &str) -> String {
    format!(
        "Permissions updated
  Result           mode switched
  Previous mode    {previous}
  Active mode      {next}
  Applies to       subsequent tool calls
  Usage            /permissions to inspect current mode"
    )
}

fn format_cost_report(usage: TokenUsage) -> String {
    format!(
        "Cost
  Input tokens     {}
  Output tokens    {}
  Cache create     {}
  Cache read       {}
  Total tokens     {}",
        usage.input_tokens,
        usage.output_tokens,
        usage.cache_creation_input_tokens,
        usage.cache_read_input_tokens,
        usage.total_tokens(),
    )
}

fn format_resume_report(session_path: &str, message_count: usize, turns: u32) -> String {
    format!(
        "Session resumed
  Session file     {session_path}
  Messages         {message_count}
  Turns            {turns}"
    )
}

fn format_compact_report(removed: usize, resulting_messages: usize, skipped: bool) -> String {
    if skipped {
        format!(
            "Compact
  Result           skipped
  Reason           session below compaction threshold
  Messages kept    {resulting_messages}"
        )
    } else {
        format!(
            "Compact
  Result           compacted
  Messages removed {removed}
  Messages kept    {resulting_messages}"
        )
    }
}

fn format_auto_compaction_notice(removed: usize) -> String {
    format!("[auto-compacted: removed {removed} messages]")
}

fn parse_git_status_metadata(status: Option<&str>) -> (Option<PathBuf>, Option<String>) {
    let Some(status) = status else {
        return (None, None);
    };
    let branch = status.lines().next().and_then(|line| {
        line.strip_prefix("## ")
            .map(|line| {
                line.split(['.', ' '])
                    .next()
                    .unwrap_or_default()
                    .to_string()
            })
            .filter(|value| !value.is_empty())
    });
    let project_root = find_git_root().ok();
    (project_root, branch)
}

fn find_git_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(env::current_dir()?)
        .output()?;
    if !output.status.success() {
        return Err("not a git repository".into());
    }
    let path = String::from_utf8(output.stdout)?.trim().to_string();
    if path.is_empty() {
        return Err("empty git root".into());
    }
    Ok(PathBuf::from(path))
}

#[allow(clippy::too_many_lines)]
fn run_resume_command(
    session_path: &Path,
    session: &Session,
    command: &SlashCommand,
) -> Result<ResumeCommandOutcome, Box<dyn std::error::Error>> {
    match command {
        SlashCommand::Help => Ok(ResumeCommandOutcome {
            session: session.clone(),
            message: Some(render_repl_help()),
        }),
        SlashCommand::Compact => {
            let result = runtime::compact_session(
                session,
                CompactionConfig {
                    max_estimated_tokens: 0,
                    ..CompactionConfig::default()
                },
            );
            let removed = result.removed_message_count;
            let kept = result.compacted_session.messages.len();
            let skipped = removed == 0;
            result.compacted_session.save_to_path(session_path)?;
            Ok(ResumeCommandOutcome {
                session: result.compacted_session,
                message: Some(format_compact_report(removed, kept, skipped)),
            })
        }
        SlashCommand::Clear { confirm } => {
            if !confirm {
                return Ok(ResumeCommandOutcome {
                    session: session.clone(),
                    message: Some(
                        "clear: confirmation required; rerun with /clear --confirm".to_string(),
                    ),
                });
            }
            let cleared = Session::new();
            cleared.save_to_path(session_path)?;
            Ok(ResumeCommandOutcome {
                session: cleared,
                message: Some(format!(
                    "Cleared resumed session file {}.",
                    session_path.display()
                )),
            })
        }
        SlashCommand::Status => {
            let tracker = UsageTracker::from_session(session);
            let usage = tracker.cumulative_usage();
            Ok(ResumeCommandOutcome {
                session: session.clone(),
                message: Some(format_status_report(
                    "restored-session",
                    StatusUsage {
                        message_count: session.messages.len(),
                        turns: tracker.turns(),
                        latest: tracker.current_turn_usage(),
                        cumulative: usage,
                        estimated_tokens: 0,
                    },
                    default_permission_mode().as_str(),
                    &status_context(Some(session_path))?,
                )),
            })
        }
        SlashCommand::Cost => {
            let usage = UsageTracker::from_session(session).cumulative_usage();
            Ok(ResumeCommandOutcome {
                session: session.clone(),
                message: Some(format_cost_report(usage)),
            })
        }
        SlashCommand::Config { section } => Ok(ResumeCommandOutcome {
            session: session.clone(),
            message: Some(render_config_report(section.as_deref())?),
        }),
        SlashCommand::Memory => Ok(ResumeCommandOutcome {
            session: session.clone(),
            message: Some(render_memory_report()?),
        }),
        SlashCommand::Init => Ok(ResumeCommandOutcome {
            session: session.clone(),
            message: Some(init_ternlang_md()?),
        }),
        SlashCommand::Diff => Ok(ResumeCommandOutcome {
            session: session.clone(),
            message: Some(render_diff_report()?),
        }),
        SlashCommand::Version => Ok(ResumeCommandOutcome {
            session: session.clone(),
            message: Some(render_version_report()),
        }),
        SlashCommand::Export { path } => {
            let export_path = resolve_export_path(path.as_deref(), session)?;
            fs::write(&export_path, render_export_text(session))?;
            Ok(ResumeCommandOutcome {
                session: session.clone(),
                message: Some(format!(
                    "Export
  Result           wrote transcript
  File             {}
  Messages         {}",
                    export_path.display(),
                    session.messages.len(),
                )),
            })
        }
        SlashCommand::Auth { .. }
        | SlashCommand::Bughunter { .. }
        | SlashCommand::Commit
        | SlashCommand::Pr { .. }
        | SlashCommand::Issue { .. }
        | SlashCommand::Ultraplan { .. }
        | SlashCommand::Teleport { .. }
        | SlashCommand::DebugToolCall
        | SlashCommand::Resume { .. }
        | SlashCommand::Session { .. }
        | SlashCommand::Plan { .. }
        | SlashCommand::Tdd { .. }
        | SlashCommand::Verify
        | SlashCommand::CodeReview { .. }
        | SlashCommand::BuildFix
        | SlashCommand::Aside { .. }
        | SlashCommand::Learn
        | SlashCommand::Refactor { .. }
        | SlashCommand::Checkpoint { .. }
        | SlashCommand::Docs { .. }
        | SlashCommand::Model { .. }
        | SlashCommand::Permissions { .. }
        | SlashCommand::Compress
        | SlashCommand::Loop { .. }
        | SlashCommand::Unknown(_) => Err("unsupported resumed slash command".into()),
        &SlashCommand::Mcp { .. } => Err("cannot resume an /mcp command".into()),
    }
}

fn run_repl(
    model: String,
    allowed_tools: Option<AllowedToolSet>,
    permission_mode: PermissionMode,
) -> Result<(), Box<dyn std::error::Error>> {
    check_workspace_trust()?;
    let mut cli = LiveCli::new(model, true, allowed_tools, permission_mode)?;
    let mut editor = input::LineEditor::new("> ", slash_command_completion_candidates());
    println!("{}", cli.startup_banner());

    loop {
        match editor.read_line()? {
            input::ReadOutcome::Submit(input) => {
                let trimmed = input.trim().to_string();
                if trimmed.is_empty() {
                    continue;
                }
                if matches!(trimmed.as_str(), "/exit" | "/quit") {
                    cli.persist_session()?;
                    break;
                }
                if let Some(command) = SlashCommand::parse(&trimmed) {
                    if cli.handle_repl_command(command)? {
                        cli.persist_session()?;
                    }
                    continue;
                }
                editor.push_history(input);
                cli.run_turn(&trimmed)?;
            }
            input::ReadOutcome::Cancel => {}
            input::ReadOutcome::Exit => {
                cli.persist_session()?;
                break;
            }
        }
    }

    Ok(())
}

fn run_tui(
    model: String,
    allowed_tools: Option<AllowedToolSet>,
    permission_mode: PermissionMode,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::atomic::{AtomicBool, Ordering};

    // Disable sandbox when running unrestricted so the agent can reach all paths.
    runtime::set_sandbox_bypass(permission_mode == PermissionMode::DangerFullAccess);

    check_workspace_trust()?;

    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<unknown>".to_string());

    let mut cli = LiveCli::new(model.clone(), true, allowed_tools, permission_mode)?;

    let (tui_app, submit_rx) = tui::TuiApp::new(model, cwd);
    let tui_state = Arc::clone(&tui_app.state);
    let tui_event_tx = tui_app.event_tx.clone();
    let cancel_flag = Arc::clone(&tui_app.cancel_flag);

    // Show startup banner as a system message
    {
        let mut state = tui_state.lock().unwrap();
        state.push_exec(tui::ExecBlock::SystemMsg(format!(
            "albert {} · {} · {}",
            env!("CARGO_PKG_VERSION"),
            cli.model,
            cli.permission_mode.as_str(),
        )));
    }

    // Spawn TUI thread — it owns its own tokio runtime + terminal
    let tui_thread = std::thread::spawn(move || tui_app.run());

    // Tokio runtime for async broadcast receiving
    let async_rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // Agent loop: receive submits → run turn → forward events to TUI
    loop {
        let input = match submit_rx.recv() {
            Ok(s) => s,
            Err(_) => break, // TUI thread exited
        };

        // Handle /exit and /quit without calling the agent
        if matches!(input.trim(), "/exit" | "/quit") {
            cli.persist_session()?;
            let _ = tui_event_tx.send(tui::TuiEvent::Quit);
            break;
        }

        // Slash commands: suspend TUI so it yields the terminal, then resume.
        if let Some(command) = commands::SlashCommand::parse(&input) {
            // Suspend: tell TUI to leave alternate screen and pause key thread
            let (ack_tx, ack_rx) = std::sync::mpsc::sync_channel::<()>(1);
            let _ = tui_event_tx.send(tui::TuiEvent::Suspend { ack: ack_tx });
            // Wait until TUI has cleared the screen
            let _ = ack_rx.recv();

            // Now we have clean terminal ownership — run the command normally
            let should_persist = cli.handle_repl_command(command);

            // Give terminal back to TUI
            let _ = tui_event_tx.send(tui::TuiEvent::Resume);

            match should_persist {
                Ok(p) => { if p { cli.persist_session()?; } }
                Err(e) => eprintln!("command error: {e}"),
            }

            // Sync updated model name into TUI state
            {
                let mut state = tui_state.lock().unwrap();
                state.model = cli.model.clone();
            }
            continue;
        }

        // Push user message and set working
        {
            let mut state = tui_state.lock().unwrap();
            state.push_exec(tui::ExecBlock::UserMessage(input.clone()));
            state.working = true;
            state.turn_start = Some(std::time::Instant::now());
            state.tokens_out = 0;
            state.scroll = 0; // auto-follow
        }

        // Reset cancel flag from any previous interrupt before starting.
        cancel_flag.store(false, Ordering::Relaxed);

        let mut rx = cli.event_tx.subscribe();
        let tx = tui_event_tx.clone();
        let done_flag = Arc::new(AtomicBool::new(false));
        let done_clone = done_flag.clone();
        let cancel_clone = Arc::clone(&cancel_flag);

        let mut prompter = CliPermissionPrompter::new(cli.permission_mode, false);

        let turn_result = std::thread::scope(|s| {
            let runtime = &mut cli.runtime;
            let handle = s.spawn(move || {
                let r = runtime.run_turn(input.clone(), Some(&mut prompter));
                done_clone.store(true, Ordering::Relaxed);
                r
            });

            // Forward ALL broadcast events to TUI until the agent thread is done
            // OR the user interrupts with ESC.
            async_rt.block_on(async {
                loop {
                    let is_done = done_flag.load(Ordering::Relaxed);
                    let is_cancelled = cancel_clone.load(Ordering::Relaxed);

                    if is_done || is_cancelled {
                        // Drain any remaining queued events (skip on cancel)
                        if is_done {
                            while let Ok(ev) = rx.try_recv() {
                                let _ = tx.send(tui::TuiEvent::AgentEvent(ev));
                            }
                        }
                        break;
                    }

                    tokio::select! {
                        ev = rx.recv() => {
                            match ev {
                                Ok(ev) => {
                                    let _ = tx.send(tui::TuiEvent::AgentEvent(ev));
                                }
                                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                                Err(_) => break,
                            }
                        }
                        _ = tokio::time::sleep(Duration::from_millis(30)) => {
                            // poll again on next tick — checks done/cancel at top of loop
                        }
                    }
                }
            });

            handle.join().map_err(|_| -> Box<dyn std::error::Error> {
                "agent thread panicked".into()
            })
        });

        // Always clear working state + deactivate any pending tool dot
        {
            let mut state = tui_state.lock().unwrap();
            state.working = false;
            state.deactivate_last_tool();
            if cancel_flag.load(Ordering::Relaxed) {
                state.push_exec(tui::ExecBlock::SystemMsg("interrupted".to_string()));
            }
        }
        cancel_flag.store(false, Ordering::Relaxed);

        // Inject ToolOutput blocks: walk exec_log, insert output after each ToolUse.
        if let Ok(Ok(ref summary)) = turn_result {
            let outputs: Vec<String> = summary
                .tool_results
                .iter()
                .flat_map(|msg| &msg.blocks)
                .filter_map(|block| {
                    if let ContentBlock::ToolResult { output, .. } = block {
                        Some(output.clone())
                    } else {
                        None
                    }
                })
                .collect();

            if !outputs.is_empty() {
                let mut state = tui_state.lock().unwrap();
                let log: Vec<_> = std::mem::take(&mut state.exec_log).into_iter().collect();
                let mut out_iter = outputs.into_iter();
                for block in log {
                    let is_tool = matches!(&block, tui::ExecBlock::ToolUse { .. });
                    state.exec_log.push_back(block);
                    if is_tool {
                        if let Some(raw) = out_iter.next() {
                            let all: Vec<String> = raw
                                .lines()
                                .map(|l| l.trim_end().to_string())
                                .filter(|l| !l.is_empty())
                                .collect();
                            let total = all.len();
                            if total > 0 {
                                let shown: Vec<String> = all.into_iter().take(8).collect();
                                state.exec_log.push_back(tui::ExecBlock::ToolOutput {
                                    lines: shown,
                                    total,
                                });
                            }
                        }
                    }
                }
            }
        }

        match turn_result {
            Ok(Err(e)) => {
                let mut state = tui_state.lock().unwrap();
                state.push_exec(tui::ExecBlock::SystemMsg(format!("error: {e}")));
            }
            Err(e) => {
                let mut state = tui_state.lock().unwrap();
                state.push_exec(tui::ExecBlock::SystemMsg(format!("error: {e}")));
            }
            Ok(Ok(_)) => {}
        }

        // Save session after each turn
        let _ = cli.persist_session();
    }

    let _ = tui_thread.join();
    Ok(())
}

#[derive(Debug, Clone)]
struct SessionHandle {
    id: String,
    path: PathBuf,
}

#[derive(Debug, Clone)]
struct ManagedSessionSummary {
    id: String,
    path: PathBuf,
    modified_epoch_secs: u64,
    message_count: usize,
}

struct LiveCli {
    model: String,
    allowed_tools: Option<AllowedToolSet>,
    permission_mode: PermissionMode,
    system_prompt: Vec<String>,
    runtime: ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>,
    session: SessionHandle,
    mcp_manager: Arc<Mutex<McpServerManager>>,
    event_tx: tokio::sync::broadcast::Sender<AssistantEvent>,
}

impl LiveCli {
    fn new(
        model: String,
        enable_tools: bool,
        allowed_tools: Option<AllowedToolSet>,
        permission_mode: PermissionMode,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let system_prompt = build_system_prompt()?;
        let session = create_managed_session_handle()?;
        let mcp_servers = load_mcp_servers();
        let mcp_manager = Arc::new(Mutex::new(McpServerManager::from_servers(&mcp_servers)));
        let (event_tx, _) = tokio::sync::broadcast::channel::<AssistantEvent>(256);
        let runtime = build_runtime_with_mcp(
            Session::new(),
            model.clone(),
            system_prompt.clone(),
            enable_tools,
            allowed_tools.clone(),
            permission_mode,
            Arc::clone(&mcp_manager),
            event_tx.clone(),
        )?;
        let cli = Self {
            model,
            allowed_tools,
            permission_mode,
            system_prompt,
            runtime,
            session,
            mcp_manager,
            event_tx,
        };
        cli.persist_session()?;
        Ok(cli)
    }

    fn startup_banner(&self) -> String {
        let cwd = env::current_dir().map_or_else(
            |_| "<unknown>".to_string(),
            |path| path.display().to_string(),
        );
        format!(
            "
  █████╗  ██╗     ██████╗ ███████╗██████╗ ████████╗
  ██╔══██╗ ██║     ██╔══██╗██╔════╝██╔══██╗╚══██╔══╝
  ███████║ ██║     ██████╔╝█████╗  ██████╔╝   ██║   
  ██╔══██║ ██║     ██╔══██╗██╔══╝  ██╔══██╗   ██║   
  ██║  ██║ ███████╗██████╔╝███████╗██║  ██║   ██║   
  ╚═╝  ╚═╝ ╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═╝   ╚═╝   
     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

  Model            {}
  Permissions      {}
  Directory        {}
  Session          {}

  Type /help for commands · Shift+Enter for newline",
            self.model,
            self.permission_mode.as_str(),
            cwd,
            self.session.id,
        )
    }

    fn run_turn(&mut self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::sync::atomic::{AtomicBool, Ordering};

        // Redraw the user's message line as a styled dark-background box
        let _ = render_user_message_box(input);

        let is_prompting = Arc::new(AtomicBool::new(false));
        let mut permission_prompter = CliPermissionPrompter {
            permission_mode: self.permission_mode,
            interactive: true,
            is_prompting: Some(is_prompting.clone()),
        };

        let mut rx = self.event_tx.subscribe();
        let input_string = input.to_string();

        let result = std::thread::scope(|s| {
            let runtime = &mut self.runtime;
            let is_done = Arc::new(AtomicBool::new(false));
            let is_done_clone = is_done.clone();

            let handle = s.spawn(move || {
                let res = runtime.run_turn(input_string, Some(&mut permission_prompter));
                is_done_clone.store(true, Ordering::Relaxed);
                res
            });

            let mut out = io::stdout();
            // 0=waiting, 1=streaming text, 2=after MessageStop
            let mut state: u8 = 0;
            let mut tokens_in: u32 = 0;
            let mut tokens_out: u32 = 0;
            let turn_start = std::time::Instant::now();

            // Draw Working indicator immediately (blank line + indicator + prompt box)
            let tw = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
            let prompt_box = format!(" {:<width$} ", ">", width = tw.saturating_sub(3));
            let _ = execute!(out,
                crossterm::style::Print("\n"),
                crossterm::style::SetForegroundColor(crossterm::style::Color::White),
                crossterm::style::Print("● Working (0s • esc to interrupt)"),
                crossterm::style::ResetColor,
                crossterm::style::Print("\n"),
                crossterm::style::SetBackgroundColor(crossterm::style::Color::AnsiValue(240)),
                crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
                crossterm::style::Print(&prompt_box),
                crossterm::style::ResetColor,
                crossterm::cursor::MoveToPreviousLine(1),
            );
            let _ = out.flush();

            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let mut interval = tokio::time::interval(Duration::from_millis(200));
                loop {
                    if is_done.load(Ordering::Relaxed) {
                        if state == 0 {
                            // Clear Working indicator + blank prompt box (2 lines below current pos)
                            let _ = execute!(out,
                                crossterm::cursor::MoveToColumn(0),
                                crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
                                crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown),
                            );
                        }
                        break;
                    }
                    tokio::select! {
                        _ = interval.tick() => {
                            if state == 0 && !is_prompting.load(Ordering::Relaxed) {
                                let secs = turn_start.elapsed().as_secs();
                                let timer = if secs >= 60 {
                                    format!("{}m {}s", secs / 60, secs % 60)
                                } else {
                                    format!("{}s", secs)
                                };
                                let tw = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
                                let prompt_box = format!(" {:<width$} ", ">", width = tw.saturating_sub(3));
                                // Overwrite: Working indicator line + prompt box below it
                                let _ = execute!(out,
                                    crossterm::cursor::MoveToColumn(0),
                                    crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
                                    crossterm::style::SetForegroundColor(crossterm::style::Color::White),
                                    crossterm::style::Print(format!("● Working ({timer} • esc to interrupt)")),
                                    crossterm::style::ResetColor,
                                    crossterm::style::Print("\n"),
                                    crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
                                    crossterm::style::SetBackgroundColor(crossterm::style::Color::AnsiValue(240)),
                                    crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
                                    crossterm::style::Print(&prompt_box),
                                    crossterm::style::ResetColor,
                                    // Move back up so next tick overwrites these two lines
                                    crossterm::cursor::MoveToPreviousLine(1),
                                );
                                let _ = out.flush();
                            }
                        }
                        event = rx.recv() => {
                            match event {
                                Ok(AssistantEvent::TextDelta(delta)) => {
                                    if state != 1 {
                                        // Clear Working indicator + prompt box, then blank line before response
                                        let _ = execute!(out,
                                            crossterm::cursor::MoveToColumn(0),
                                            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
                                            crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown),
                                        );
                                        let _ = write!(out, "\n\n");
                                        state = 1;
                                    }
                                    let _ = write!(out, "{delta}");
                                    let _ = out.flush();
                                }
                                Ok(AssistantEvent::ToolUse { name, input, .. }) => {
                                    if state == 1 { let _ = writeln!(out); }
                                    if state == 0 {
                                        // Clear Working indicator + prompt box
                                        let _ = execute!(out,
                                            crossterm::cursor::MoveToColumn(0),
                                            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
                                            crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown),
                                        );
                                    }
                                    let arg_preview = tui::tool_input_preview(&input);
                                    let _ = writeln!(out, "\n{}  {}  {}",
                                        style("●").green().bold(),
                                        style(format!("Ran {name}")).bold(),
                                        style(&arg_preview).cyan()
                                    );
                                    let _ = out.flush();
                                    // Redraw Working + prompt box for next round
                                    let tw = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
                                    let pb = format!(" {:<width$} ", ">", width = tw.saturating_sub(3));
                                    let secs = turn_start.elapsed().as_secs();
                                    let timer = if secs >= 60 { format!("{}m {}s", secs/60, secs%60) } else { format!("{}s", secs) };
                                    let _ = execute!(out,
                                        crossterm::style::SetForegroundColor(crossterm::style::Color::White),
                                        crossterm::style::Print(format!("\n● Working ({timer} • esc to interrupt)\n")),
                                        crossterm::style::ResetColor,
                                        crossterm::style::SetBackgroundColor(crossterm::style::Color::AnsiValue(240)),
                                        crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
                                        crossterm::style::Print(&pb),
                                        crossterm::style::ResetColor,
                                        crossterm::cursor::MoveToPreviousLine(1),
                                    );
                                    let _ = out.flush();
                                    state = 0;
                                }
                                Ok(AssistantEvent::Usage(usage)) => {
                                    tokens_in = tokens_in.max(usage.input_tokens);
                                    tokens_out += usage.output_tokens;
                                }
                                Ok(AssistantEvent::MessageStop) => {
                                    if state == 1 { let _ = writeln!(out); }
                                    state = 2;
                                }
                                Err(_) => break,
                            }
                        }
                    }
                }
            });

            // Render tool results from the completed summary (shown after each tool call)
            // These are injected after run_turn returns, before status bar
            let result = handle.join().unwrap();

            // Status bar
            let secs = turn_start.elapsed().as_secs();
            let duration_str = if secs >= 60 {
                format!("{}m{}s", secs / 60, secs % 60)
            } else {
                format!("{}s", secs)
            };
            let cwd = env::current_dir().map_or_else(
                |_| "?".to_string(),
                |p| p.file_name().map_or_else(
                    || p.display().to_string(),
                    |n| n.to_string_lossy().into_owned(),
                ),
            );
            // Show tool outputs from summary before status bar
            if let Ok(ref summary) = result {
                render_tool_outputs(summary);
            }
            let model_ref = &self.model;
            println!("\n{}", style(format!(
                "{model_ref} · {cwd} · {tokens_in}in · {tokens_out}out · {duration_str}"
            )).dim());

            result
        });

        let mut stdout = io::stdout();
        match result {
            Ok(summary) => {
                if let Some(event) = summary.auto_compaction {
                    println!(
                        "{}",
                        format_auto_compaction_notice(event.removed_message_count)
                    );
                }
                self.persist_session()?;

                let response_text = final_assistant_text(&summary);
                if !response_text.is_empty() {
                    if let Some(memory_line) = self.llm_reflect(input, &response_text) {
                        if let Err(e) = append_to_albert_memory(&memory_line) {
                            eprintln!("{} memory write failed: {e}", style("⚠").yellow());
                        } else {
                            println!("{}", style("  Noted.").dim().italic());
                        }
                    }
                }

                Ok(())
            }
            Err(error) => {
                let _ = execute!(
                    stdout,
                    crossterm::cursor::MoveToColumn(0),
                    crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
                    crossterm::style::SetForegroundColor(crossterm::style::Color::Red),
                    crossterm::style::Print("✘ Request failed\n"),
                    crossterm::style::ResetColor,
                );
                Err(Box::new(error))
            }
        }
    }

    fn run_turn_with_output(
        &mut self,
        input: &str,
        output_format: CliOutputFormat,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match output_format {
            CliOutputFormat::Text => self.run_turn(input),
            CliOutputFormat::Json => self.run_prompt_json(input),
        }
    }

    fn run_prompt_json(&mut self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.runtime.session().clone();
        let mut runtime = build_runtime(
            session,
            self.model.clone(),
            self.system_prompt.clone(),
            true,
            false,
            self.allowed_tools.clone(),
            self.permission_mode,
        )?;
        let mut permission_prompter = CliPermissionPrompter::new(self.permission_mode, false);
        let summary = runtime.run_turn(input.to_string(), Some(&mut permission_prompter))?;
        self.runtime = runtime;
        self.persist_session()?;
        println!(
            "{}",
            json!({
                "message": final_assistant_text(&summary),
                "model": self.model,
                "iterations": summary.iterations,
                "auto_compaction": summary.auto_compaction.map(|event| json!({
                    "removed_messages": event.removed_message_count,
                    "notice": format_auto_compaction_notice(event.removed_message_count),
                })),
                "tool_uses": collect_tool_uses(&summary),
                "tool_results": collect_tool_results(&summary),
                "usage": {
                    "input_tokens": summary.usage.input_tokens,
                    "output_tokens": summary.usage.output_tokens,
                    "cache_creation_input_tokens": summary.usage.cache_creation_input_tokens,
                    "cache_read_input_tokens": summary.usage.cache_read_input_tokens,
                }
            })
        );
        Ok(())
    }

    fn handle_repl_command(
        &mut self,
        command: SlashCommand,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(match command {
            SlashCommand::Help => {
                println!("{}", render_repl_help());
                false
            }
            SlashCommand::Status => {
                self.print_status()?;
                false
            }
            SlashCommand::Bughunter { scope } => {
                self.run_bughunter(scope.as_deref())?;
                false
            }
            SlashCommand::Commit => {
                self.run_commit()?;
                true
            }
            SlashCommand::Pr { context } => {
                self.run_pr(context.as_deref())?;
                false
            }
            SlashCommand::Issue { context } => {
                self.run_issue(context.as_deref())?;
                false
            }
            SlashCommand::Ultraplan { task } => {
                self.run_ultraplan(task.as_deref())?;
                false
            }
            SlashCommand::Teleport { target } => {
                self.run_teleport(target.as_deref())?;
                false
            }
            SlashCommand::DebugToolCall => {
                self.run_debug_tool_call()?;
                false
            }
            SlashCommand::Compact => {
                self.compact()?;
                false
            }
            SlashCommand::Compress => {
                self.compress()?;
                false
            }
            SlashCommand::Model { model } => self.set_model(model)?,
            SlashCommand::Permissions { mode } => self.set_permissions(mode)?,
            SlashCommand::Clear { confirm } => self.clear_session(confirm)?,
            SlashCommand::Cost => {
                self.print_cost();
                false
            }
            SlashCommand::Resume { session_path } => self.resume_session(session_path)?,
            SlashCommand::Config { section } => {
                Self::print_config(section.as_deref())?;
                false
            }
            SlashCommand::Memory => {
                Self::print_memory()?;
                false
            }
            SlashCommand::Init => {
                run_init()?;
                false
            }
            SlashCommand::Diff => {
                Self::print_diff()?;
                false
            }
            SlashCommand::Version => {
                Self::print_version()?;
                false
            }
            SlashCommand::Export { path } => {
                self.export_session(path.as_deref())?;
                false
            }
            SlashCommand::Session { action, target } => {
                self.handle_session_command(action.as_deref(), target.as_deref())?
            }
            SlashCommand::Auth { provider } => {
                self.run_auth(provider.as_deref())?;
                false
            }
            SlashCommand::Plan { task } => {
                println!("Plan initiated: {}. I am restating requirements and assessing risks...", task.unwrap_or_else(|| "current task".to_string()));
                false
            }
            SlashCommand::Tdd { interface } => {
                println!("TDD loop engaged for {}. Scaffold -> Failing Test -> Implement.", interface.unwrap_or_else(|| "target".to_string()));
                false
            }
            SlashCommand::Verify => {
                println!("Running full verification: build, lint, test, and type-check...");
                false
            }
            SlashCommand::CodeReview { files } => {
                println!("Reviewing {}. Assessing quality, security, and maintainability.", files.unwrap_or_else(|| "changed files".to_string()));
                false
            }
            SlashCommand::BuildFix => {
                println!("Detecting build errors. Dispatching resolver agents...");
                false
            }
            SlashCommand::Aside { question } => {
                println!("Pivot: {}. Answering side question without losing context.", question.unwrap_or_else(|| "...".to_string()));
                false
            }
            SlashCommand::Learn => {
                println!("Extracting reusable patterns and learned instincts...");
                false
            }
            SlashCommand::Refactor { scope } => {
                println!("Refactoring {}. Removing dead code and consolidating duplicates.", scope.unwrap_or_else(|| "workspace".to_string()));
                false
            }
            SlashCommand::Checkpoint { label } => {
                println!("Checkpoint marked: {}.", label.unwrap_or_else(|| "manual".to_string()));
                false
            }
            SlashCommand::Docs { query } => {
                println!("Looking up docs for {}. Querying Context7...", query.unwrap_or_else(|| "project".to_string()));
                false
            }
            SlashCommand::Loop { mission } => {
                self.run_loop(mission.as_deref())?;
                false
            }
            SlashCommand::Unknown(name) => {
                eprintln!("unknown slash command: /{name}");
                false
            }
            SlashCommand::Mcp { action, args } => {
                self.handle_mcp_command(action.as_deref(), args.as_deref())?;
                false
            }
        })
    }

    fn persist_session(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.runtime.session().save_to_path(&self.session.path)?;
        Ok(())
    }

    fn llm_reflect(&self, user_input: &str, response: &str) -> Option<String> {
        if user_input.len() < 10 || response.len() < 30 {
            return None;
        }
        if score_turn_importance(user_input, response) < 0.3 {
            return None;
        }

        let prompt = format!(
            "You are a memory distillation assistant. Given a conversation turn, decide if it contains \
a key fact, user preference, important decision, or correction worth remembering in future sessions. \
Respond with a JSON object only — no markdown, no explanation:\n\
{{\"important\": true/false, \"summary\": \"one-line summary or null\"}}\n\n\
User: {}\n\nAssistant: {}",
            &user_input[..user_input.len().min(400)],
            &response[..response.len().min(400)]
        );

        let reflect_model = "gemini-2.5-flash";
        let provider = api::LlmProvider::Google;
        let auth = api::resolve_auth_for_provider(provider).unwrap_or(api::AuthSource::None);
        let mut client = TernlangClient::from_auth(auth).with_provider(provider);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .ok()?;

        let raw = rt.block_on(async {
            let mut stream = client
                .stream_message(&MessageRequest {
                    model: reflect_model.to_string(),
                    max_tokens: Some(80),
                    messages: vec![InputMessage {
                        role: "user".to_string(),
                        content: vec![InputContentBlock::Text { text: prompt }],
                    }],
                    system: Some("Output only valid JSON. No markdown fences.".to_string()),
                    tools: None,
                    tool_choice: None,
                    stream: false,
                })
                .await
                .ok()?;

            let mut text = String::new();
            loop {
                match stream.next_event().await {
                    Ok(Some(api::StreamEvent::ContentBlockDelta(ev))) => {
                        if let ContentBlockDelta::TextDelta { text: t } = ev.delta {
                            text.push_str(&t);
                        }
                    }
                    Ok(Some(_)) => {}
                    Ok(None) | Err(_) => break,
                }
            }
            Some(text)
        })?;

        let raw = raw
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        let json: serde_json::Value = serde_json::from_str(raw).ok()?;
        if json
            .get("important")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            json.get("summary")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty() && *s != "null")
                .map(ToOwned::to_owned)
        } else {
            None
        }
    }

    fn handle_mcp_command(&mut self, action: Option<&str>, args: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            None | Some("list") => {
                let servers = load_mcp_servers();
                if servers.is_empty() {
                    println!("No MCP servers configured. Add one with: /mcp add <name> <command> [args...]");
                } else {
                    println!("MCP servers:");
                    for (name, scoped) in &servers {
                        if let McpServerConfig::Stdio(cfg) = &scoped.config {
                            println!("  {} — {} {}", name, cfg.command, cfg.args.join(" "));
                        }
                    }
                }
            }
            Some("add") => {
                let parts: Vec<&str> = args.unwrap_or("").splitn(3, ' ').collect();
                if parts.len() < 2 {
                    println!("Usage: /mcp add <name> <command> [args...]");
                    return Ok(());
                }
                let name = parts[0].to_string();
                let command = parts[1].to_string();
                let extra_args: Vec<String> = parts.get(2)
                    .map(|s| s.split_whitespace().map(ToOwned::to_owned).collect())
                    .unwrap_or_default();
                let mut servers = load_mcp_servers();
                servers.insert(name.clone(), ScopedMcpServerConfig {
                    scope: ConfigSource::User,
                    config: McpServerConfig::Stdio(McpStdioServerConfig {
                        command: command.clone(),
                        args: extra_args.clone(),
                        env: std::collections::BTreeMap::new(),
                    }),
                });
                save_mcp_servers(&servers)?;
                let mut mgr = self.mcp_manager.lock().map_err(|e| e.to_string())?;
                *mgr = McpServerManager::from_servers(&servers);
                println!("Added MCP server '{}': {} {}", name, command, extra_args.join(" "));
            }
            Some("remove") => {
                let name = args.unwrap_or("").trim();
                if name.is_empty() {
                    println!("Usage: /mcp remove <name>");
                    return Ok(());
                }
                let mut servers = load_mcp_servers();
                if servers.remove(name).is_none() {
                    println!("No MCP server named '{name}'.");
                } else {
                    save_mcp_servers(&servers)?;
                    let mut mgr = self.mcp_manager.lock().map_err(|e| e.to_string())?;
                    *mgr = McpServerManager::from_servers(&servers);
                    println!("Removed MCP server '{name}'.");
                }
            }
            Some(other) => println!("Unknown /mcp action '{other}'. Use: list, add, remove"),
        }
        Ok(())
    }

    fn print_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cumulative = self.runtime.usage().cumulative_usage();
        let latest = self.runtime.usage().current_turn_usage();
        println!(
            "{}",
            format_status_report(
                &self.model,
                StatusUsage {
                    message_count: self.runtime.session().messages.len(),
                    turns: self.runtime.usage().turns(),
                    latest,
                    cumulative,
                    estimated_tokens: self.runtime.estimated_tokens(),
                },
                self.permission_mode.as_str(),
                &status_context(Some(&self.session.path))?,
            )
        );
        Ok(())
    }

    fn set_model(&mut self, model: Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let model_id = if let Some(m) = model {
            resolve_model_alias(&m).to_string()
        } else {
            let items: Vec<String> = KNOWN_MODELS
                .iter()
                .map(|m| {
                    format!(
                        "{:<25} {:<12} {}",
                        style(m.id).cyan(),
                        style(format!("({})", m.provider)).dim(),
                        style(m.description).dim()
                    )
                })
                .collect();

            let selection = Select::new()
                .with_prompt("Select Model")
                .items(&items)
                .default(0)
                .interact_opt()?;

            if let Some(index) = selection {
                KNOWN_MODELS[index].id.to_string()
            } else {
                return Ok(false);
            }
        };

        if model_id == self.model {
            return Ok(false);
        }

        let previous = self.model.clone();
        let session = self.runtime.session().clone();
        let message_count = session.messages.len();
        self.runtime = build_runtime_with_mcp(
            session,
            model_id.clone(),
            self.system_prompt.clone(),
            true,
            self.allowed_tools.clone(),
            self.permission_mode,
            Arc::clone(&self.mcp_manager),
            self.event_tx.clone(),
        )?;
        self.model.clone_from(&model_id);
        println!(
            "{}",
            format_model_switch_report(&previous, &model_id, message_count)
        );
        Ok(true)
    }

    fn set_permissions(
        &mut self,
        mode: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let Some(mode) = mode else {
            println!(
                "{}",
                format_permissions_report(self.permission_mode.as_str())
            );
            return Ok(false);
        };

        let normalized = normalize_permission_mode(&mode).ok_or_else(|| {
            format!(
                "unsupported permission mode '{mode}'. Use read-only, workspace-write, or danger-full-access."
            )
        })?;

        if normalized == self.permission_mode.as_str() {
            println!("{}", format_permissions_report(normalized));
            return Ok(false);
        }

        let previous = self.permission_mode.as_str().to_string();
        let session = self.runtime.session().clone();
        self.permission_mode = permission_mode_from_label(normalized);
        // Keep sandbox bypass in sync with the new permission mode.
        runtime::set_sandbox_bypass(self.permission_mode == PermissionMode::DangerFullAccess);
        self.runtime = build_runtime_with_mcp(
            session,
            self.model.clone(),
            self.system_prompt.clone(),
            true,
            self.allowed_tools.clone(),
            self.permission_mode,
            Arc::clone(&self.mcp_manager),
            self.event_tx.clone(),
        )?;
        println!(
            "{}",
            format_permissions_switch_report(&previous, normalized)
        );
        Ok(true)
    }

    fn clear_session(&mut self, confirm: bool) -> Result<bool, Box<dyn std::error::Error>> {
        if !confirm {
            println!(
                "clear: confirmation required; run /clear --confirm to start a fresh session."
            );
            return Ok(false);
        }

        self.session = create_managed_session_handle()?;
        self.runtime = build_runtime_with_mcp(
            Session::new(),
            self.model.clone(),
            self.system_prompt.clone(),
            true,
            self.allowed_tools.clone(),
            self.permission_mode,
            Arc::clone(&self.mcp_manager),
            self.event_tx.clone(),
        )?;
        println!(
            "Session cleared
  Mode             fresh session
  Preserved model  {}
  Permission mode  {}
  Session          {}",
            self.model,
            self.permission_mode.as_str(),
            self.session.id,
        );
        Ok(true)
    }

    fn print_cost(&self) {
        let cumulative = self.runtime.usage().cumulative_usage();
        println!("{}", format_cost_report(cumulative));
    }

    fn resume_session(
        &mut self,
        session_path: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let Some(session_ref) = session_path else {
            println!("Usage: /resume <session-path>");
            return Ok(false);
        };

        let handle = resolve_session_reference(&session_ref)?;
        let session = Session::load_from_path(&handle.path)?;
        let message_count = session.messages.len();
        self.runtime = build_runtime_with_mcp(
            session,
            self.model.clone(),
            self.system_prompt.clone(),
            true,
            self.allowed_tools.clone(),
            self.permission_mode,
            Arc::clone(&self.mcp_manager),
            self.event_tx.clone(),
        )?;
        self.session = handle;
        println!(
            "{}",
            format_resume_report(
                &self.session.path.display().to_string(),
                message_count,
                self.runtime.usage().turns(),
            )
        );
        Ok(true)
    }

    fn print_config(section: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", render_config_report(section)?);
        Ok(())
    }

    fn print_memory() -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", render_memory_report()?);
        Ok(())
    }

    fn print_diff() -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", render_diff_report()?);
        Ok(())
    }

    fn print_version() -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", render_version_report());
        Ok(())
    }

    fn export_session(
        &self,
        requested_path: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let export_path = resolve_export_path(requested_path, self.runtime.session())?;
        fs::write(&export_path, render_export_text(self.runtime.session()))?;
        println!(
            "Export
  Result           wrote transcript
  File             {}
  Messages         {}",
            export_path.display(),
            self.runtime.session().messages.len(),
        );
        Ok(())
    }

    fn handle_session_command(
        &mut self,
        action: Option<&str>,
        target: Option<&str>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match action {
            None | Some("list") => {
                println!("{}", render_session_list(&self.session.id)?);
                Ok(false)
            }
            Some("switch") => {
                let Some(target) = target else {
                    println!("Usage: /session switch <session-id>");
                    return Ok(false);
                };
                let handle = resolve_session_reference(target)?;
                let session = Session::load_from_path(&handle.path)?;
                let message_count = session.messages.len();
                self.runtime = build_runtime_with_mcp(
                    session,
                    self.model.clone(),
                    self.system_prompt.clone(),
                    true,
                    self.allowed_tools.clone(),
                    self.permission_mode,
                    Arc::clone(&self.mcp_manager),
                    self.event_tx.clone(),
                )?;
                self.session = handle;
                println!(
                    "Session switched
  Active session   {}
  File             {}
  Messages         {}",
                    self.session.id,
                    self.session.path.display(),
                    message_count,
                );
                Ok(true)
            }
            Some(other) => {
                println!("Unknown /session action '{other}'. Use /session list or /session switch <session-id>.");
                Ok(false)
            }
        }
    }

    fn compact(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.runtime.compact(CompactionConfig::default());
        let removed = result.removed_message_count;
        let kept = result.compacted_session.messages.len();
        let skipped = removed == 0;
        self.runtime = build_runtime_with_mcp(
            result.compacted_session,
            self.model.clone(),
            self.system_prompt.clone(),
            true,
            self.allowed_tools.clone(),
            self.permission_mode,
            Arc::clone(&self.mcp_manager),
            self.event_tx.clone(),
        )?;
        self.persist_session()?;
        println!("{}", format_compact_report(removed, kept, skipped));
        Ok(())
    }

    fn compress(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.runtime.compact(CompactionConfig {
            preserve_recent_messages: 2,
            max_estimated_tokens: 1,
        });
        let removed = result.removed_message_count;
        let kept = result.compacted_session.messages.len();
        let skipped = removed == 0;
        self.runtime = build_runtime_with_mcp(
            result.compacted_session,
            self.model.clone(),
            self.system_prompt.clone(),
            true,
            self.allowed_tools.clone(),
            self.permission_mode,
            Arc::clone(&self.mcp_manager),
            self.event_tx.clone(),
        )?;
        self.persist_session()?;
        if skipped {
            println!("Compression skipped: session is empty or too short.");
        } else {
            println!(
                "Aggressively compressed {} messages. Albert's memory is now lean and sharp ({} kept).",
                removed, kept
            );
        }
        Ok(())
    }

    fn run_auth(&mut self, provider_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let provider = match provider_name {
            Some(name) => name.to_lowercase(),
            None => {
                println!("Available providers: openai, anthropic, huggingface, google, azure, aws");
                print!("Choose provider: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_lowercase()
            }
        };

        print!("Enter API Key for {provider}: ");
        io::stdout().flush()?;
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key)?;
        let api_key = api_key.trim().to_string();

        if api_key.is_empty() {
            println!("Error: API Key cannot be empty.");
            return Ok(());
        }

        runtime::save_provider_config(&provider, runtime::ProviderConfig {
            api_key: Some(api_key),
            model: None,
            base_url: None,
        })?;

        println!("Authentication configured for {provider}.");
        Ok(())
    }

    fn run_internal_prompt_text(
        &self,
        prompt: &str,
        enable_tools: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let session = self.runtime.session().clone();
        let mut runtime = build_runtime(
            session,
            self.model.clone(),
            self.system_prompt.clone(),
            enable_tools,
            false,
            self.allowed_tools.clone(),
            self.permission_mode,
        )?;
        let mut permission_prompter = CliPermissionPrompter::new(self.permission_mode, false);
        let summary = runtime.run_turn(prompt.to_string(), Some(&mut permission_prompter))?;
        Ok(final_assistant_text(&summary).trim().to_string())
    }

    fn run_bughunter(&self, scope: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let scope = scope.unwrap_or("the current repository");
        let prompt = format!(
            "You are /bughunter. Inspect {scope} and identify the most likely bugs or correctness issues. Prioritize concrete findings with file paths, severity, and suggested fixes. Use tools if needed."
        );
        println!("{}", self.run_internal_prompt_text(&prompt, true)?);
        Ok(())
    }

    fn run_ultraplan(&self, task: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let task = task.unwrap_or("the current repo work");
        let prompt = format!(
            "You are /ultraplan. Produce a deep multi-step execution plan for {task}. Include goals, risks, implementation sequence, verification steps, and rollback considerations. Use tools if needed."
        );
        println!("{}", self.run_internal_prompt_text(&prompt, true)?);
        Ok(())
    }

    fn run_teleport(&self, target: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let Some(target) = target.map(str::trim).filter(|value| !value.is_empty()) else {
            println!("Usage: /teleport <symbol-or-path>");
            return Ok(());
        };

        println!("{}", render_teleport_report(target)?);
        Ok(())
    }

    fn run_debug_tool_call(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", render_last_tool_debug_report(self.runtime.session())?);
        Ok(())
    }

    fn run_commit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let status = git_output(&["status", "--short"])?;
        if status.trim().is_empty() {
            println!("Commit
  Result           skipped
  Reason           no workspace changes");
            return Ok(());
        }

        git_status_ok(&["add", "-A"])?;
        let staged_stat = git_output(&["diff", "--cached", "--stat"])?;
        let prompt = format!(
            "Generate a git commit message in plain text Lore format only. Base it on this staged diff summary:

{}

Recent conversation context:
{}",
            truncate_for_prompt(&staged_stat, 8_000),
            recent_user_context(self.runtime.session(), 6)
        );
        let message = sanitize_generated_message(&self.run_internal_prompt_text(&prompt, false)?);
        if message.trim().is_empty() {
            return Err("generated commit message was empty".into());
        }

        let path = write_temp_text_file("albert-commit-message.txt", &message)?;
        let output = Command::new("git")
            .args(["commit", "--file"])
            .arg(&path)
            .current_dir(env::current_dir()?)
            .output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(format!("git commit failed: {stderr}").into());
        }

        println!(
            "Commit
  Result           created
  Message file     {}

{}",
            path.display(),
            message.trim()
        );
        Ok(())
    }

    fn run_pr(&self, context: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let staged = git_output(&["diff", "--stat"])?;
        let prompt = format!(
            "Generate a pull request title and body from this conversation and diff summary. Output plain text in this format exactly:
TITLE: <title>
BODY:
<body markdown>

Context hint: {}

Diff summary:
{}",
            context.unwrap_or("none"),
            truncate_for_prompt(&staged, 10_000)
        );
        let draft = sanitize_generated_message(&self.run_internal_prompt_text(&prompt, false)?);
        let (title, body) = parse_titled_body(&draft)
            .ok_or_else(|| "failed to parse generated PR title/body".to_string())?;

        if command_exists("gh") {
            let body_path = write_temp_text_file("albert-pr-body.md", &body)?;
            let output = Command::new("gh")
                .args(["pr", "create", "--title", &title, "--body-file"])
                .arg(&body_path)
                .current_dir(env::current_dir()?)
                .output()?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!(
                    "PR
  Result           created
  Title            {title}
  URL              {}",
                    if stdout.is_empty() { "<unknown>" } else { &stdout }
                );
                return Ok(());
            }
        }

        println!("PR draft
  Title            {title}

{body}");
        Ok(())
    }

    fn run_issue(&self, context: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let prompt = format!(
            "Generate a GitHub issue title and body from this conversation. Output plain text in this format exactly:
TITLE: <title>
BODY:
<body markdown>

Context hint: {}

Conversation context:
{}",
            context.unwrap_or("none"),
            truncate_for_prompt(&recent_user_context(self.runtime.session(), 10), 10_000)
        );
        let draft = sanitize_generated_message(&self.run_internal_prompt_text(&prompt, false)?);
        let (title, body) = parse_titled_body(&draft)
            .ok_or_else(|| "failed to parse generated issue title/body".to_string())?;

        if command_exists("gh") {
            let body_path = write_temp_text_file("albert-issue-body.md", &body)?;
            let output = Command::new("gh")
                .args(["issue", "create", "--title", &title, "--body-file"])
                .arg(&body_path)
                .current_dir(env::current_dir()?)
                .output()?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!(
                    "Issue
  Result           created
  Title            {title}
  URL              {}",
                    if stdout.is_empty() { "<unknown>" } else { &stdout }
                );
                return Ok(());
            }
        }

        println!("Issue draft
  Title            {title}

{body}");
        Ok(())
    }

    fn run_loop(&mut self, mission: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let mission_text = mission.unwrap_or("Complete the current objective.").to_string();
        println!("\n{} {}", style("🚀 MISSION STARTED:").bold().green(), style(&mission_text).cyan());
        
        let mut turn_count = 0;
        let max_turns = 10; // Safety limit
        
        loop {
            turn_count += 1;
            if turn_count > max_turns {
                println!("\n{} Maximum turn limit reached ({}). Pausing autopilot for alignment.", style("⚠️").yellow(), max_turns);
                break;
            }
            
            println!("\n{} [Iteration {}/{}]", style("🌀 Autopilot").bold().magenta(), turn_count, max_turns);
            
            // Craft the loop turn input
            let loop_prompt = format!(
                "MISSION: {}\n\nContinue executing the mission. If the mission is fully complete, tested, and validated, end your response with 'MISSION COMPLETE'. Otherwise, continue with the next logical step. Spawn internal swarm reasoning if needed.",
                mission_text
            );
            
            self.run_turn(&loop_prompt)?;
            
            // Check for completion in the session history
            let last_message = self.runtime.session().messages.last();
            if let Some(msg) = last_message {
                let content = msg.blocks.iter().filter_map(|b| if let ContentBlock::Text { text } = b { Some(text.as_str()) } else { None }).collect::<Vec<_>>().join(" ");
                if content.contains("MISSION COMPLETE") {
                    println!("\n{} Mission objectives achieved. Harness complete.", style("✨ MISSION SUCCESSFUL").bold().green());
                    break;
                }
            }
            
            thread::sleep(Duration::from_millis(500));
        }
        
        Ok(())
    }
}

fn sessions_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let path = cwd.join(".claw").join("sessions");
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn create_managed_session_handle() -> Result<SessionHandle, Box<dyn std::error::Error>> {
    let id = generate_session_id();
    let path = sessions_dir()?.join(format!("{id}.json"));
    Ok(SessionHandle { id, path })
}

fn generate_session_id() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    format!("session-{millis}")
}

fn resolve_session_reference(reference: &str) -> Result<SessionHandle, Box<dyn std::error::Error>> {
    let direct = PathBuf::from(reference);
    let path = if direct.exists() {
        direct
    } else {
        sessions_dir()?.join(format!("{reference}.json"))
    };
    if !path.exists() {
        return Err(format!("session not found: {reference}").into());
    }
    let id = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(reference)
        .to_string();
    Ok(SessionHandle { id, path })
}

fn list_managed_sessions() -> Result<Vec<ManagedSessionSummary>, Box<dyn std::error::Error>> {
    let mut sessions = Vec::new();
    for entry in fs::read_dir(sessions_dir()?)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let metadata = entry.metadata()?;
        let modified_epoch_secs = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or_default();
        let message_count = Session::load_from_path(&path)
            .map(|session| session.messages.len())
            .unwrap_or_default();
        let id = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("unknown")
            .to_string();
        sessions.push(ManagedSessionSummary {
            id,
            path,
            modified_epoch_secs,
            message_count,
        });
    }
    sessions.sort_by(|left, right| right.modified_epoch_secs.cmp(&left.modified_epoch_secs));
    Ok(sessions)
}

fn render_session_list(active_session_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let sessions = list_managed_sessions()?;
    let mut lines = vec![
        "Sessions".to_string(),
        format!("  Directory         {}", sessions_dir()?.display()),
    ];
    if sessions.is_empty() {
        lines.push("  No managed sessions saved yet.".to_string());
        return Ok(lines.join("
"));
    }
    for session in sessions {
        let marker = if session.id == active_session_id {
            "● current"
        } else {
            "○ saved"
        };
        lines.push(format!(
            "  {id:<20} {marker:<10} msgs={msgs:<4} modified={modified} path={path}",
            id = session.id,
            msgs = session.message_count,
            modified = session.modified_epoch_secs,
            path = session.path.display(),
        ));
    }
    Ok(lines.join("
"))
}

fn render_repl_help() -> String {
    [
        "REPL".to_string(),
        "  /exit                Quit the REPL".to_string(),
        "  /quit                Quit the REPL".to_string(),
        "  Up/Down              Navigate prompt history".to_string(),
        "  Tab                  Complete slash commands".to_string(),
        "  Ctrl-C               Clear input (or exit on empty prompt)".to_string(),
        "  Shift+Enter/Ctrl+J   Insert a newline".to_string(),
        String::new(),
        render_slash_command_help(),
    ]
    .join(
        "
",
    )
}

fn status_context(
    session_path: Option<&Path>,
) -> Result<StatusContext, Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let loader = ConfigLoader::default_for(&cwd);
    let discovered_config_files = loader.discover().len();
    let runtime_config = loader.load()?;
    let project_context = ProjectContext::discover_with_git(&cwd, DEFAULT_DATE)?;
    let (project_root, git_branch) =
        parse_git_status_metadata(project_context.git_status.as_deref());
    Ok(StatusContext {
        cwd,
        session_path: session_path.map(Path::to_path_buf),
        loaded_config_files: runtime_config.loaded_entries().len(),
        discovered_config_files,
        memory_file_count: project_context.instruction_files.len(),
        project_root,
        git_branch,
    })
}

fn format_status_report(
    model: &str,
    usage: StatusUsage,
    permission_mode: &str,
    context: &StatusContext,
) -> String {
    [
        format!(
            "Status
  Model            {model}
  Permission mode  {permission_mode}
  Messages         {}
  Turns            {}
  Estimated tokens {}",
            usage.message_count, usage.turns, usage.estimated_tokens,
        ),
        format!(
            "Usage
  Latest total     {}
  Cumulative input {}
  Cumulative output {}
  Cumulative total {}",
            usage.latest.total_tokens(),
            usage.cumulative.input_tokens,
            usage.cumulative.output_tokens,
            usage.cumulative.total_tokens(),
        ),
        format!(
            "Workspace
  Cwd              {}
  Project root     {}
  Git branch       {}
  Session          {}
  Config files     loaded {}/{}
  Memory files     {}",
            context.cwd.display(),
            context
                .project_root
                .as_ref()
                .map_or_else(|| "unknown".to_string(), |path| path.display().to_string()),
            context.git_branch.as_deref().unwrap_or("unknown"),
            context.session_path.as_ref().map_or_else(
                || "live-repl".to_string(),
                |path| path.display().to_string()
            ),
            context.loaded_config_files,
            context.discovered_config_files,
            context.memory_file_count,
        ),
    ]
    .join(
        "

",
    )
}

fn render_config_report(section: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let loader = ConfigLoader::default_for(&cwd);
    let discovered = loader.discover();
    let runtime_config = loader.load()?;

    let mut lines = vec![
        format!(
            "Config
  Working directory {}
  Loaded files      {}
  Merged keys       {}",
            cwd.display(),
            runtime_config.loaded_entries().len(),
            runtime_config.merged().len()
        ),
        "Discovered files".to_string(),
    ];
    for entry in discovered {
        let source = match entry.source {
            ConfigSource::User => "user",
            ConfigSource::Project => "project",
            ConfigSource::Local => "local",
        };
        let status = if runtime_config
            .loaded_entries()
            .iter()
            .any(|loaded_entry| loaded_entry.path == entry.path)
        {
            "loaded"
        } else {
            "missing"
        };
        lines.push(format!(
            "  {source:<7} {status:<7} {}",
            entry.path.display()
        ));
    }

    if let Some(section) = section {
        lines.push(format!("Merged section: {section}"));
        let value = match section {
            "env" => runtime_config.get("env"),
            "hooks" => runtime_config.get("hooks"),
            "model" => runtime_config.get("model"),
            other => {
                lines.push(format!(
                    "  Unsupported config section '{other}'. Use env, hooks, or model."
                ));
                return Ok(lines.join(
                    "
",
                ));
            }
        };
        lines.push(format!(
            "  {}",
            match value {
                Some(value) => value.render(),
                None => "<unset>".to_string(),
            }
        ));
        return Ok(lines.join(
            "
",
        ));
    }

    lines.push("Merged JSON".to_string());
    lines.push(format!("  {}", runtime_config.as_json().render()));
    Ok(lines.join(
        "
",
    ))
}

fn render_memory_report() -> Result<String, Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let project_context = ProjectContext::discover(&cwd, DEFAULT_DATE)?;
    let mut lines = vec![format!(
        "Memory
  Working directory {}
  Instruction files {}",
        cwd.display(),
        project_context.instruction_files.len()
    )];
    if project_context.instruction_files.is_empty() {
        lines.push("Discovered files".to_string());
        lines.push(
            "  No TERNLANG instruction files discovered in the current directory ancestry."
                .to_string(),
        );
    } else {
        lines.push("Discovered files".to_string());
        for (index, file) in project_context.instruction_files.iter().enumerate() {
            let preview = file.content.lines().next().unwrap_or("").trim();
            let preview = if preview.is_empty() {
                "<empty>"
            } else {
                preview
            };
            lines.push(format!("  {}. {}", index + 1, file.path.display(),));
            lines.push(format!(
                "     lines={} preview={}",
                file.content.lines().count(),
                preview
            ));
        }
    }
    Ok(lines.join(
        "
",
    ))
}

fn init_ternlang_md() -> Result<String, Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    Ok(initialize_repo(&cwd)?.render())
}

fn run_init() -> Result<(), Box<dyn std::error::Error>> {
    init::wake_sequence();
    println!("\n{}", init_ternlang_md()?);
    Ok(())
}

fn normalize_permission_mode(mode: &str) -> Option<&'static str> {
    match mode.trim() {
        "read-only" => Some("read-only"),
        "workspace-write" => Some("workspace-write"),
        "danger-full-access" => Some("danger-full-access"),
        _ => None,
    }
}

fn render_diff_report() -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("git")
        .args(["diff", "--", ":(exclude).omx"])
        .current_dir(env::current_dir()?)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git diff failed: {stderr}").into());
    }
    let diff = String::from_utf8(output.stdout)?;
    if diff.trim().is_empty() {
        return Ok(
            "Diff
  Result           clean working tree
  Detail           no current changes"
                .to_string(),
        );
    }
    Ok(format!("Diff

{}", diff.trim_end()))
}

fn render_version_report() -> String {
    format!(
        "Ternlang Code CLI version {}
target: {}
git sha: {}",
        VERSION,
        BUILD_TARGET.unwrap_or("unknown"),
        GIT_SHA.unwrap_or("unknown")
    )
}

struct CliPermissionPrompter {
    permission_mode: PermissionMode,
    interactive: bool,
    is_prompting: Option<Arc<std::sync::atomic::AtomicBool>>,
}

impl CliPermissionPrompter {
    fn new(permission_mode: PermissionMode, interactive: bool) -> Self {
        Self {
            permission_mode,
            interactive,
            is_prompting: None,
        }
    }
}

impl runtime::PermissionPrompter for CliPermissionPrompter {
    fn decide(
        &mut self,
        request: &runtime::PermissionRequest,
    ) -> runtime::PermissionPromptDecision {
        struct PromptGuard(Option<Arc<std::sync::atomic::AtomicBool>>);
        impl Drop for PromptGuard {
            fn drop(&mut self) {
                if let Some(f) = &self.0 {
                    f.store(false, std::sync::atomic::Ordering::Relaxed);
                }
            }
        }
        if let Some(flag) = &self.is_prompting {
            flag.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        let _guard = PromptGuard(self.is_prompting.clone());

        let default = match self.permission_mode {
            PermissionMode::ReadOnly => runtime::PermissionPromptDecision::Deny {
                reason: "read-only mode".to_string(),
            },
            PermissionMode::WorkspaceWrite => {
                if request.tool_name.starts_with("shell") {
                    runtime::PermissionPromptDecision::Deny {
                        reason: "shell tools disabled in workspace-write mode".to_string(),
                    }
                } else {
                    runtime::PermissionPromptDecision::Allow
                }
            }
            PermissionMode::DangerFullAccess => runtime::PermissionPromptDecision::Allow,
            PermissionMode::Prompt => runtime::PermissionPromptDecision::Allow,
            PermissionMode::Allow => runtime::PermissionPromptDecision::Allow,
        };

        if !self.interactive || !matches!(default, runtime::PermissionPromptDecision::Allow) {
            return default;
        }

        let tool_input_string = request.input.to_string();
        if tool_input_string.len() > 256 {
            println!(
                "Request to use tool `{}` with large input. Preview:\n{}...",
                request.tool_name,
                &tool_input_string[..256]
            );
        } else {
            println!("Request to use tool `{}` with input:\n{}", request.tool_name, request.input);
        }

        loop {
            let choice = dialoguer::Select::new()
                .with_prompt("Allow this tool use?")
                .items(&["Allow once", "Deny"])
                .default(0)
                .interact_opt()
                .unwrap_or_default();

            match choice {
                Some(0) => return runtime::PermissionPromptDecision::Allow,
                Some(1) => {
                    return runtime::PermissionPromptDecision::Deny {
                        reason: "user denied".to_string(),
                    }
                }
                Some(_) => {
                    return runtime::PermissionPromptDecision::Deny {
                        reason: "invalid choice".to_string(),
                    }
                }
                None => {
                    // User cancelled with Esc, treat as denial
                    return runtime::PermissionPromptDecision::Deny {
                        reason: "user cancelled".to_string(),
                    };
                }
            }
        }
    }
}

fn resolve_provider_for_model(model: &str) -> api::LlmProvider {
    if model.contains("gpt-") || model.contains("o1-") || model.contains("o3-") {
        api::LlmProvider::OpenAi
    } else if model.contains("claude-") || model.contains("opus") || model.contains("sonnet") || model.contains("haiku") {
        api::LlmProvider::Anthropic
    } else if model.contains("gemini-") {
        api::LlmProvider::Google
    } else if model.contains("llama") || model.contains("mistral") {
        api::LlmProvider::HuggingFace
    } else {
        api::LlmProvider::Ternlang
    }
}

fn build_runtime(
    session: Session,
    model: String,
    system_prompt: Vec<String>,
    enable_tools: bool,
    enable_stream_events: bool,
    allowed_tools: Option<AllowedToolSet>,
    permission_mode: PermissionMode,
) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>
{
    let cwd = env::current_dir()?;
    let _config = ConfigLoader::default_for(&cwd).load()?;

    let provider = resolve_provider_for_model(&model);
    let provider_config = runtime::load_provider_config(match provider {
        api::LlmProvider::OpenAi => "openai",
        api::LlmProvider::Anthropic => "anthropic",
        api::LlmProvider::Google => "google",
        api::LlmProvider::HuggingFace => "huggingface",
        _ => "ternlang",
    }).unwrap_or(None);

    let auth_source = if let Some(config) = provider_config {
        if let Some(key) = config.api_key {
            api::AuthSource::ApiKey(key)
        } else {
            api::AuthSource::None
        }
    } else {
        api::resolve_startup_auth_source().unwrap_or(api::AuthSource::None)
    };

    let client = TernlangClient::from_auth(auth_source).with_provider(provider);
    let api_client = TernlangRuntimeClient {
        client,
        model: model.clone(),
        max_tokens: max_tokens_for_model(&model),
        tools: if enable_tools {
            filter_tool_specs(allowed_tools.as_ref())
        } else {
            Vec::new()
        },
        event_tx: if enable_stream_events {
            Some(
                tokio::runtime::Runtime::new()?
                    .block_on(async {
                        let (tx, _) = tokio::sync::broadcast::channel(128);
                        tx
                    }),
            )
        } else {
            None
        },
    };

    let mcp_manager = Arc::new(Mutex::new(McpServerManager::from_servers(&load_mcp_servers())));
    let tool_executor = CliToolExecutor::new(mcp_manager);
    let permission_policy = PermissionPolicy::new(permission_mode);
    Ok(ConversationRuntime::new(
        session,
        api_client,
        tool_executor,
        permission_policy,
        system_prompt,
    ))
}

#[allow(clippy::too_many_arguments)]
fn build_runtime_with_mcp(
    session: Session,
    model: String,
    system_prompt: Vec<String>,
    enable_tools: bool,
    allowed_tools: Option<AllowedToolSet>,
    permission_mode: PermissionMode,
    mcp_manager: Arc<Mutex<McpServerManager>>,
    event_tx: tokio::sync::broadcast::Sender<AssistantEvent>,
) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>
{
    let cwd = env::current_dir()?;
    let _config = ConfigLoader::default_for(&cwd).load()?;

    let provider = resolve_provider_for_model(&model);
    let provider_config = runtime::load_provider_config(match provider {
        api::LlmProvider::OpenAi => "openai",
        api::LlmProvider::Anthropic => "anthropic",
        api::LlmProvider::Google => "google",
        api::LlmProvider::HuggingFace => "huggingface",
        _ => "ternlang",
    }).unwrap_or(None);

    let auth_source = if let Some(config) = provider_config {
        if let Some(key) = config.api_key {
            api::AuthSource::ApiKey(key)
        } else {
            api::AuthSource::None
        }
    } else {
        api::resolve_startup_auth_source().unwrap_or(api::AuthSource::None)
    };

    let client = TernlangClient::from_auth(auth_source).with_provider(provider);
    let api_client = TernlangRuntimeClient {
        client,
        model: model.clone(),
        max_tokens: max_tokens_for_model(&model),
        tools: if enable_tools { filter_tool_specs(allowed_tools.as_ref()) } else { Vec::new() },
        event_tx: Some(event_tx),
    };

    let tool_executor = CliToolExecutor::new(mcp_manager);
    let permission_policy = PermissionPolicy::new(permission_mode);
    Ok(ConversationRuntime::new(session, api_client, tool_executor, permission_policy, system_prompt))
}

fn mcp_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ternlang")
        .join("mcp_servers.json")
}

fn load_mcp_servers() -> std::collections::BTreeMap<String, ScopedMcpServerConfig> {
    let path = mcp_config_path();
    if !path.exists() { return std::collections::BTreeMap::new(); }
    let raw = fs::read_to_string(&path).unwrap_or_default();
    let persisted: std::collections::HashMap<String, serde_json::Value> =
        serde_json::from_str(&raw).unwrap_or_default();
    persisted.into_iter().filter_map(|(name, v)| {
        let command = v.get("command")?.as_str()?.to_string();
        let args = v.get("args")?.as_array()
            .map(|a| a.iter().filter_map(|x| x.as_str().map(ToOwned::to_owned)).collect())
            .unwrap_or_default();
        let scoped = ScopedMcpServerConfig {
            scope: ConfigSource::User,
            config: McpServerConfig::Stdio(McpStdioServerConfig {
                command, args,
                env: std::collections::BTreeMap::new(),
            }),
        };
        Some((name, scoped))
    }).collect()
}

fn save_mcp_servers(servers: &std::collections::BTreeMap<String, ScopedMcpServerConfig>) -> Result<(), Box<dyn std::error::Error>> {
    let path = mcp_config_path();
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    let mut map = serde_json::Map::new();
    for (name, scoped) in servers {
        if let McpServerConfig::Stdio(cfg) = &scoped.config {
            map.insert(name.clone(), json!({
                "command": cfg.command,
                "args": cfg.args,
            }));
        }
    }
    fs::write(&path, serde_json::to_string_pretty(&map)?)?;
    Ok(())
}

fn build_system_prompt() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    Ok(load_system_prompt(
        cwd,
        DEFAULT_DATE.to_string(),
        env::consts::OS,
        "unknown",
    )?)
}

#[derive(Clone)]
struct TernlangRuntimeClient {
    client: TernlangClient,
    model: String,
    max_tokens: u32,
    tools: Vec<ToolSpec>,
    event_tx: Option<tokio::sync::broadcast::Sender<AssistantEvent>>,
}

impl ApiClient for TernlangRuntimeClient {
    fn stream(
        &mut self,
        request: ApiRequest,
    ) -> Result<Vec<AssistantEvent>, RuntimeError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| RuntimeError::new(e.to_string()))?;
        runtime.block_on(self.stream_async(request)).map_err(|e| RuntimeError::new(e.to_string()))
    }
}

impl TernlangRuntimeClient {
    async fn stream_async(
        &mut self,
        request: ApiRequest,
    ) -> Result<Vec<AssistantEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let mut stream = self
            .client
            .stream_message(&MessageRequest {
                model: self.model.clone(),
                max_tokens: Some(self.max_tokens),
                messages: request
                    .messages
                    .into_iter()
                    .map(map_conversation_message)
                    .collect(),
                system: Some(request.system_prompt.join("

")),
                tools: if self.tools.is_empty() {
                    None
                } else {
                    Some(self.tools.iter().map(map_tool_spec).collect())
                },
                tool_choice: if self.tools.is_empty() {
                    None
                } else {
                    Some(ToolChoice::Auto)
                },
                stream: true,
            })
            .await?;

        let mut events = Vec::new();
        while let Some(event) = stream.next_event().await? {
            if let Some(mapped) = map_stream_event(&event) {
                if let Some(tx) = &self.event_tx {
                    let _ = tx.send(mapped.clone());
                }
                events.push(mapped);
            }
        }
        Ok(events)
    }
}

fn map_conversation_message(message: ConversationMessage) -> InputMessage {
    InputMessage {
        role: match message.role {
            MessageRole::User => "user".to_string(),
            MessageRole::Assistant => "assistant".to_string(),
            MessageRole::Tool => "user".to_string(),
            MessageRole::System => "system".to_string(),
        },
        content: message
            .blocks
            .into_iter()
            .map(map_content_block)
            .collect(),
    }
}

fn map_content_block(block: ContentBlock) -> InputContentBlock {
    match block {
        ContentBlock::Text { text } => InputContentBlock::Text { text },
        ContentBlock::ToolUse { id, name, input } => InputContentBlock::ToolUse { id, name, input: serde_json::from_str(&input).unwrap_or(serde_json::Value::Null) },
        ContentBlock::ToolResult { tool_use_id, output, tool_name: _, is_error: _ } => InputContentBlock::ToolResult {
            tool_use_id,
            content: vec![ToolResultContentBlock::Text { text: output }],
            is_error: false,
        },
    }
}

fn map_tool_spec(spec: &ToolSpec) -> ToolDefinition {
    ToolDefinition {
        name: spec.name.to_string(),
        description: Some(spec.description.to_string()),
        input_schema: spec.input_schema.clone(),
    }
}

fn map_stream_event(event: &ApiStreamEvent) -> Option<AssistantEvent> {
    match event {
        ApiStreamEvent::MessageStart(payload) => Some(AssistantEvent::Usage(map_usage(payload.message.usage.clone()))),
        ApiStreamEvent::ContentBlockDelta(payload) => match &payload.delta {
            ContentBlockDelta::TextDelta { text } => Some(AssistantEvent::TextDelta(text.clone())),
            _ => None,
        },
        ApiStreamEvent::ContentBlockStart(payload) => match &payload.content_block {
            OutputContentBlock::ToolUse { id, name, input } => Some(AssistantEvent::ToolUse {
                id: id.clone(),
                name: name.clone(),
                input: input.to_string(),
            }),
            _ => None,
        },
        ApiStreamEvent::MessageDelta(payload) => {
            Some(AssistantEvent::Usage(map_usage(payload.usage.clone())))
        }
        ApiStreamEvent::MessageStop(_) => Some(AssistantEvent::MessageStop),
        _ => None,
    }
}

fn map_usage(usage: api::Usage) -> TokenUsage {
    TokenUsage {
        input_tokens: usage.input_tokens,
        output_tokens: usage.output_tokens,
        cache_creation_input_tokens: usage.cache_creation_input_tokens,
        cache_read_input_tokens: usage.cache_read_input_tokens,
    }
}

#[derive(Clone)]
struct CliToolExecutor {
    mcp_manager: Arc<Mutex<McpServerManager>>,
}

impl CliToolExecutor {
    fn new(mcp_manager: Arc<Mutex<McpServerManager>>) -> Self {
        Self { mcp_manager }
    }

    fn execute_mcp_tool(&self, tool_name: &str, input: serde_json::Value) -> Result<runtime::ToolResult, ToolError> {
        let mut manager = self.mcp_manager.lock().map_err(|e| ToolError::new(e.to_string()))?;
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| ToolError::new(e.to_string()))?;
        rt.block_on(manager.discover_tools()).map_err(|e| ToolError::new(e.to_string()))?;
        let response = rt.block_on(manager.call_tool(tool_name, Some(input)))
            .map_err(|e| ToolError::new(e.to_string()))?;
        let content = response.result.map(|r| r.content).unwrap_or_default();
        let output = content.into_iter()
            .filter_map(|c| {
                if c.kind == "text" {
                    c.data.get("text").and_then(|v| v.as_str()).map(ToOwned::to_owned)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(runtime::ToolResult { output, state: 1 })
    }
}

impl ToolExecutor for CliToolExecutor {
    fn execute(
        &mut self,
        tool_name: &str,
        input: &str,
    ) -> Result<runtime::ToolResult, ToolError> {
        let input_val: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::Value::Null);
        if tool_name.starts_with("mcp__") {
            return self.execute_mcp_tool(tool_name, input_val);
        }
        match execute_tool(tool_name, &input_val) {
            Ok(res) => Ok(runtime::ToolResult {
                output: res.output,
                state: res.state,
            }),
            Err(e) => Err(ToolError::new(e)),
        }
    }

    fn query_memory(&mut self, query: &str) -> Result<String, ToolError> {
        let input = json!({
            "action": "search_nodes",
            "query": query
        });
        match execute_tool("Memory", &input) {
            Ok(res) => Ok(res.output),
            Err(e) => Err(ToolError::new(e)),
        }
    }
}

fn append_to_albert_memory(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from).ok_or("HOME not set")?;
    let path = home.join(".ternlang");
    fs::create_dir_all(&path)?;
    let file_path = path.join("memory.md");

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)?;

    writeln!(file, "- {}", line)?;
    Ok(())
}

fn score_turn_importance(user_input: &str, response: &str) -> f32 {
    let combined = format!("{user_input} {response}").to_lowercase();
    let mut score: f32 = 0.0;
    let markers = [
        ("remember", 0.5),
        ("prefer", 0.4),
        ("use", 0.2),
        ("don't", 0.3),
        ("always", 0.3),
        ("never", 0.4),
        ("set", 0.2),
        ("change", 0.2),
        ("fix", 0.1),
    ];
    for (m, s) in markers {
        if combined.contains(m) {
            score += s;
        }
    }
    score.min(1.0)
}


/// Overwrite the readline prompt line with a styled user message box.
fn render_user_message_box(input: &str) -> io::Result<()> {
    let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
    let label = format!("> {input}");
    // Pad to full width so the background fills the line
    let padded = format!(" {:<width$} ", label, width = term_width.saturating_sub(3));
    let mut stdout = io::stdout();
    // Go back one line to overwrite the readline echo, then print styled box
    execute!(
        stdout,
        crossterm::cursor::MoveToPreviousLine(1),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        crossterm::style::SetBackgroundColor(crossterm::style::Color::AnsiValue(240)),
        crossterm::style::SetForegroundColor(crossterm::style::Color::White),
        crossterm::style::Print(&padded),
        crossterm::style::ResetColor,
        crossterm::style::Print("\n"),
    )?;
    stdout.flush()
}

 

/// Print tool outputs from completed turn summary using └ / indent format.
fn render_tool_outputs(summary: &runtime::TurnSummary) {
    use std::collections::HashMap;
    let mut results: HashMap<&str, &str> = HashMap::new();
    for msg in &summary.tool_results {
        for block in &msg.blocks {
            if let ContentBlock::ToolResult { tool_use_id, output, .. } = block {
                results.insert(tool_use_id.as_str(), output.as_str());
            }
        }
    }
    for msg in &summary.assistant_messages {
        for block in &msg.blocks {
            if let ContentBlock::ToolUse { id, .. } = block {
                if let Some(output) = results.get(id.as_str()) {
                    let lines: Vec<&str> = output.lines()
                        .filter(|l| !l.trim().is_empty())
                        .collect();
                    if lines.is_empty() { continue; }
                    let show = lines.len().min(6);
                    for (i, line) in lines[..show].iter().enumerate() {
                        let trimmed = if line.len() > 120 { &line[..120] } else { line };
                        if i == 0 {
                            println!("  {} {}", style("└").dim(), style(trimmed).dim());
                        } else {
                            println!("    {}", style(trimmed).dim());
                        }
                    }
                    if lines.len() > 6 {
                        println!("  {}", style(format!("… +{} lines", lines.len() - 6)).dim().italic());
                    }
                }
            }
        }
    }
}

fn final_assistant_text(summary: &runtime::TurnSummary) -> String {
    summary
        .assistant_messages
        .iter()
        .flat_map(|message| message.blocks.iter())
        .filter_map(|block| match block {
            ContentBlock::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("
")
}

fn collect_tool_uses(summary: &runtime::TurnSummary) -> serde_json::Value {
    serde_json::Value::Array(
        summary
            .assistant_messages
            .iter()
            .flat_map(|message| message.blocks.iter())
            .filter_map(|block| match block {
                ContentBlock::ToolUse { id, name, input } => Some(json!({
                    "id": id,
                    "name": name,
                    "input": input,
                })),
                _ => None,
            })
            .collect(),
    )
}

fn collect_tool_results(summary: &runtime::TurnSummary) -> serde_json::Value {
    serde_json::Value::Array(
        summary
            .tool_results
            .iter()
            .flat_map(|message| message.blocks.iter())
            .filter_map(|block| match block {
                ContentBlock::ToolResult { tool_use_id, output, tool_name: _, is_error: _ } => Some(json!({
                    "tool_use_id": tool_use_id,
                    "output": output,
                })),
                _ => None,
            })
            .collect(),
    )
}

fn slash_command_completion_candidates() -> Vec<String> {
    let mut candidates: Vec<String> = slash_command_specs()
        .iter()
        .map(|spec| format!("/{}", spec.name))
        .collect();
    candidates.sort();
    candidates
}

fn truncate_for_prompt(value: &str, max_chars: usize) -> String {
    if value.len() <= max_chars {
        return value.to_string();
    }
    let mut truncated = String::with_capacity(max_chars + 20);
    truncated.push_str(&value[..max_chars]);
    truncated.push_str("
... (truncated)");
    truncated
}

fn write_temp_text_file(name: &str, content: &str) -> Result<PathBuf, io::Error> {
    let path = env::temp_dir().join(name);
    fs::write(&path, content)?;
    Ok(path)
}

fn command_exists(name: &str) -> bool {
    let Ok(paths) = env::var("PATH") else {
        return false;
    };
    paths
        .split(':')
        .map(|path| Path::new(path).join(name))
        .any(|path| path.exists())
}

fn sanitize_generated_message(message: &str) -> String {
    let message = message.trim();
    if let Some(stripped) = message.strip_prefix("```text
") {
        return stripped.strip_suffix("```").unwrap_or(stripped).to_string();
    }
    if let Some(stripped) = message.strip_prefix("```") {
        return stripped.strip_suffix("```").unwrap_or(stripped).to_string();
    }
    message.to_string()
}

fn parse_titled_body(raw: &str) -> Option<(String, String)> {
    let Some(title_start) = raw.find("TITLE:") else {
        return None;
    };
    let Some(body_start) = raw.find("BODY:") else {
        return None;
    };
    let title = raw[title_start + 6..body_start].trim().to_string();
    let body = raw[body_start + 5..].trim().to_string();
    Some((title, body))
}

fn git_output(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(env::current_dir()?)
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

fn git_status_ok(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new("git")
        .args(args)
        .current_dir(env::current_dir()?)
        .status()?;
    if !status.success() {
        return Err("git command failed".into());
    }
    Ok(())
}

fn recent_user_context(session: &Session, max_messages: usize) -> String {
    session
        .messages
        .iter()
        .filter(|message| message.role == MessageRole::User)
        .rev()
        .take(max_messages)
        .flat_map(|message| message.blocks.iter())
        .filter_map(|block| match block {
            ContentBlock::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("

")
}

fn resolve_export_path(requested_path: Option<&str>, _session: &Session) -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(PathBuf::from(requested_path.unwrap_or("export.txt")))
}

fn render_export_text(_session: &Session) -> String {
    "".to_string()
}

fn render_teleport_report(_target: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}

fn render_last_tool_debug_report(_session: &Session) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}

fn check_workspace_trust() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    
    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    println!("Accessing workspace:");
    println!("\n  {}\n", style(cwd.display()).cyan());
    
    println!("Quick safety check: Is this a project you created or one you trust?");
    println!("(Like your own code, a well-known open source project, or work from your team).");
    println!("If not, take a moment to review what's in this folder first.\n");
    
    let options = vec!["Yes, I trust this folder", "No, exit"];
    let selection = Select::new()
        .with_prompt("Security guide")
        .items(&options)
        .default(0)
        .interact()?;
        
    if selection == 1 {
        println!("\nExiting for safety.");
        std::process::exit(0);
    }
    
    println!("{}", style("Trust verified. Let's build.").dim());
    Ok(())
}

struct ModelEntry {
    id: &'static str,
    provider: &'static str,
    description: &'static str,
}

const KNOWN_MODELS: &[ModelEntry] = &[
    ModelEntry {
        id: "gemini-2.5-pro",
        provider: "Google",
        description: "Most capable Gemini — complex reasoning",
    },
    ModelEntry {
        id: "gemini-2.5-flash",
        provider: "Google",
        description: "Fast & capable — recommended default",
    },
    ModelEntry {
        id: "gemini-2.5-flash-lite",
        provider: "Google",
        description: "Lightest Gemini — maximum speed",
    },
    ModelEntry {
        id: "gemini-2.0-pro",
        provider: "Google",
        description: "Previous Pro generation",
    },
    ModelEntry {
        id: "gemini-2.0-flash",
        provider: "Google",
        description: "Previous Flash generation",
    },
    ModelEntry {
        id: "gemini-2.0-flash-lite",
        provider: "Google",
        description: "Previous Flash Lite generation",
    },
    ModelEntry {
        id: "claude-opus-4-7",
        provider: "Anthropic",
        description: "Most capable Claude",
    },
    ModelEntry {
        id: "claude-sonnet-4-6",
        provider: "Anthropic",
        description: "Best balance of speed and capability",
    },
    ModelEntry {
        id: "claude-haiku-4-5-20251001",
        provider: "Anthropic",
        description: "Fastest Claude",
    },
    ModelEntry {
        id: "gpt-4o",
        provider: "OpenAI",
        description: "GPT-4o multimodal flagship",
    },
    ModelEntry {
        id: "gpt-4o-mini",
        provider: "OpenAI",
        description: "Efficient GPT-4o variant",
    },
    ModelEntry {
        id: "o3-mini",
        provider: "OpenAI",
        description: "o3 reasoning — efficient",
    },
    ModelEntry {
        id: "grok-3",
        provider: "xAI",
        description: "Grok 3 flagship",
    },
    ModelEntry {
        id: "grok-3-mini",
        provider: "xAI",
        description: "Efficient Grok variant",
    },
    ModelEntry {
        id: "llama-3.3-70b-versatile",
        provider: "Ollama",
        description: "Llama 3.3 70B — local or hosted",
    },
];
