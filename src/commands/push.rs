use std::process::Command;

use crate::config::resolve_target;
use crate::output::error;

/// Runs `git push` in the dotfiles target folder.
pub fn push() {
    let target = resolve_target();

    let status = Command::new("git")
        .args(["-C", target.to_str().expect("non-UTF-8 path"), "push"])
        .status()
        .unwrap_or_else(|e| {
            error!("failed to run git: {}", e);
            std::process::exit(1);
        });

    if !status.success() {
        error!("git push failed");
        std::process::exit(1);
    }
}
