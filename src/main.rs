mod audio;
mod cli;
mod config;
mod error;
mod setup;

use std::process;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Play => cmd_play(),
        Commands::List => cmd_list(),
        Commands::Theme { name } => cmd_theme(&name),
        Commands::SetSound { path } => cmd_set_sound(&path),
        Commands::Reset => cmd_reset(),
        Commands::Setup => cmd_setup(),
    };

    match result {
        Ok(()) => process::exit(0),
        Err(e) => {
            if e.is_silent() {
                // Audio failures: silent exit, per PRD spec
                process::exit(0);
            }
            eprintln!("codefart: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_play() -> Result<(), error::CodefartError> {
    let config = Config::load()?;
    audio::play_sound(&config)?;
    Ok(())
}

fn cmd_list() -> Result<(), error::CodefartError> {
    let config = Config::load()?;
    let current_theme = config.active_theme();

    println!("Available themes:");
    for (name, desc) in config::BUILTIN_THEMES {
        let marker = if *name == current_theme { " ← current" } else { "" };
        println!("  {:<12} {}{}", name, desc, marker);
    }

    if let Some(ref custom) = config.custom_sound {
        println!("\nCustom sound: {}", custom);
    }

    Ok(())
}

fn cmd_theme(name: &str) -> Result<(), error::CodefartError> {
    if !Config::is_valid_theme(name) {
        eprintln!(
            "Unknown theme: {}\nValid themes: {}",
            name,
            config::BUILTIN_THEMES
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Err(error::CodefartError::UnknownTheme(name.to_string()));
    }

    let mut config = Config::load()?;
    config.theme = Some(name.to_string());
    config.save()?;
    println!("Theme set to \"{}\"", name);
    Ok(())
}

fn cmd_set_sound(path: &str) -> Result<(), error::CodefartError> {
    // Expand ~ in path
    let expanded = shellexpand::tilde(path);
    let expanded_path = expanded.as_ref();

    // Validate file exists
    if !std::path::Path::new(expanded_path).exists() {
        return Err(error::CodefartError::SoundFileNotFound(path.to_string()));
    }

    let mut config = Config::load()?;
    config.custom_sound = Some(path.to_string());
    config.save()?;
    println!("Custom sound set to {}", path);
    Ok(())
}

fn cmd_reset() -> Result<(), error::CodefartError> {
    let config = Config::default();
    config.save()?;
    println!("Reset to default theme (classic). Custom sound cleared.");
    Ok(())
}

fn cmd_setup() -> Result<(), error::CodefartError> {
    match setup::install_hook() {
        Ok(true) => {
            println!("✓ Added Stop hook to ~/.claude/settings.json");
            println!("✓ Done. Next Claude session will play a sound on response.");
        }
        Ok(false) => {
            println!("CodeFart hook is already installed. Nothing to do.");
        }
        Err(e) => return Err(e),
    }
    Ok(())
}
