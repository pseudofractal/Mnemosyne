use anyhow::Result;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Config {
    pub root: std::path::PathBuf,
    pub output_file: String,
    pub extra_ignores: Vec<String>,
    pub ignore_filename: String,
}

impl Config {
    pub fn project_name(&self) -> String {
        self.root
            .canonicalize()
            .unwrap_or_else(|_| self.root.clone())
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_owned()
    }
    pub fn load(
        root: &Path,
        out: &str,
        extra: &[String],
        ignire_file: Option<&str>,
    ) -> Result<Self> {
        // optional JSONC config: $XDG_CONFIG_HOME/mnemosyne/config.jsonc
        let mut extra_ignores = extra.to_vec();
        if let Some(home) = std::env::var_os("XDG_CONFIG_HOME") {
            let cfg_path = Path::new(&home).join("mnemosyne").join("config.jsonc");
            if cfg_path.exists() {
                let raw = fs::read_to_string(cfg_path)?;
                // tolerate comments
                let json: serde_json::Value = json5::from_str(&raw)?;
                if let Some(arr) = json.get("ignore").and_then(|v| v.as_array()) {
                    for v in arr {
                        if let Some(s) = v.as_str() {
                            extra_ignores.push(s.to_owned());
                        }
                    }
                }
            }
        }
        Ok(Self {
            root: root.to_path_buf(),
            output_file: out.to_owned(),
            extra_ignores,
            ignore_filename: ignire_file.unwrap_or(".mnemosyne.ignore").to_owned(),
        })
    }
}
