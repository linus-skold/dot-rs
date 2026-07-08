use std::process::Command;

use crate::config::resolve_target;
use crate::output::error;

/// Runs `git pull` in the dotfiles target folder, then applies the updated entries.
pub fn pull(names: &[String], all: bool, force: bool) {
    let target = resolve_target();

    let status = Command::new("git")
        .args(["-C", target.to_str().expect("non-UTF-8 path"), "pull"])
        .status()
        .unwrap_or_else(|e| {
            error!("failed to run git: {}", e);
            std::process::exit(1);
        });

    if !status.success() {
        error!("git pull failed");
        std::process::exit(1);
    }

    super::apply::apply(names, all, force);
}
