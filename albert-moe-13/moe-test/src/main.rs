//! # Albert-Test: Scientific Dashboard v2.0.0 (13-Node Evolution)
//! 
//! High-fidelity interactive Sandbox for the TIS with real-time telemetry, Smart-Stop, and Scrolling.

use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant, UNIX_EPOCH};
use std::fs;

use candle_core::{Device, DType, Tensor, IndexOp};
use candle_nn::VarBuilder;
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use serde_json::Value;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct App {
    model: Transformer,
    tokenizer: BpeTokenizer,
    input: String,
    transcript: String, 
    messages: Vec<(String, String)>, 
    model_id: String,
    checkpoint: String,
    total_epochs: u32,
    
    // Scientific Metrics
    token_latency_ms: u64,
    tokens_per_sec: f32,
    active_experts: String,
    est_gflops: f32,
    
    // UI State
    is_generating: bool,
    current_tokens: Vec<u32>,
    prompt_token_len: usize,
    tokens_to_generate: usize,
    scroll_pos: u16,
    auto_scroll: bool,
}

impl App {
    fn new() -> Self {
        let dev = Device::Cpu;
        let (checkpoint_path, version) = find_latest_checkpoint();
        
        let metadata = fs::metadata(&checkpoint_path).ok();
        let mtime = metadata.and_then(|m| m.modified().ok()).unwrap_or(UNIX_EPOCH);
        let elapsed = mtime.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO).as_secs();
        
        let vocab_path = "data/vocab.json";
        let tokenizer = BpeTokenizer::new(vocab_path);
        
        let mut config_path = format!("models/bible_ternary_{}.config.json", version);
        if !std::path::Path::new(&config_path).exists() {
            config_path = format!("models/registry/bible_ternary_{}.config.json", version);
        }
        let config_str = fs::read_to_string(&config_path).expect("Unable to read config.json. The HuggingFace standard requires a config file next to the model.");
        let config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");

        let mut config = TransformerConfig::default();
        config.vocab_size = tokenizer.vocab_size();
        config.hidden_size = config_json["hidden_size"].as_u64().unwrap() as usize;
        config.num_layers = config_json["num_layers"].as_u64().unwrap() as usize;
        config.num_heads = config_json["num_heads"].as_u64().unwrap() as usize;
        config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap() as usize;
        config.num_experts = config_json["num_experts"].as_u64().unwrap() as usize;

        let varmap = candle_nn::VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
        let model = Transformer::new(&config, vb).expect("Architecture initialization failed");

        let checkpoint_data = candle_core::safetensors::load(&checkpoint_path, &dev)
            .expect("Failed to load .safetensors weights");
        let all_vars = varmap.data().lock().unwrap();
        
        let mut loaded_count = 0;
        let mut missing_count = 0;

        for (name, var) in all_vars.iter() {
            if let Some(tensor) = checkpoint_data.get(name) {
                let _ = var.set(tensor);
                loaded_count += 1;
            } else {
                missing_count += 1;
            }
        }
        
        let meta_path = format!("models/bible_ternary_{}.meta", version);
        let total_epochs = fs::read_to_string(&meta_path).unwrap_or("0".to_string()).trim().parse::<u32>().unwrap_or(0);

        Self {
            model,
            tokenizer,
            input: String::new(),
            transcript: format!("System: Model {} loaded. Epoch Mileage: {}\n", version, total_epochs),
            messages: Vec::new(),
            model_id: "MoE-13-Ternary".to_string(),
            checkpoint: version,
            total_epochs,
            token_latency_ms: 0,
            tokens_per_sec: 0.0,
            active_experts: format!("{}/{} (Top-3)", if config.num_experts > 0 { 3 } else { 0 }, config.num_experts),
            est_gflops: 0.0,
            is_generating: false,
            current_tokens: Vec::new(),
            prompt_token_len: 0,
            tokens_to_generate: 0,
            scroll_pos: 0,
            auto_scroll: true,
        }
    }

    fn start_generation(&mut self) {
        if self.input.trim().is_empty() { return; }

        let user_msg = self.input.drain(..).collect::<String>();
        self.transcript.push_str(&format!("\nUser: {}\nAlbert: ", user_msg));

        self.current_tokens = self.tokenizer.encode(&user_msg);
        self.prompt_token_len = self.current_tokens.len();
        self.messages.push(("User".to_string(), user_msg));
        self.messages.push(("Albert".to_string(), String::new()));
        
        self.is_generating = true;
        self.tokens_to_generate = 64; 
        
        // Dynamic scroll adjustment based on line count (rough estimate)
        if self.auto_scroll {
            let line_count = self.transcript.lines().count() as u16;
            self.scroll_pos = line_count.saturating_sub(10); // Aim for the bottom
        }
    }

    fn step_generation(&mut self) {
        if !self.is_generating || self.tokens_to_generate == 0 {
            self.is_generating = false;
            return;
        }

        let start = Instant::now();
        let dev = Device::Cpu;
        
        let context_len = 64;
        let start_idx = self.current_tokens.len().saturating_sub(context_len);
        let context = &self.current_tokens[start_idx..];
        
        let input = Tensor::new(context, &dev).unwrap().unsqueeze(0).unwrap();
        let logits = self.model.forward(&input).unwrap();
        let dims = logits.dims();
        let last_logits = logits.i((0, dims[1] - 1)).unwrap();
        
        let pr = candle_nn::ops::softmax(&last_logits, 0).unwrap().to_vec1::<f32>().unwrap();
        let next_token = pr.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i as u32)
            .unwrap();

        self.current_tokens.push(next_token);
        self.tokens_to_generate -= 1;
        
        let elapsed = start.elapsed();
        self.token_latency_ms = elapsed.as_millis() as u64;
        if self.token_latency_ms > 0 {
            self.tokens_per_sec = 1000.0 / self.token_latency_ms as f32;
            self.est_gflops = (self.tokens_per_sec * 2_000_000.0) / 1_000_000_000.0;
        }

        // Decode only the generated tokens (everything after the prompt)
        let generated = &self.current_tokens[self.prompt_token_len..];
        let albert_text = self.tokenizer.decode(generated);

        if let Some(msg) = self.messages.last_mut() {
            let delta = if albert_text.len() > msg.1.len() {
                albert_text[msg.1.len()..].to_string()
            } else { String::new() };

            self.transcript.push_str(&delta);
            msg.1 = albert_text;
            
            // Stall detection
            let current_words: Vec<&str> = msg.1.split_whitespace().collect();
            if current_words.len() > 10 {
                let last_n = 3;
                if current_words.len() > last_n * 2 {
                    let tail = &current_words[current_words.len()-last_n..];
                    let prev = &current_words[current_words.len()-(last_n*2)..current_words.len()-last_n];
                    if tail == prev {
                        self.is_generating = false;
                        self.tokens_to_generate = 0;
                        self.transcript.push_str(" [STALL-VETO]");
                    }
                }
            }
        }

        if self.auto_scroll {
            let line_count = self.transcript.lines().count() as u16;
            self.scroll_pos = line_count.saturating_sub(10);
        }

        if self.tokens_to_generate == 0 {
            self.is_generating = false;
        }
    }
}

fn find_latest_checkpoint() -> (PathBuf, String) {
    let models_dir = "models";
    let search_paths = [
        ("v2.0.0", format!("{}/bible_ternary_v2.0.0.safetensors", models_dir)),
        ("v1.3.7", format!("{}/registry/bible_ternary_v1.3.7.safetensors", models_dir)),
        ("v1.3.7", format!("{}/bible_ternary_v1.3.7.safetensors", models_dir)),
        ("v1.3.6", format!("{}/bible_ternary_v1.3.6.safetensors", models_dir)),
        ("v1.3.5", format!("{}/bible_ternary_v1.3.5.safetensors", models_dir)),
    ];

    for (version, path_str) in search_paths {
        let path = PathBuf::from(path_str);
        if path.exists() {
            return (path, version.to_string());
        }
    }
    panic!("No checkpoints found in models/ or models/registry/. Start training first!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = if app.is_generating { Duration::from_millis(1) } else { Duration::from_millis(10) };
        
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        if app.is_generating {
                            app.is_generating = false;
                            app.tokens_to_generate = 0;
                        } else {
                            return Ok(());
                        }
                    }
                    KeyCode::Char(c) => {
                        if !app.is_generating { app.input.push(c); }
                    }
                    KeyCode::Backspace => {
                        if !app.is_generating { app.input.pop(); }
                    }
                    KeyCode::Enter => {
                        if !app.is_generating { app.start_generation(); }
                    }
                    KeyCode::Up => {
                        app.scroll_pos = app.scroll_pos.saturating_sub(1);
                        app.auto_scroll = false;
                    }
                    KeyCode::Down => {
                        app.scroll_pos = app.scroll_pos.saturating_add(1);
                        app.auto_scroll = false;
                    }
                    KeyCode::PageUp => {
                        app.scroll_pos = app.scroll_pos.saturating_sub(10);
                        app.auto_scroll = false;
                    }
                    KeyCode::PageDown => {
                        app.scroll_pos = app.scroll_pos.saturating_add(10);
                        app.auto_scroll = false;
                    }
                    KeyCode::End => {
                        app.auto_scroll = true;
                        app.scroll_pos = 1000;
                    }
                    _ => {}
                }
            }
        }

        if app.is_generating {
            app.step_generation();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Sandbox
                Constraint::Length(3), // Input
                Constraint::Length(3), // Metrics Row 1
                Constraint::Length(3), // Metrics Row 2
            ]
            .as_ref(),
        )
        .split(area);

    let header = Paragraph::new(format!(" {} | Scientific Dashboard", app.model_id))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    let scroll_text = if app.auto_scroll { 
        " [AUTO-FOLLOW] ".to_string() 
    } else { 
        format!(" [SCROLL: {}] ", app.scroll_pos) 
    };
    
    let sandbox = Paragraph::new(app.transcript.as_str())
        .block(Block::default().borders(Borders::ALL).title(format!(" Sandbox @ Simeon{} ", scroll_text)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .scroll((app.scroll_pos, 0));
    f.render_widget(sandbox, chunks[1]);

    let input_title = if app.is_generating { " [THINKING...] (Esc to Stop) " } else { " Input Terminal (Arrows to Scroll) " };
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[2]);
    
    if !app.is_generating {
        f.set_cursor_position((
            chunks[2].x + app.input.len() as u16 + 1,
            chunks[2].y + 1,
        ));
    }

    let m1_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
        .split(chunks[3]);

    f.render_widget(Paragraph::new(format!(" Brain: {}", app.checkpoint)).block(Block::default().borders(Borders::ALL).title(" Identity ")), m1_chunks[0]);
    f.render_widget(Paragraph::new(format!(" Speed: {:.2} tok/s", app.tokens_per_sec)).block(Block::default().borders(Borders::ALL).title(" Performance ")), m1_chunks[1]);
    f.render_widget(Paragraph::new(format!(" Experts: {}", app.active_experts)).block(Block::default().borders(Borders::ALL).title(" MoE Load ")), m1_chunks[2]);

    let m2_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
        .split(chunks[4]);

    f.render_widget(Paragraph::new(format!(" Mileage: {} Epochs", app.total_epochs)).block(Block::default().borders(Borders::ALL).title(" Total Experience ")), m2_chunks[0]);
    f.render_widget(Paragraph::new(format!(" Load: {:.4} GFLOPS", app.est_gflops)).block(Block::default().borders(Borders::ALL).title(" Intensity ")), m2_chunks[1]);
    f.render_widget(Paragraph::new(format!(" Latency: {}ms/tok", app.token_latency_ms)).block(Block::default().borders(Borders::ALL).title(" Depth ")), m2_chunks[2]);
}
