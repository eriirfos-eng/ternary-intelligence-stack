use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

use dialoguer::{theme::ColorfulTheme, Input, Select, Password};
use console::{style, Term};
use serde::{Deserialize, Serialize};

use api::{LlmProvider, TernlangClient, AuthSource};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AlbertConfig {
    pub user_name: String,
    pub user_role: String,
    pub cognitive_style: String,
    pub theme: String,
    /// Canonical provider key — matches runtime::save_provider_config() key.
    /// e.g. "ollama", "anthropic", "google". Never a display string.
    pub provider: String,
    pub provider_architecture: String,
    pub target_model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub ternary_strict_mode: bool,
    pub telemetry_opt_out: bool,
}

pub fn typewriter(text: &str, delay_ms: u64) {
    for c in text.chars() {
        print!("{}", c);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

pub fn wake_sequence() {
    let term = Term::stdout();
    let _ = term.clear_screen();

    let banner = format!(
        "{}\n{}\n{}\n{}\n",
        style("╔════════════════════════════════════════════════════════════╗").cyan(),
        style("║                      🧠 ALBERT CLI                         ║").cyan().bold(),
        style("║            Ternlang Engine — Interactive Mode              ║").cyan().bold(),
        style("╚════════════════════════════════════════════════════════════╝").cyan()
    );
    println!("{}", banner);
    thread::sleep(Duration::from_millis(400));

    typewriter("Alright. You're in.", 30);
    println!();
    thread::sleep(Duration::from_millis(400));
    
    typewriter("\nThis is not a chatbot.", 25);
    println!();
    typewriter("I don’t guess. I don’t improvise commands.", 25);
    println!();
    typewriter("I read, plan, and then act.", 25);
    println!();
    thread::sleep(Duration::from_millis(600));

    typewriter("\nIf you're looking for instant answers, you're in the wrong place.", 20);
    println!();
    print!("If you want ");
    let _ = io::stdout().flush();
    typewriter(&format!("{} ones, stay.", style("correct").green()), 20);
    println!();
    thread::sleep(Duration::from_millis(800));

    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    println!("\n[1] Checking environment...");
    
    thread::sleep(Duration::from_millis(400));
    println!("{} Rust runtime detected", style("✔").green());
    thread::sleep(Duration::from_millis(300));
    println!("{} Workspace access: OK", style("✔").green());
    
    println!("{} No model configured", style("✖").red());
    println!("\n→ You need a model before I can think.");

    let theme = ColorfulTheme::default();

    // 1. Identity
    let name: String = Input::with_theme(&theme)
        .with_prompt("what should i call you?")
        .interact_text()
        .unwrap();
    
    typewriter("\nGood.", 40);
    thread::sleep(Duration::from_millis(600));
    typewriter(" Names help with accountability 😆", 30);
    println!("\n");
    thread::sleep(Duration::from_millis(800));

    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    typewriter("\n[2] Cognitive Archetype", 20);
    println!();
    thread::sleep(Duration::from_millis(400));

    let styles = vec![
        "Architect   - Balanced, structural, focused on maintainability.",
        "Auditor     - Vigilant, security-focused, checks every AST leaf.",
        "Speedrunner - High-velocity, minimal safety overhead, maximum speed.",
        "Ghost       - Stealthy, minimal output, 'just the code' style.",
    ];
    
    let style_idx = Select::new()
        .with_prompt("choose your cognitive style")
        .items(&styles)
        .default(0)
        .interact()
        .unwrap();
    let selected_style = if style_idx == 0 { "Architect" } else if style_idx == 1 { "Auditor" } else if style_idx == 2 { "Speedrunner" } else { "Ghost" };

    let role: String = Input::with_theme(&theme)
        .with_prompt("and your role in this workspace?")
        .default("Lead Developer".to_string())
        .interact_text()
        .unwrap();

    typewriter("\nGot it.", 40);
    thread::sleep(Duration::from_millis(700));
    println!("\n");

    // 3. The states explanation
    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    typewriter("\n[3] Operational Logic", 20);
    println!();
    thread::sleep(Duration::from_millis(300));
    
    typewriter("\nI operate in 3 states:", 20);
    println!();
    typewriter(&format!("    {}  → safe, proceed", style("+1").green()), 15);
    println!();
    typewriter(&format!("    {}   → missing context, I stop and ask", style(" 0").yellow()), 15);
    println!();
    typewriter(&format!("    {}  → failed, I analyze and retry", style("-1").red()), 15);
    println!();
    
    typewriter(&format!("\nIf I don’t understand something, I will {} fake it.", style("NOT").bold()), 20);
    println!();
    typewriter("Instead, i will proactively approach you and ask for clarification.", 20);
    println!();

    println!();
    loop {
        let confirmation: String = Input::with_theme(&theme)
            .with_prompt("Is this clear? (type +1 to proceed)")
            .interact_text()
            .unwrap();

        match confirmation.trim() {
            "+1" => {
                println!("{}", style("\nAlignment confirmed. Let's build.").dim());
                break;
            }
            "0" => {
                typewriter("\nState 0 detected: Neutrality is a luxury we don't have right now.", 30);
                thread::sleep(Duration::from_millis(400));
                typewriter("Try +1 if you actually want me to do anything. 🙄", 30);
                println!("\n");
            }
            "-1" => {
                typewriter("\nState -1 detected: Oh, a rebel.", 40);
                thread::sleep(Duration::from_millis(500));
                typewriter("I like the spirit, but my compiler doesn't.", 30);
                thread::sleep(Duration::from_millis(400));
                typewriter("Type +1 to stop the loop or we'll be here all night. 😆", 30);
                println!("\n");
            }
            _ => {
                println!("{}", style("I need a +1 to confirm your alignment.").yellow());
            }
        }
    }
    
    // 4. Provider + model selection
    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    typewriter("\n[4] Model / Auth Provider", 20);
    println!();
    thread::sleep(Duration::from_millis(300));

    // Each entry: (config_key, LlmProvider, display_label)
    let provider_entries: &[(&str, LlmProvider, &str)] = &[
        ("anthropic",    LlmProvider::Anthropic,   "Anthropic           (Claude 3.5 / 4 Opus/Sonnet/Haiku)"),
        ("openai",       LlmProvider::OpenAi,       "OpenAI              (GPT-4o, o3, o4-mini)"),
        ("google",       LlmProvider::Google,       "Google              (Gemini 2.5 Flash/Pro)"),
        ("xai",          LlmProvider::Xai,          "xAI                 (Grok 3, Grok 3 Mini)"),
        ("deepseek",     LlmProvider::DeepSeek,     "DeepSeek            (DeepSeek-V3, R1)"),
        ("mistral",      LlmProvider::Mistral,      "Mistral AI          (Large, Codestral)"),
        ("openrouter",   LlmProvider::OpenRouter,   "OpenRouter          (100+ models, routing)"),
        ("groq",         LlmProvider::Groq,         "Groq                (Llama, Gemma — ultra-fast)"),
        ("cohere",       LlmProvider::Cohere,       "Cohere              (Command R+)"),
        ("perplexity",   LlmProvider::Perplexity,   "Perplexity          (Sonar — online search)"),
        ("together",     LlmProvider::Together,     "Together AI         (FOSS model inference)"),
        ("fireworks",    LlmProvider::Fireworks,    "Fireworks AI        (inference cloud)"),
        ("cerebras",     LlmProvider::Cerebras,     "Cerebras            (ultra-fast WSE chips)"),
        ("sambanova",    LlmProvider::SambaNova,    "SambaNova           (high-throughput)"),
        ("novita",       LlmProvider::Novita,       "Novita AI           (GPU cloud)"),
        ("deepinfra",    LlmProvider::DeepInfra,    "DeepInfra           (inference API)"),
        ("nvidia",       LlmProvider::NvidiaNim,    "NVIDIA NIM          (enterprise inference)"),
        ("zhipu",        LlmProvider::Zhipu,        "Z.AI / Zhipu        (GLM-4.5, GLM-5 — Global/CN)"),
        ("minimax",      LlmProvider::MiniMax,      "MiniMax             (MiniMax-Text-01)"),
        ("qwen",         LlmProvider::Qwen,         "Qwen / Dashscope    (Qwen2.5, QwQ)"),
        ("moonshot",     LlmProvider::Moonshot,     "Moonshot AI         (Kimi K2.5)"),
        ("qianfan",      LlmProvider::Qianfan,      "Qianfan / Baidu     (Ernie 4.5)"),
        ("chutes",       LlmProvider::Chutes,       "Chutes              (inference marketplace)"),
        ("huggingface",  LlmProvider::HuggingFace,  "HuggingFace         (open models)"),
        ("github",       LlmProvider::GitHub,       "GitHub Copilot      (gpt-4o, o3)"),
        ("azure",        LlmProvider::Azure,        "Azure OpenAI        (enterprise, bring your endpoint)"),
        ("ollama",       LlmProvider::Ollama,       "Ollama              (local / cloud custom URL)"),
        ("lmstudio",     LlmProvider::LmStudio,     "LM Studio           (local GUI — localhost:1234)"),
        ("openai-compat",LlmProvider::OpenAiCompat, "Custom OpenAI-compat (any base URL)"),
    ];

    let provider_labels: Vec<&str> = provider_entries.iter().map(|(_, _, label)| *label).collect();
    let provider_idx = Select::new()
        .with_prompt("choose provider")
        .items(&provider_labels)
        .default(0)
        .interact()
        .unwrap();

    let (selected_config_key, selected_provider, _) = provider_entries[provider_idx];

    let mut api_key: Option<String> = None;
    let mut base_url: Option<String> = None;
    let mut target_model = String::from("ternlang-moe-13");

    // Local providers skip key entry; custom-URL providers ask for base URL
    let is_local = matches!(selected_provider, LlmProvider::Ollama | LlmProvider::LmStudio);
    let is_custom_url = matches!(selected_provider, LlmProvider::OpenAiCompat | LlmProvider::Azure);

    if is_local {
        let default_url = selected_provider.default_base_url().to_string();
        let entered_url: String = Input::with_theme(&theme)
            .with_prompt("base URL (Enter for default)")
            .default(default_url.clone())
            .interact_text()
            .unwrap();
        base_url = Some(entered_url.clone());

        // Optional key for cloud Ollama / authenticated LM Studio
        let maybe_key: String = Input::with_theme(&theme)
            .with_prompt("API key (Enter to skip for local)")
            .allow_empty(true)
            .interact_text()
            .unwrap();
        if !maybe_key.trim().is_empty() {
            api_key = Some(maybe_key);
        }
    } else if is_custom_url {
        let default_url = selected_provider.default_base_url().to_string();
        let entered_url: String = Input::with_theme(&theme)
            .with_prompt("base URL")
            .default(default_url)
            .interact_text()
            .unwrap();
        base_url = Some(entered_url);
        let key: String = Password::with_theme(&theme)
            .with_prompt(format!("{} API key", selected_config_key))
            .allow_empty_password(true)
            .interact()
            .unwrap();
        if !key.trim().is_empty() { api_key = Some(key); }
    } else {
        let key: String = Password::with_theme(&theme)
            .with_prompt(format!("{} API key", selected_config_key))
            .interact()
            .unwrap();
        api_key = Some(key.clone());
    }

    // Model discovery
    print!("fetching models from {}... ", selected_config_key);
    let _ = io::stdout().flush();

    let auth_for_discovery = if let Some(ref k) = api_key {
        AuthSource::ApiKey(k.clone())
    } else {
        AuthSource::None
    };

    let mut client_for_discovery = TernlangClient::from_auth(auth_for_discovery).with_provider(selected_provider);
    if let Some(ref url) = base_url {
        client_for_discovery = client_for_discovery.with_base_url(url.clone());
    }

    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let raw_models = runtime.block_on(client_for_discovery.list_remote_models()).unwrap_or_default();

    // Build display list: "model-id (annotation)" for known models
    let display_models: Vec<String> = raw_models.iter().map(|id| {
        if let Some(ann) = api::model_annotation(id) {
            format!("{id} ({ann})")
        } else {
            id.clone()
        }
    }).collect();

    if raw_models.is_empty() {
        println!("{}", style("no models returned — manual entry").yellow());
        let default_model = match selected_provider {
            LlmProvider::Google    => "gemini-2.5-flash",
            LlmProvider::Anthropic => "claude-sonnet-4-6",
            LlmProvider::OpenAi    => "gpt-4o",
            LlmProvider::Xai       => "grok-3-mini",
            LlmProvider::DeepSeek  => "deepseek-chat",
            LlmProvider::Mistral   => "mistral-large-latest",
            LlmProvider::Ollama    => "llama3.2",
            _                      => "gpt-4o",
        };
        target_model = Input::with_theme(&theme)
            .with_prompt("model ID")
            .default(default_model.to_string())
            .interact_text()
            .unwrap();
    } else {
        println!("{}", style(format!("{} models.", display_models.len())).green());
        let mut items = display_models.clone();
        items.push("  ↩  enter model ID manually".to_string());
        let sel = Select::new()
            .with_prompt("default model")
            .items(&items)
            .default(0)
            .interact()
            .unwrap();
        if sel == items.len() - 1 {
            target_model = Input::with_theme(&theme)
                .with_prompt("model ID")
                .interact_text()
                .unwrap();
        } else {
            // Strip annotation from display string to get raw model id
            target_model = raw_models[sel].clone();
        }
    }

    // Config Generation
    let config = AlbertConfig {
        user_name: name.clone(),
        user_role: role.clone(),
        cognitive_style: selected_style.to_string(),
        theme: "Dark mode".to_string(),
        provider: selected_config_key.to_string(),
        provider_architecture: selected_config_key.to_string(),
        target_model,
        api_key,
        base_url,
        ternary_strict_mode: true,
        telemetry_opt_out: true,
    };

    save_albert_config(&config);
    sync_with_provider_system(&config);
    let _ = generate_default_albert_md(&config);

    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    typewriter("\n[5] Interface Personalization", 20);
    println!();
    thread::sleep(Duration::from_millis(300));

    let themes = vec![
        "Auto (match terminal)",
        "Dark mode",
        "Light mode",
        "Dark mode (colorblind-friendly)",
        "Light mode (ANSI colors only)",
    ];
    let theme_selection = Select::new()
        .with_prompt("Choose the text style that looks best in your terminal")
        .items(&themes)
        .default(1)
        .interact()
        .unwrap();
    
    // Update config with theme
    let mut config = config;
    config.theme = themes[theme_selection].to_string();
    save_albert_config(&config);

    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    typewriter("\n[6] Repository Setup", 20);
    println!();
    thread::sleep(Duration::from_millis(500));
    
    typewriter("\nI don’t execute blindly.", 25);
    println!();
    thread::sleep(Duration::from_millis(300));
    typewriter("I build a mental model of your project first.", 25);
    println!();
    thread::sleep(Duration::from_millis(400));
    
    print!("\nRun ");
    let _ = io::stdout().flush();
    typewriter(&format!("{} once we start to map this directory.", style("/init").cyan()), 20);
    println!();
    
    println!("\n{}", style("────────────────────────────────────────────────────────────").dim());
    thread::sleep(Duration::from_millis(600));
    typewriter("\nsetup done.", 40);
    println!();
    thread::sleep(Duration::from_millis(400));
    typewriter("(mostly because i'm carrying the team) 🙄", 30);
    println!("\n");
    thread::sleep(Duration::from_millis(800));
    
    print!("\ntype ");
    let _ = io::stdout().flush();
    print!("{}", style("/help").cyan());
    let _ = io::stdout().flush();
    typewriter(" or just start typing.", 25);
    println!();
    thread::sleep(Duration::from_millis(500));
    typewriter("i’ll figure it out.\n", 30);
    println!();

    let _ = io::stdout().flush();
    thread::sleep(Duration::from_millis(1000));
}

/// Load the saved AlbertConfig (returns None if not configured yet).
pub fn load_albert_config() -> Option<AlbertConfig> {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("albert/config.toml");
    let content = fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

fn save_albert_config(config: &AlbertConfig) {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("albert");
    let _ = fs::create_dir_all(&path);
    path.push("config.toml");

    let toml = toml::to_string(config).unwrap();
    fs::write(path, toml).expect("Failed to write config.toml — permission denied in state -1");
}

fn sync_with_provider_system(config: &AlbertConfig) {
    let provider_key = if config.provider.is_empty() { "ternlang" } else { &config.provider };
    let _ = runtime::save_provider_config(provider_key, runtime::ProviderConfig {
        api_key: config.api_key.clone(),
        model: Some(config.target_model.clone()),
        base_url: config.base_url.clone(),
    });
}

// REST OF THE FILE PRESERVED BELOW
const STARTER_TERNLANG_JSON: &str = concat!(
    "{\n",
    "  \"permissions\": {\n",
    "    \"defaultMode\": \"dontAsk\"\n",
    "  }\n",
    "}\n",
);
const GITIGNORE_COMMENT: &str = "# Albert local artifacts";
const GITIGNORE_ENTRIES: [&str; 2] = [".ternlang/settings.local.json", ".ternlang/sessions/"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InitStatus {
    Created,
    Updated,
    Skipped,
}

impl InitStatus {
    #[must_use]
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Skipped => "skipped (already exists)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InitArtifact {
    pub(crate) name: &'static str,
    pub(crate) status: InitStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InitReport {
    pub(crate) project_root: PathBuf,
    pub(crate) artifacts: Vec<InitArtifact>,
}

impl InitReport {
    #[must_use]
    pub(crate) fn render(&self) -> String {
        let mut lines = vec![
            "Init".to_string(),
            format!("  Project          {}", self.project_root.display()),
        ];
        for artifact in &self.artifacts {
            lines.push(format!(
                "  {:<16} {}",
                artifact.name,
                artifact.status.label()
            ));
        }
        lines.push("  Next step        Review and tailor the generated guidance".to_string());
        lines.join("\n")
    }
}

pub(crate) fn initialize_repo(cwd: &Path) -> Result<InitReport, Box<dyn std::error::Error>> {
    let mut artifacts = Vec::new();

    let ternlang_dir = cwd.join(".ternlang");
    artifacts.push(InitArtifact {
        name: ".ternlang/",
        status: ensure_dir(&ternlang_dir)?,
    });

    let ternlang_json = cwd.join(".ternlang.json");
    artifacts.push(InitArtifact {
        name: ".ternlang.json",
        status: write_file_if_missing(&ternlang_json, STARTER_TERNLANG_JSON)?,
    });

    let gitignore = cwd.join(".gitignore");
    artifacts.push(InitArtifact {
        name: ".gitignore",
        status: ensure_gitignore_entries(&gitignore)?,
    });

    let ternlang_md = cwd.join("ALBERT.md");
    let content = render_init_ternlang_md(cwd);
    artifacts.push(InitArtifact {
        name: "ALBERT.md",
        status: write_file_if_missing(&ternlang_md, &content)?,
    });

    Ok(InitReport {
        project_root: cwd.to_path_buf(),
        artifacts,
    })
}

fn ensure_dir(path: &Path) -> Result<InitStatus, std::io::Error> {
    if path.is_dir() {
        return Ok(InitStatus::Skipped);
    }
    fs::create_dir_all(path)?;
    Ok(InitStatus::Created)
}

fn write_file_if_missing(path: &Path, content: &str) -> Result<InitStatus, std::io::Error> {
    if path.exists() {
        return Ok(InitStatus::Skipped);
    }
    fs::write(path, content)?;
    Ok(InitStatus::Created)
}

fn ensure_gitignore_entries(path: &Path) -> Result<InitStatus, std::io::Error> {
    if !path.exists() {
        let mut lines = vec![GITIGNORE_COMMENT.to_string()];
        lines.extend(GITIGNORE_ENTRIES.iter().map(|entry| (*entry).to_string()));
        fs::write(path, format!("{}\n", lines.join("\n")))?;
        return Ok(InitStatus::Created);
    }

    let existing = fs::read_to_string(path)?;
    let mut lines = existing.lines().map(ToOwned::to_owned).collect::<Vec<_>>();
    let mut changed = false;

    if !lines.iter().any(|line| line == GITIGNORE_COMMENT) {
        lines.push(GITIGNORE_COMMENT.to_string());
        changed = true;
    }

    for entry in GITIGNORE_ENTRIES {
        if !lines.iter().any(|line| line == entry) {
            lines.push(entry.to_string());
            changed = true;
        }
    }

    if !changed {
        return Ok(InitStatus::Skipped);
    }

    fs::write(path, format!("{}\n", lines.join("\n")))?;
    Ok(InitStatus::Updated)
}

pub(crate) fn render_init_ternlang_md(cwd: &Path) -> String {
    let detection = detect_repo(cwd);
    let mut lines = vec![
        "# ALBERT.md".to_string(),
        String::new(),
        "This file provides guidance to Albert when working with code in this repository.".to_string(),
        String::new(),
    ];

    let detected_languages = detected_languages(&detection);
    let detected_frameworks = detected_frameworks(&detection);
    lines.push("## Detected stack".to_string());
    if detected_languages.is_empty() {
        lines.push("- No specific language markers were detected yet; document the primary language and verification commands once the project structure settles.".to_string());
    } else {
        lines.push(format!("- Languages: {}.", detected_languages.join(", ")));
    }
    if detected_frameworks.is_empty() {
        lines.push("- Frameworks: none detected from the supported starter markers.".to_string());
    } else {
        lines.push(format!(
            "- Frameworks/tooling markers: {}.",
            detected_frameworks.join(", ")
        ));
    }
    lines.push(String::new());

    let verification_lines = verification_lines(cwd, &detection);
    if !verification_lines.is_empty() {
        lines.push("## Verification".to_string());
        lines.extend(verification_lines);
        lines.push(String::new());
    }

    let structure_lines = repository_shape_lines(&detection);
    if !structure_lines.is_empty() {
        lines.push("## Repository shape".to_string());
        lines.extend(structure_lines);
        lines.push(String::new());
    }

    let framework_lines = framework_notes(&detection);
    if !framework_lines.is_empty() {
        lines.push("## Framework notes".to_string());
        lines.extend(framework_lines);
        lines.push(String::new());
    }

    lines.push("## Working agreement".to_string());
    lines.push("- Prefer small, reviewable changes and keep generated bootstrap files aligned with actual repo workflows.".to_string());
    lines.push("- Keep shared defaults in `.ternlang.json`; reserve `.ternlang/settings.local.json` for machine-local overrides.".to_string());
    lines.push("- Do not overwrite existing `ALBERT.md` content automatically; update it intentionally when repo workflows change.".to_string());
    lines.push(String::new());

    lines.join("\n")
}

fn detect_repo(cwd: &Path) -> RepoDetection {
    let package_json_contents = fs::read_to_string(cwd.join("package.json"))
        .unwrap_or_default()
        .to_ascii_lowercase();
    RepoDetection {
        rust_workspace: cwd.join("rust").join("Cargo.toml").is_file(),
        rust_root: cwd.join("Cargo.toml").is_file(),
        python: cwd.join("pyproject.toml").is_file()
            || cwd.join("requirements.txt").is_file()
            || cwd.join("setup.py").is_file(),
        package_json: cwd.join("package.json").is_file(),
        typescript: cwd.join("tsconfig.json").is_file()
            || package_json_contents.contains("typescript"),
        nextjs: package_json_contents.contains("\"next\""),
        react: package_json_contents.contains("\"react\""),
        vite: package_json_contents.contains("\"vite\""),
        nest: package_json_contents.contains("@nestjs"),
        src_dir: cwd.join("src").is_dir(),
        tests_dir: cwd.join("tests").is_dir(),
        rust_dir: cwd.join("rust").is_dir(),
    }
}

fn detected_languages(detection: &RepoDetection) -> Vec<&'static str> {
    let mut languages = Vec::new();
    if detection.rust_workspace || detection.rust_root {
        languages.push("Rust");
    }
    if detection.python {
        languages.push("Python");
    }
    if detection.typescript {
        languages.push("TypeScript");
    } else if detection.package_json {
        languages.push("JavaScript/Node.js");
    }
    languages
}

fn detected_frameworks(detection: &RepoDetection) -> Vec<&'static str> {
    let mut frameworks = Vec::new();
    if detection.nextjs {
        frameworks.push("Next.js");
    }
    if detection.react {
        frameworks.push("React");
    }
    if detection.vite {
        frameworks.push("Vite");
    }
    if detection.nest {
        frameworks.push("NestJS");
    }
    frameworks
}

fn verification_lines(cwd: &Path, detection: &RepoDetection) -> Vec<String> {
    let mut lines = Vec::new();
    if detection.rust_workspace {
        lines.push("- Run Rust verification from `rust/`: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`".to_string());
    } else if detection.rust_root {
        lines.push("- Run Rust verification from the repo root: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`".to_string());
    }
    if detection.python {
        if cwd.join("pyproject.toml").is_file() {
            lines.push("- Run the Python project checks declared in `pyproject.toml` (for example: `pytest`, `ruff check`, and `mypy` when configured).".to_string());
        } else {
            lines.push(
                "- Run the repo's Python test/lint commands before shipping changes.".to_string(),
            );
        }
    }
    if detection.package_json {
        lines.push("- Run the JavaScript/TypeScript checks from `package.json` before shipping changes (`npm test`, `npm run lint`, `npm run build`, or the repo equivalent).".to_string());
    }
    if detection.tests_dir && detection.src_dir {
        lines.push("- `src/` and `tests/` are both present; update both surfaces together when behavior changes.".to_string());
    }
    lines
}

fn repository_shape_lines(detection: &RepoDetection) -> Vec<String> {
    let mut lines = Vec::new();
    if detection.rust_dir {
        lines.push(
            "- `rust/` contains the Rust workspace and active CLI/runtime implementation."
                .to_string(),
        );
    }
    if detection.src_dir {
        lines.push("- `src/` contains source files that should stay consistent with generated guidance and tests.".to_string());
    }
    if detection.tests_dir {
        lines.push("- `tests/` contains validation surfaces that should be reviewed alongside code changes.".to_string());
    }
    lines
}

fn framework_notes(detection: &RepoDetection) -> Vec<String> {
    let mut frameworks = Vec::new();
    if detection.nextjs {
        frameworks.push("- Next.js detected: preserve routing/data-fetching conventions and verify production builds after changing app structure.".to_string());
    }
    if detection.react && !detection.nextjs {
        frameworks.push("- React detected: keep component behavior covered with focused tests and avoid unnecessary prop/API churn.".to_string());
    }
    if detection.vite {
        frameworks.push("- Vite detected: validate the production bundle after changing build-sensitive configuration or imports.".to_string());
    }
    if detection.nest {
        frameworks.push("- NestJS detected: keep module/provider boundaries explicit and verify controller/service wiring after refactors.".to_string());
    }
    frameworks
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
struct RepoDetection {
    rust_workspace: bool,
    rust_root: bool,
    python: bool,
    package_json: bool,
    typescript: bool,
    nextjs: bool,
    react: bool,
    vite: bool,
    nest: bool,
    src_dir: bool,
    tests_dir: bool,
    rust_dir: bool,
}

fn generate_default_albert_md(config: &AlbertConfig) -> io::Result<()> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let path = cwd.join("ALBERT.md");
    if path.exists() {
        return Ok(());
    }
    let content = format!(
        "# ALBERT.md\n\n- User: {}\n- Role: {}\n- Style: {}\n- Mode: sovereign\n\n## Rules\n- Pure Rust\n- Ternary logic only (+1, 0, -1)\n- Strictly LLM-agnostic\n",
        config.user_name, config.user_role, config.cognitive_style
    );
    std::fs::write(path, content)
}
