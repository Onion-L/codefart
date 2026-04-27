use clap::{Parser, Subcommand};

/// CodeFart — play a sound when your AI finishes thinking.
#[derive(Parser, Debug)]
#[command(name = "codefart", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Play the currently configured sound (used by Claude hook)
    Play,

    /// List all built-in sound themes
    List,

    /// Switch to a built-in theme (interactive selection if no name given)
    Theme {
        /// Name of the theme
        name: Option<String>,
    },

    /// Set a custom sound file (overrides theme)
    SetSound {
        /// Path to an audio file (WAV, MP3, etc.)
        path: String,
    },

    /// Reset to default theme (classic), clear custom sound
    Reset,

    /// Remove custom sound, fall back to theme
    Remove,

    /// Install the Stop hook into Claude Code settings
    Setup,

    /// Preview a built-in theme without switching
    Preview {
        /// Name of the theme to preview
        name: String,
    },

    /// Update codefart to the latest version
    Update,

    /// Run a command and play a sound when it finishes
    Run {
        /// The command and its arguments (after --)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
        args: Vec<String>,
    },
}
