use anyhow::Result;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Config {
    pub root: std::path::PathBuf,
    pub output_file: String,
    pub extra_ignores: Vec<String>,
    pub ignore_filename: String,
    pub dependency_graph: bool,
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
        output_path: &str,
        extra: &[String],
        ignore_file: Option<&str>,
        dependency_graph: bool,
    ) -> Result<Self> {
        // optional JSONC config: $XDG_CONFIG_HOME/mnemosyne/config.jsonc
        let mut extra_ignores = extra.to_vec();
        if let Some(home) = std::env::var_os("XDG_CONFIG_HOME") {
            let config_path = Path::new(&home).join("mnemosyne").join("config.jsonc");
            if config_path.exists() {
                let raw_file = fs::read_to_string(config_path)?;
                // tolerate comments
                let json: serde_json::Value = json5::from_str(&raw_file)?;
                if let Some(array) = json.get("ignore").and_then(|v| v.as_array()) {
                    for value in array {
                        if let Some(string) = value.as_str() {
                            extra_ignores.push(string.to_owned());
                        }
                    }
                }
            }
        }
        Ok(Self {
            root: root.to_path_buf(),
            output_file: output_path.to_owned(),
            extra_ignores,
            ignore_filename: ignore_file.unwrap_or(".mnemosyne.ignore").to_owned(),
            dependency_graph,
        })
    }
}
