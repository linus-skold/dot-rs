use std::path::Path;

use crate::config::{resolve_target, DotEntries, ENTRIES_FILENAME};
use crate::output::{error, info, success, warning};

/// Syncs the named entries (or all entries when `names` is empty) from their
/// source locations into the dotfiles folder, honouring `<target>/.dotignore`.
pub fn sync(names: &[String]) {
    let target = resolve_target();
    let entries_path = target.join(ENTRIES_FILENAME);

    let entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        error!("failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    let mut items = entries.get_entries();

    if items.is_empty() {
        info!("nothing to sync — no entries in {}", entries_path.display());
        return;
    }

    if !names.is_empty() {
        let all: std::collections::HashMap<_, _> = items.into_iter().collect();
        items = names
            .iter()
            .filter_map(|name| match all.get(name) {
                Some(src) => Some((name.clone(), src.clone())),
                None => {
                    warning!("no entry named '{}' in {}", name, entries_path.display());
                    None
                }
            })
            .collect();
    }

    let ignore = super::load_ignore(&target);

    for (name, source) in &items {
        if !source.exists() {
            warning!("skipping '{}' — source does not exist: {}", name, source.display());
            continue;
        }

        let dest = target.join(name);

        if super::is_link_to(source, &dest) {
            info!("'{}' linked — already in sync", name);
            continue;
        }

        if super::is_ignored(ignore.as_ref(), Path::new(name), source.is_dir()) {
            info!("skipping '{}' — ignored by .dotignore", name);
            continue;
        }

        match super::copy_entry_filtered(source, &dest, ignore.as_ref(), Path::new(name)) {
            Ok(()) => success!("synced '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => error!("failed to sync '{}': {}", name, e),
        }
    }
}
