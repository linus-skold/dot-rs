
use clap::Parser;

mod cli;
mod commands;
mod config;
mod output;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Add { path, name, raw } => commands::add::add(&path, name.as_deref(), raw),
        cli::Commands::Remove { name } => commands::remove::remove(&name),
        cli::Commands::Apply { names, all, force } => commands::apply::apply(&names, all, force),
        cli::Commands::Diff => commands::diff::diff(),
        cli::Commands::Push => commands::push::push(),
        cli::Commands::Sync => commands::sync::sync(),
        cli::Commands::Init { url, path } => commands::init::init(url.as_deref(), path.as_deref()),
    }
}

