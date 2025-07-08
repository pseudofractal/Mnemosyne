use anyhow::Result;
use std::io::Write;
use std::path::Path;

pub fn ensure_gitignore(root: &Path, patterns: &[&str]) -> Result<()> {
    if !root.join(".git").exists() {
        return Ok(());
    }

    let gitignore = root.join(".gitignore");
    let existing = std::fs::read_to_string(&gitignore).unwrap_or_default();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&gitignore)?;

    for pat in patterns {
        if !existing.lines().any(|l| l.trim() == *pat) {
            writeln!(file, "{}", pat)?;
        }
    }
    Ok(())
}
