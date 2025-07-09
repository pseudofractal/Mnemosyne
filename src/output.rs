use crate::config::Config;
use crate::fs_walk::FileEntry;
use crate::skip;
use anyhow::Result;
use chrono::Utc;
use ignore::WalkBuilder;
use os_info::Version;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{File, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};

const INSTRUCTION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/data/INSTRUCTIONS.txt"
));
const SCHEMA_PATH: &str = "https://raw.githubusercontent.com/pseudofractal/Mnemosyne/refs/heads/main/src/data/schema.json";

#[derive(Serialize)]
struct Manifest<'a> {
    schema: &'a str,
    project_name: String,
    instruction: &'a str,
    env: Env,
    files: Vec<FileOut<'a>>,
    dependency_graph: Option<HashMap<String, Vec<String>>>,
    directory_tree: String,
    ignored_files: Vec<String>,
}

#[derive(Serialize)]
struct Env {
    os: String,
    arch: String,
    generated_at: String,
}

#[derive(Serialize)]
struct FileOut<'a> {
    path: String,
    language: &'a str,
    sha256: &'a str,
    bytes: usize,
    tokens: usize,
    chunks: Vec<ChunkOut<'a>>,
}

#[derive(Serialize)]
struct ChunkOut<'a> {
    idx: usize,
    start_line: usize,
    end_line: usize,
    text: &'a str,
}

pub fn write_manifest(
    cfg: &Config,
    files: Vec<FileEntry>,
    graph: Option<HashMap<String, Vec<String>>>,
) -> Result<()> {
    let manifest = Manifest {
        schema: SCHEMA_PATH,
        project_name: project_name(&cfg.root)?,
        instruction: INSTRUCTION,
        env: build_env(),
        files: files
            .iter()
            .map(|f| FileOut {
                path: rel(&f.path),
                language: &f.language,
                sha256: &f.sha256,
                bytes: f.bytes,
                tokens: f.tokens,
                chunks: f
                    .chunks
                    .iter()
                    .map(|c| ChunkOut {
                        idx: c.idx,
                        start_line: c.start_line,
                        end_line: c.end_line,
                        text: &c.text,
                    })
                    .collect(),
            })
            .collect(),
        dependency_graph: graph,
        directory_tree: ascii_tree(cfg)?,
        ignored_files: cfg.extra_ignores.clone(),
    };

    let out_path = cfg.root.join(&cfg.output_file);
    if out_path.exists() {
        remove_file(&out_path)?;
    }
    let mut f = File::create(&out_path)?;
    f.write_all(serde_json::to_vec_pretty(&manifest)?.as_slice())?;
    Ok(())
}

fn build_env() -> Env {
    let info = os_info::get();
    let os_version = match info.version() {
        Version::Semantic(maj, min, pat) => format!("{}.{}.{}", maj, min, pat),
        Version::Rolling(Some(v)) => format!("rolling-{}", v),
        Version::Rolling(None) => "rolling".into(),
        Version::Custom(s) => s.clone(),
        Version::Unknown => "unknown".into(),
    };
    Env {
        os: format!("{} {}", info.os_type(), os_version),
        arch: std::env::consts::ARCH.into(),
        generated_at: Utc::now().to_rfc3339(),
    }
}

fn ascii_tree(config: &Config) -> Result<String> {
    let root = &config.root;

    let root_name = root
        .canonicalize()?
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_owned();
    let mut out = String::new();
    out.push_str(&root_name);
    out.push('\n');

    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .git_ignore(true)
        .git_exclude(true)
        .add_custom_ignore_filename(&config.ignore_filename)
        .filter_entry(|e| !skip::directory(e.path()));

    for pat in &config.extra_ignores {
        builder.add_ignore(pat);
    }

    let walker = builder.build();

    let mut paths: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.depth() > 0)
        .map(|e| e.path().strip_prefix(root).unwrap().to_path_buf())
        .collect();
    paths.sort();

    for p in paths {
        let depth = p.components().count() - 1;
        let indent = "│   ".repeat(depth.saturating_sub(1));
        let prefix = if depth == 0 {
            "├── "
        } else {
            "└── "
        };
        out.push_str(&indent);
        out.push_str(prefix);
        out.push_str(&p.to_string_lossy());
        out.push('\n');
    }
    Ok(out)
}

fn project_name(root: &Path) -> Result<String> {
    Ok(root
        .canonicalize()?
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_owned())
}

fn rel(p: &Path) -> String {
    p.to_string_lossy().into()
}
