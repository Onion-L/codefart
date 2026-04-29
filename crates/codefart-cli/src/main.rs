mod cli;
mod runner;

use std::process;

use clap::Parser;
use cli::{Cli, Commands};
use codefart_core::config::Config;
use codefart_core::error::CodefartError;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Play => cmd_play(),
        Commands::List => cmd_list(),
        Commands::Theme { name } => cmd_theme(&name),
        Commands::SetSound { path } => cmd_set_sound(&path),
        Commands::Reset => cmd_reset(),
        Commands::Clear => cmd_clear(),
        Commands::Setup => cmd_setup(),
        Commands::Status => cmd_status(),
        Commands::Preview { name } => cmd_preview(&name),
        Commands::Update => cmd_update(),
        Commands::Run { args } => cmd_run(&args),
    };

    let result: Result<(), CodefartError> = result;

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

fn cmd_play() -> Result<(), CodefartError> {
    let config = Config::load()?;
    codefart_core::audio::play_sound(&config)?;
    Ok(())
}

fn cmd_list() -> Result<(), CodefartError> {
    let config = Config::load()?;
    let current_theme = config.active_theme();

    println!("Available themes:");
    for (name, desc) in codefart_core::config::BUILTIN_THEMES {
        let marker = if *name == current_theme {
            " ← current"
        } else {
            ""
        };
        println!("  {:<12} {}{}", name, desc, marker);
    }

    if let Some(ref custom) = config.custom_sound {
        println!("\nCustom sound: {}", custom);
    }

    Ok(())
}

fn cmd_theme(name: &Option<String>) -> Result<(), CodefartError> {
    let theme_name = match name {
        Some(n) => n.clone(),
        None => select_theme_interactive()?,
    };

    if !Config::is_valid_theme(&theme_name) {
        return Err(CodefartError::UnknownTheme(theme_name));
    }

    let mut config = Config::load()?;
    config.set_theme(&theme_name)?;
    config.save()?;
    println!("Theme set to \"{}\"", theme_name);
    Ok(())
}

fn select_theme_interactive() -> Result<String, CodefartError> {
    let config = Config::load().unwrap_or_default();
    let current = config.active_theme();

    let items: Vec<String> = codefart_core::config::BUILTIN_THEMES
        .iter()
        .map(|(name, desc)| format!("{:<12} {}", name, desc))
        .collect();

    let default_idx = codefart_core::config::BUILTIN_THEMES
        .iter()
        .position(|(n, _)| *n == current)
        .unwrap_or(0);

    let selection = dialoguer::Select::new()
        .with_prompt("Choose a theme")
        .items(&items)
        .default(default_idx)
        .interact()
        .map_err(|e| CodefartError::Other(format!("selection failed: {}", e)))?;

    Ok(codefart_core::config::BUILTIN_THEMES[selection]
        .0
        .to_string())
}

fn cmd_set_sound(path: &str) -> Result<(), CodefartError> {
    let mut config = Config::load()?;
    config.set_custom_sound_from_path(path)?;
    config.save()?;
    println!("Custom sound set to {}", path);
    Ok(())
}

fn cmd_clear() -> Result<(), CodefartError> {
    let mut config = Config::load()?;
    config.clear_custom_sound();
    config.save()?;

    println!(
        "Custom sound cleared. Using theme: {}",
        config.active_theme()
    );
    Ok(())
}

fn cmd_reset() -> Result<(), CodefartError> {
    let config = Config::default();
    config.save()?;
    println!("Reset to default theme (classic). Custom sound cleared.");
    Ok(())
}

fn cmd_setup() -> Result<(), CodefartError> {
    match codefart_core::setup::install_hook() {
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

fn cmd_status() -> Result<(), CodefartError> {
    let config = Config::load()?;

    // Hook status
    match codefart_core::setup::check_hook_installed() {
        Ok(true) => println!("Hook:     ✓ installed (~/.claude/settings.json)"),
        Ok(false) => println!("Hook:     ✗ not installed (run `codefart setup`)"),
        Err(_) => println!("Hook:     ? unable to check"),
    }

    // Theme / sound
    if let Some(ref path) = config.custom_sound {
        println!("Sound:    custom ({})", path);
    } else {
        println!("Theme:    {}", config.active_theme());
    }

    Ok(())
}

fn cmd_update() -> Result<(), CodefartError> {
    let path = codefart_core::update::update()?;
    println!("✓ Updated to latest version: {}", path);
    Ok(())
}

fn cmd_preview(name: &Option<String>) -> Result<(), CodefartError> {
    if let Some(n) = name {
        if !Config::is_valid_theme(n) {
            return Err(CodefartError::UnknownTheme(n.to_string()));
        }
        println!("Previewing {}...", n);
        return codefart_core::audio::play_theme(n);
    }

    // Interactive mode: select + preview in a loop, Ctrl-C to exit
    let items: Vec<String> = codefart_core::config::BUILTIN_THEMES
        .iter()
        .map(|(n, desc)| format!("{:<12} {}", n, desc))
        .collect();

    loop {
        let selection = dialoguer::Select::new()
            .with_prompt("Preview theme (Ctrl-C to exit)")
            .items(&items)
            .default(0)
            .interact()
            .map_err(|e| CodefartError::Other(format!("selection failed: {}", e)))?;

        let theme_name = codefart_core::config::BUILTIN_THEMES[selection].0;
        let _ = codefart_core::audio::play_theme(theme_name);
    }
}

fn cmd_run(args: &[String]) -> Result<(), CodefartError> {
    if args.is_empty() {
        return Err(CodefartError::Other(
            "usage: codefart run -- <command> [args...]\nnote: the `--` separator is required"
                .into(),
        ));
    }

    let cmd = &args[0];
    let cmd_args = &args[1..];

    let status = runner::run_command(cmd, cmd_args)?;

    // Play sound regardless of exit code (audio errors are silent)
    let config = Config::load().unwrap_or_default();
    let _ = codefart_core::audio::play_sound(&config);

    // Forward the command's exit code
    let code = runner::status_to_code(status);
    if code != 0 {
        process::exit(code);
    }

    Ok(())
}
