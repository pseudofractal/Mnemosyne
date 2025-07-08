use crate::config::Config;
use crate::skip;
use anyhow::Result;
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::{path::Path, path::PathBuf};
use walkdir::WalkDir;

pub struct FileEntry {
    pub path: PathBuf,
    pub language: String,
    pub sha256: String,
    pub bytes: usize,
    pub tokens: usize,
    pub chunks: Vec<Chunk>,
    pub text: String,
}

#[derive(Clone)]
pub struct Chunk {
    pub idx: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub text: String,
}

const LINES_PER_CHUNK: usize = 100;

pub fn collect(cfg: &Config) -> Result<Vec<FileEntry>> {
    let total_files = WalkDir::new(&cfg.root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count() as u64;

    let pb = ProgressBar::new(total_files.max(1));
    pb.set_style(ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}",
    )?);

    let mut builder = WalkBuilder::new(&cfg.root);
    builder
        .hidden(false)
        .git_ignore(true)
        .git_exclude(true)
        .add_custom_ignore_filename(&cfg.ignore_filename)
        .filter_entry(|e| !skip::directory(e.path()));
    for pat in &cfg.extra_ignores {
        builder.add_ignore(pat);
    }

    let walker = builder.build();

    let files: Result<Vec<FileEntry>> = walker
        .filter_map(|res| res.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|e| match to_entry(e.path()) {
            Ok(Some(fe)) => {
                pb.inc(1);
                Some(Ok(fe))
            }
            Ok(None) => {
                pb.inc(1);
                None
            }
            Err(err) => {
                pb.inc(1);
                Some(Err(err))
            }
        })
        .collect();

    pb.finish_with_message("done");
    files
}

fn to_entry(path: &Path) -> Result<Option<FileEntry>> {
    let raw = std::fs::read(path)?;
    if skip::file(path, &raw[..raw.len().min(4096)]) {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&raw).into_owned();
    let sha256 = hex::encode(Sha256::digest(&raw));
    let bytes = raw.len();
    let tokens = text.split_whitespace().count();
    let language = language_from_ext(path);

    let lines: Vec<&str> = text.lines().collect();
    let mut chunks = Vec::new();
    for (i, window) in lines.chunks(LINES_PER_CHUNK).enumerate() {
        let start = i * LINES_PER_CHUNK + 1;
        let end = start + window.len() - 1;
        chunks.push(Chunk {
            idx: i,
            start_line: start,
            end_line: end,
            text: window.join("\n"),
        });
    }

    Ok(Some(FileEntry {
        path: path.to_owned(),
        language,
        sha256,
        bytes,
        tokens,
        chunks,
        text,
    }))
}

fn language_from_ext(p: &Path) -> String {
    p.extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
}
