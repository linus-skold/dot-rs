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
        /// Path to the folder to track (e.g. ~/AppData/Roaming/nvim)
        path: String,
        /// Name for this entry in .dotrc (defaults to folder name)
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Remove a tracked config
    Remove {
        /// Name of the config entry to remove
        name: String,
    },
    /// Apply configurations based on .dotrc
    Apply,
    Diff,
    Push,
    /// Initialize a dotfiles repo, optionally from a git URL
    Init {
        /// Git repository URL to clone (omit to just create local config)
        url: Option<String>,
        /// Where to clone the repo or create local config (defaults to ./<repo-name> or current dir)
        #[arg(short, long)]
        path: Option<String>,
    },
}



