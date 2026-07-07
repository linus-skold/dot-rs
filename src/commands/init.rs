use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::config::{dotrc_path, expand_tilde, resolve_target, DotEntries, DotRc, ENTRIES_FILENAME};
use crate::output::{error, info, success};

pub fn init(url: Option<&str>, path: Option<&str>) {
    match url {
        Some(url) => init_from_url(url, path),
        None => init_local(path),
    }
}

fn init_local(path: Option<&str>) {
    let dest = match path {
        Some(p) => expand_tilde(p),
        None => resolve_target(),
    };

    if !dest.exists() {
        fs::create_dir_all(&dest).unwrap_or_else(|e| {
            error!("failed to create {}: {}", dest.display(), e);
            std::process::exit(1);
        });
        success!("created {}", dest.display());
    }

    git_init(&dest);
    create_dotrc_and_entries(&dest);
}

fn init_from_url(url: &str, path: Option<&str>) {
    let dest = resolve_dest(url, path);

    if dest.exists() {
        error!("destination '{}' already exists", dest.display());
        std::process::exit(1);
    }

    info!("cloning {} -> {}", url, dest.display());
    git_clone(url, &dest);

    create_dotrc_and_entries(&dest);
    success!("done — dotfiles repo ready at {}", dest.display());
}

fn create_dotrc_and_entries(target: &PathBuf) {
    // Write ~/.dotrc (single line: path to target folder)
    let dotrc_path = dotrc_path();
    if dotrc_path.exists() {
        info!("~/.dotrc already present — skipping");
    } else {
        // Store the raw unexpanded path so it stays portable
        let raw = format!("{}/", target.display());
        DotRc::new_default(&dotrc_path).save().unwrap_or_else(|e| {
            error!("failed to write ~/.dotrc: {}", e);
            std::process::exit(1);
        });
        success!("created ~/.dotrc -> {}", raw);
    }

    // Create entries.toml inside target if not present
    let entries_path = target.join(ENTRIES_FILENAME);
    if entries_path.exists() {
        info!("{} already present — skipping", entries_path.display());
    } else {
        DotEntries::load(&entries_path).and_then(|e| e.save()).unwrap_or_else(|e| {
            error!("failed to create {}: {}", entries_path.display(), e);
            std::process::exit(1);
        });
        success!("created {}", entries_path.display());
    }
}

fn resolve_dest(url: &str, path: Option<&str>) -> PathBuf {
    if let Some(p) = path {
        return expand_tilde(p);
    }
    let name = url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("dotfiles")
        .trim_end_matches(".git");
    std::env::current_dir()
        .expect("failed to get current dir")
        .join(name)
}

fn git_init(dest: &PathBuf) {
    let git_dir = dest.join(".git");
    if git_dir.exists() {
        info!("git repo already present — skipping git init");
        return;
    }

    let status = Command::new("git")
        .args(["-C", dest.to_str().expect("non-UTF-8 path"), "init", "-b", "main"])
        .status()
        .unwrap_or_else(|e| {
            error!("failed to run git: {}", e);
            std::process::exit(1);
        });

    if !status.success() {
        error!("git init failed");
        std::process::exit(1);
    }
}

fn git_clone(url: &str, dest: &PathBuf) {
    let status = Command::new("git")
        .args(["clone", url, dest.to_str().expect("non-UTF-8 path")])
        .status()
        .unwrap_or_else(|e| {
            error!("failed to run git: {}", e);
            std::process::exit(1);
        });

    if !status.success() {
        error!("git clone failed");
        std::process::exit(1);
    }
}
