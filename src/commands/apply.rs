use crate::config::{DotRc, DOTRC_FILENAME};

pub fn apply() {
    let dotrc_path = std::env::current_dir()
        .expect("failed to get current dir")
        .join(DOTRC_FILENAME);

    let dotrc = DotRc::load(&dotrc_path).unwrap_or_else(|e| {
        eprintln!("error: failed to load {}: {}", DOTRC_FILENAME, e);
        std::process::exit(1);
    });

    let target_base = dotrc.get_target().unwrap_or_else(|| {
        eprintln!(
            "error: no target configured in {}. Add [settings] with target.win/unix.",
            DOTRC_FILENAME
        );
        std::process::exit(1);
    });

    let entries = dotrc.get_entries();

    if entries.is_empty() {
        println!("nothing to apply — no entries configured in {}", DOTRC_FILENAME);
        return;
    }

    for (name, dest) in &entries {
        let source = target_base.join(name);

        if !source.exists() {
            eprintln!(
                "warning: skipping '{}' — dotfiles source does not exist: {}",
                name,
                source.display()
            );
            continue;
        }

        match super::copy_dir_all(&source, dest) {
            Ok(()) => println!("applied '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => eprintln!(
                "error: failed to apply '{}' ({} -> {}): {}",
                name,
                source.display(),
                dest.display(),
                e
            ),
        }
    }
}
