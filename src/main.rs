
use clap::Parser;

mod cli;
mod commands;
mod config;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Add { path, name, raw } => commands::add::add(&path, name.as_deref(), raw),
        cli::Commands::Remove { name: _ } => println!("remove: not yet implemented"),
        cli::Commands::Apply => commands::apply::apply(),
        cli::Commands::Diff => println!("diff: not yet implemented"),
        cli::Commands::Push => println!("push: not yet implemented"),
        cli::Commands::Sync => commands::sync::sync(),
        cli::Commands::Init { url, path } => commands::init::init(url.as_deref(), path.as_deref()),
    }
}

