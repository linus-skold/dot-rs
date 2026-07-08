use std::fs;
use std::path::{Path, PathBuf};

pub const DOTRC_FILENAME: &str = ".dotrc";
pub const ENTRIES_FILENAME: &str = "entries.toml";

/// Returns `~/.dotrc`.
pub fn dotrc_path() -> PathBuf {
    expand_tilde(&format!("~/{}", DOTRC_FILENAME))
}

/// Resolves the dotfiles target directory using this priority:
/// 1. `DOTCONF` environment variable (if set)
/// 2. `~/.dotrc` file (if present)
/// 3. Default: `~/.dot`
pub fn resolve_target() -> PathBuf {
    if let Ok(val) = std::env::var("DOTCONF") {
        return expand_tilde(val.trim());
    }
    let rc = dotrc_path();
    if rc.exists() {
        if let Ok(dotrc) = DotRc::load(&rc) {
            return dotrc.target;
        }
    }
    expand_tilde("~/.dot")
}


pub struct DotRc {
    pub path: PathBuf,
    /// Expanded target directory (e.g. `C:\Users\foo\.dot`).
    pub target: PathBuf,
    raw: String,
}

impl DotRc {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = fs::read_to_string(path)?.trim().to_string();
        if raw.is_empty() {
            return Err("~/.dotrc is empty — expected a target path".into());
        }
        let target = expand_tilde(&raw);
        Ok(Self { path: path.to_path_buf(), target, raw })
    }

    pub fn new_default(path: &Path) -> Self {
        let raw = "~/.dot/".to_string();
        let target = expand_tilde(&raw);
        Self { path: path.to_path_buf(), target, raw }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(&self.path, &self.raw)?;
        Ok(())
    }

    /// Path to the entries file inside the target folder.
    pub fn entries_path(&self) -> PathBuf {
        self.target.join(ENTRIES_FILENAME)
    }
}

/// Reads and writes `<target>/entries.toml`, tracking name -> source path mappings.
pub struct DotEntries {
    pub path: PathBuf,
    data: toml::Table,
}

impl DotEntries {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::new()
        };
        let data: toml::Table = toml::from_str(&content)?;
        Ok(Self { path: path.to_path_buf(), data })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(&self.path, toml::to_string(&self.data)?)?;
        Ok(())
    }

    pub fn is_tracked(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    /// Removes an entry, returning `true` if it was tracked.
    pub fn remove_entry(&mut self, name: &str) -> bool {
        self.data.remove(name).is_some()
    }

    pub fn add_entry(&mut self, name: &str, source_path: &str) {
        let mut entry = toml::Table::new();
        #[cfg(target_os = "windows")]
        entry.insert("win".to_string(), toml::Value::String(source_path.to_string()));
        #[cfg(not(target_os = "windows"))]
        entry.insert("unix".to_string(), toml::Value::String(source_path.to_string()));
        self.data.insert(name.to_string(), toml::Value::Table(entry));
    }

    /// Returns all entries as `(name, expanded_source_path)` pairs.
    pub fn get_entries(&self) -> Vec<(String, PathBuf)> {
        #[cfg(target_os = "windows")]
        let os_key = "win";
        #[cfg(not(target_os = "windows"))]
        let os_key = "unix";

        self.data
            .iter()
            .filter_map(|(name, value)| {
                let raw = value.as_table()?.get(os_key)?.as_str()?;
                Some((name.clone(), expand_tilde(raw)))
            })
            .collect()
    }
}

/// Collapses the user's home directory prefix back to `~/` for portable storage.
/// e.g. `C:\Users\linsko\AppData\Local\nvim` → `~/AppData/Local/nvim`
pub fn collapse_home(path: &std::path::Path) -> String {
    if let Some(home) = home_dir() {
        if let Ok(rest) = path.strip_prefix(&home) {
            let rest_str = rest.to_string_lossy().replace('\\', "/");
            let rest_str = rest_str.trim_end_matches('/');
            return format!("~/{}", rest_str);
        }
    }
    let s = path.to_string_lossy().replace('\\', "/");
    s.trim_end_matches('/').to_string()
}

/// Expands a leading `~` to the user's home directory.
pub fn expand_tilde(path: &str) -> PathBuf {
    if path == "~" {
        return home_dir().unwrap_or_else(|| PathBuf::from(path));
    }
    if let Some(rest) = path.strip_prefix("~/").or_else(|| path.strip_prefix("~\\")) {
        if let Some(home) = home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(path)
}

fn home_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE")
            .ok()
            .map(PathBuf::from)
            .or_else(|| {
                let drive = std::env::var("HOMEDRIVE").ok()?;
                let path = std::env::var("HOMEPATH").ok()?;
                Some(PathBuf::from(format!("{}{}", drive, path)))
            })
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
