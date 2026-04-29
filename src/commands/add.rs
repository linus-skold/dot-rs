use crate::config::{expand_tilde, DotRc, DOTRC_FILENAME};

pub fn add(path: &str, name: Option<&str>) {
    let source = expand_tilde(path);

    if !source.exists() {
        eprintln!("error: path does not exist: {}", source.display());
        std::process::exit(1);
    }
    if !source.is_dir() {
        eprintln!("error: path is not a directory: {}", source.display());
        std::process::exit(1);
    }

    // Derive entry name from the folder name if not provided.
    let entry_name = name
        .map(|n| n.to_string())
        .or_else(|| {
            source
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.to_string())
        })
        .unwrap_or_else(|| {
            eprintln!("error: could not determine entry name from path");
            std::process::exit(1);
        });

    let dotrc_path = std::env::current_dir()
        .expect("failed to get current dir")
        .join(DOTRC_FILENAME);

    let mut dotrc = DotRc::load(&dotrc_path).unwrap_or_else(|e| {
        eprintln!("error: failed to load {}: {}", DOTRC_FILENAME, e);
        std::process::exit(1);
    });

    if dotrc.is_tracked(&entry_name) {
        eprintln!(
            "warning: '{}' is already tracked in {} — skipping",
            entry_name, DOTRC_FILENAME
        );
        return;
    }

    let target_base = dotrc.get_target().unwrap_or_else(|| {
        eprintln!(
            "error: no target configured in {}. Add [settings] with target.win/unix.",
            DOTRC_FILENAME
        );
        std::process::exit(1);
    });

    let dest = target_base.join(&entry_name);

    if let Err(e) = super::copy_dir_all(&source, &dest) {
        eprintln!(
            "error: failed to copy '{}' to '{}': {}",
            source.display(),
            dest.display(),
            e
        );
        std::process::exit(1);
    }

    // Store the original source path (unexpanded) so it stays portable.
    dotrc.add_entry(&entry_name, path);

    if let Err(e) = dotrc.save() {
        eprintln!("error: failed to save {}: {}", DOTRC_FILENAME, e);
        std::process::exit(1);
    }

    println!(
        "tracked '{}': {} -> {}",
        entry_name,
        source.display(),
        dest.display()
    );
}

