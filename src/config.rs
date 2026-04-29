use std::fs;
use std::path::{Path, PathBuf};

pub const DOTRC_FILENAME: &str = ".dotrc";

pub struct DotRc {
    pub path: PathBuf,
    data: toml::Table,
}

impl DotRc {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::new()
        };
        let data: toml::Table = toml::from_str(&content)?;
        Ok(Self {
            path: path.to_path_buf(),
            data,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(&self.data)?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    /// Returns the target base directory for the current OS, expanding `~`.
    pub fn get_target(&self) -> Option<PathBuf> {
        let settings = self.data.get("settings")?.as_table()?;
        let target = settings.get("target")?.as_table()?;

        #[cfg(target_os = "windows")]
        let key = "win";
        #[cfg(not(target_os = "windows"))]
        let key = "unix";

        let raw = target.get(key)?.as_str()?;
        Some(expand_tilde(raw))
    }

    pub fn is_tracked(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    /// Adds an entry recording where the folder lives on this OS.
    pub fn add_entry(&mut self, name: &str, source_path: &str) {
        let mut entry = toml::Table::new();

        #[cfg(target_os = "windows")]
        entry.insert("win".to_string(), toml::Value::String(source_path.to_string()));
        #[cfg(not(target_os = "windows"))]
        entry.insert("unix".to_string(), toml::Value::String(source_path.to_string()));

        self.data
            .insert(name.to_string(), toml::Value::Table(entry));
    }
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
