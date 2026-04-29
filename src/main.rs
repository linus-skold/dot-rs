
use clap::Parser;

mod cli;
mod commands;
mod config;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Add { path, name } => commands::add::add(&path, name.as_deref()),
        cli::Commands::Remove { name: _ } => println!("remove: not yet implemented"),
        cli::Commands::Apply => println!("apply: not yet implemented"),
        cli::Commands::Diff => println!("diff: not yet implemented"),
        cli::Commands::Push => println!("push: not yet implemented"),
        cli::Commands::Init { url, path } => commands::init::init(&url, path.as_deref()),
    }
}

