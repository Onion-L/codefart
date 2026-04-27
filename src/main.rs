mod audio;
mod cli;
mod config;
mod error;
mod runner;
mod setup;
mod update;

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
        Commands::Remove => cmd_remove(),
        Commands::Setup => cmd_setup(),
        Commands::Preview { name } => cmd_preview(&name),
        Commands::Update => cmd_update(),
        Commands::Run { args } => cmd_run(&args),
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

fn cmd_theme(name: &Option<String>) -> Result<(), error::CodefartError> {
    let theme_name = match name {
        Some(n) => n.clone(),
        None => select_theme_interactive()?,
    };

    if !Config::is_valid_theme(&theme_name) {
        eprintln!(
            "Unknown theme: {}\nValid themes: {}",
            theme_name,
            config::BUILTIN_THEMES
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Err(error::CodefartError::UnknownTheme(theme_name));
    }

    let mut config = Config::load()?;
    config.theme = Some(theme_name.clone());
    config.save()?;
    println!("Theme set to \"{}\"", theme_name);
    Ok(())
}

fn select_theme_interactive() -> Result<String, error::CodefartError> {
    let config = Config::load().unwrap_or_default();
    let current = config.active_theme();

    let items: Vec<String> = config::BUILTIN_THEMES
        .iter()
        .map(|(name, desc)| format!("{:<12} {}", name, desc))
        .collect();

    let default_idx = config::BUILTIN_THEMES
        .iter()
        .position(|(n, _)| *n == current)
        .unwrap_or(0);

    let selection = dialoguer::Select::new()
        .with_prompt("Choose a theme")
        .items(&items)
        .default(default_idx)
        .interact()
        .map_err(|e| error::CodefartError::Other(format!("selection failed: {}", e)))?;

    Ok(config::BUILTIN_THEMES[selection].0.to_string())
}

fn cmd_set_sound(path: &str) -> Result<(), error::CodefartError> {
    // Expand ~ in path
    let expanded = shellexpand::tilde(path);
    let source = std::path::Path::new(expanded.as_ref());

    if !source.exists() {
        return Err(error::CodefartError::SoundFileNotFound(path.to_string()));
    }

    // Copy to managed sounds directory
    let sounds_dir = Config::sounds_dir();
    std::fs::create_dir_all(&sounds_dir)
        .map_err(|e| error::CodefartError::Other(format!("cannot create sounds dir: {}", e)))?;

    let filename = source
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("custom");
    let dest = sounds_dir.join(filename);
    std::fs::copy(source, &dest).map_err(|e| {
        error::CodefartError::Other(format!("cannot copy sound file: {}", e))
    })?;

    let stored_path = dest.to_string_lossy().to_string();
    let mut config = Config::load()?;
    config.custom_sound = Some(stored_path);
    config.save()?;
    println!("Custom sound set to {}", path);
    Ok(())
}

fn cmd_remove() -> Result<(), error::CodefartError> {
    let mut config = Config::load()?;
    config.custom_sound = None;
    config.save()?;

    // Clean up managed sounds
    let sounds_dir = Config::sounds_dir();
    if sounds_dir.exists() {
        let _ = std::fs::remove_dir_all(&sounds_dir);
    }

    println!("Custom sound removed. Using theme: {}", config.active_theme());
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

fn cmd_update() -> Result<(), error::CodefartError> {
    let path = update::update()?;
    println!("✓ Updated to latest version: {}", path);
    Ok(())
}

fn cmd_preview(name: &str) -> Result<(), error::CodefartError> {
    if !Config::is_valid_theme(name) {
        return Err(error::CodefartError::UnknownTheme(name.to_string()));
    }
    println!("Previewing {}...", name);
    audio::play_theme(name)
}

fn cmd_run(args: &[String]) -> Result<(), error::CodefartError> {
    if args.is_empty() {
        return Err(error::CodefartError::Other(
            "usage: codefart run -- <command> [args...]".into(),
        ));
    }

    let cmd = &args[0];
    let cmd_args = &args[1..];

    let status = runner::run_command(cmd, cmd_args)?;

    // Play sound regardless of exit code (audio errors are silent)
    let config = Config::load().unwrap_or_default();
    let _ = audio::play_sound(&config);

    // Forward the command's exit code
    let code = runner::status_to_code(status);
    if code != 0 {
        process::exit(code);
    }

    Ok(())
}
