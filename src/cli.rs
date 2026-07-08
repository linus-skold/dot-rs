use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dot", about = "Simple dotfiles manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Track a new config folder
    Add {
        /// Path to the folder or file to track (e.g. ~/AppData/Roaming/nvim)
        path: String,
        /// Name for this entry in entries.toml (defaults to folder/file name)
        #[arg(short, long)]
        name: Option<String>,
        /// Copy into the dotfiles folder without creating an entry in entries.toml
        #[arg(long)]
        raw: bool,
    },
    /// Remove a tracked config
    Remove {
        /// Name of the config entry to remove
        name: String,
    },
    /// Apply tracked entries from entries.toml to their source locations
    Apply {
        /// Only apply entries with these names (skips the interactive picker)
        names: Vec<String>,
        /// Apply all entries without prompting
        #[arg(short, long)]
        all: bool,
        /// Overwrite targets even if they have local changes
        #[arg(short, long)]
        force: bool,
    },
    /// Show differences between tracked entries and their source locations
    Diff,
    /// Push the dotfiles repo upstream (runs `git push` in the target folder)
    Push,
    /// Pull the dotfiles repo (git pull) and apply the updated entries
    Pull {
        /// Only apply entries with these names (skips the interactive picker)
        names: Vec<String>,
        /// Apply all entries without prompting
        #[arg(short, long)]
        all: bool,
        /// Overwrite targets even if they have local changes
        #[arg(short, long)]
        force: bool,
    },
    /// Sync tracked configs from their source locations into the dotfiles folder
    Sync,
    /// Initialize a dotfiles repo, optionally from a git URL
    Init {
        /// Git repository URL to clone (omit to just create local config)
        url: Option<String>,
        /// Where to clone the repo or create local config (defaults to ./<repo-name> or current dir)
        #[arg(short, long)]
        path: Option<String>,
    },
}



