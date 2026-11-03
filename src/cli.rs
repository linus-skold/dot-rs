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
        /// Symlink targets to the dotfiles copy instead of copying. On Windows this needs
        /// Developer Mode enabled or an elevated (administrator) shell
        #[arg(short, long)]
        link: bool,
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
        /// Symlink targets to the dotfiles copy instead of copying. On Windows this needs
        /// Developer Mode enabled or an elevated (administrator) shell
        #[arg(short, long)]
        link: bool,
    },
    /// Sync tracked configs from their source locations into the dotfiles folder
    #[command(long_about = "Sync tracked configs from their source locations into the dotfiles folder.\n\n\
With no names, every entry in entries.toml is synced. Give one or more entry names to sync only those.\n\n\
Paths matching patterns in <dotfiles folder>/.dotignore (gitignore syntax, relative to the repo root; \
see `dot ignore --help`) are skipped. Entries that are symlinked to the dotfiles folder (dot apply --link) are already in sync and are skipped.\n\n\
Examples:\n  dot sync                  sync all entries\n  dot sync nvim wezterm     sync only nvim and wezterm")]
    Sync {
        /// Only sync entries with these names (default: all entries)
        names: Vec<String>,
    },
    /// Ignore paths when syncing/applying (appends to <dotfiles folder>/.dotignore)
    #[command(long_about = "Add patterns to <dotfiles folder>/.dotignore (next to entries.toml).\n\n\
The file uses gitignore syntax and patterns are relative to the dotfiles repo root, so an entry's files are addressed as <entry>/<path>. \
Ignored paths are skipped by `dot sync` and by copy-mode `dot apply` (not by `apply --link`). Duplicate patterns are not added twice.\n\n\
Examples:\n  dot ignore nvim/lazy-lock.json\n  dot ignore '*.log' '**/cache/'\n  dot ignore --list")]
    Ignore {
        /// gitignore-style patterns, relative to the dotfiles repo root
        patterns: Vec<String>,
        /// Print the current patterns instead of adding
        #[arg(long)]
        list: bool,
    },
    /// Initialize a dotfiles repo, optionally from a git URL
    Init {
        /// Git repository URL to clone (omit to just create local config)
        url: Option<String>,
        /// Where to clone the repo or create local config (defaults to ./<repo-name> or current dir)
        #[arg(short, long)]
        path: Option<String>,
    },
}



