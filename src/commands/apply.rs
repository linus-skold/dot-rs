use crate::config::{resolve_target, DotEntries, ENTRIES_FILENAME};
use crate::output::{error, info, success, warning};
use dialoguer::{theme::Theme, MultiSelect};
use std::fmt;
use std::path::Path;
use std::process::Command;

struct CircleTheme;

impl Theme for CircleTheme {
    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {}",
            match (checked, active) {
                (true, true) => "> ◉",
                (true, false) => "  ◉",
                (false, true) => "> ○",
                (false, false) => "  ○",
            },
            text
        )
    }
}

/// Returns `true` if `tracked` and `dest` differ, per `git diff --no-index`.
/// Missing files or comparison errors are treated as "no local changes".
fn has_local_changes(tracked: &Path, dest: &Path) -> bool {
    if !tracked.exists() || !dest.exists() {
        return false;
    }

    Command::new("git")
        .arg("diff")
        .arg("--no-index")
        .arg("--quiet")
        .arg("--")
        .arg(tracked)
        .arg(dest)
        .status()
        .map(|status| status.code() == Some(1))
        .unwrap_or(false)
}

pub fn apply(names: &[String], all: bool, force: bool) {
    let target = resolve_target();
    let entries_path = target.join(ENTRIES_FILENAME);

    let entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        error!("failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    let mut items = entries.get_entries();
    items.sort_by(|a, b| a.0.cmp(&b.0));

    if items.is_empty() {
        info!("nothing to apply — no entries in {}", entries_path.display());
        return;
    }

    let selected: Vec<(String, std::path::PathBuf)> = if !names.is_empty() {
        let items: std::collections::HashMap<_, _> = items.into_iter().collect();
        names
            .iter()
            .filter_map(|name| match items.get(name) {
                Some(dest) => Some((name.clone(), dest.clone())),
                None => {
                    warning!("no entry named '{}' in {}", name, entries_path.display());
                    None
                }
            })
            .collect()
    } else if all {
        items
    } else {
        let mut labels: Vec<String> = items
            .iter()
            .map(|(name, dest)| format!("{} -> {}", name, dest.display()))
            .collect();
        labels.push("All".to_string());
        let all_idx = labels.len() - 1;

        let chosen = MultiSelect::with_theme(&CircleTheme)
            .with_prompt("Select dotfiles to apply (space to toggle, enter to confirm, esc to cancel)")
            .items(&labels)
            .interact_opt()
            .unwrap_or_else(|e| {
                error!("failed to read selection: {}", e);
                std::process::exit(1);
            });

        let Some(chosen) = chosen else {
            info!("cancelled");
            return;
        };

        if chosen.is_empty() {
            info!("nothing selected — aborting");
            return;
        }

        if chosen.contains(&all_idx) {
            items
        } else {
            chosen.into_iter().map(|i| items[i].clone()).collect()
        }
    };

    if selected.is_empty() {
        info!("nothing to apply");
        return;
    }

    for (name, dest) in &selected {
        let source = target.join(&name);

        if !source.exists() {
            warning!("skipping '{}' — dotfiles source does not exist: {}", name, source.display());
            continue;
        }

        if !force && has_local_changes(&source, dest) {
            warning!(
                "skipping '{}' — target has local changes not present in the dotfiles copy: {}",
                name, dest.display()
            );
            warning!("         run 'dot diff' to review, or re-run with --force to overwrite");
            continue;
        }

        match super::copy_entry(&source, dest) {
            Ok(()) => success!("applied '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => error!("failed to apply '{}': {}", name, e),
        }
    }
}
