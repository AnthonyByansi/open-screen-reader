use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::fs;

#[derive(Parser)]
#[command(name = "opensr", about = "Open Screen Reader demo shell")]
struct Args {
    /// Path to config TOML
    #[arg(long, default_value = "../config/config.toml")]
    config: String,

    /// Verbosity mode: short or verbose
    #[arg(long)]
    verbosity: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    speech: Speech,
    ui: Ui,
    verbosity: Verbosity,
}

#[derive(Debug, Deserialize)]
struct Speech {
    engine: String,
    rate: u32,
    pitch: u32,
    volume: u32,
    voice: String,
}

#[derive(Debug, Deserialize)]
struct Ui {
    language: String,
    theme: String,
}

#[derive(Debug, Deserialize)]
struct Verbosity {
    mode: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let cfg_text = fs::read_to_string(&args.config)?;
    let mut cfg: Config = toml::from_str(&cfg_text)?;

    if let Some(v) = args.verbosity {
        cfg.verbosity.mode = v;
    }

    println!("Open Screen Reader");
    println!("Speech engine: {}", cfg.speech.engine);
    println!("Voice: {}", cfg.speech.voice);
    println!("Language: {}", cfg.ui.language);
    println!("Verbosity: {}", cfg.verbosity.mode);
    println!("This is a starter shell. Hook UIA and speech drivers next.");
    Ok(())
}
