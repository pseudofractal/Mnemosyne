use crate::fs_walk::FileEntry;
use std::collections::HashMap;

pub fn build(files: &[FileEntry]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    for f in files {
        if f.language != "rs" {
            continue;
        }
        let deps: Vec<String> = f
            .text
            .lines()
            .filter_map(|l| l.trim_start().strip_prefix("use "))
            .filter_map(|l| l.split("::").next())
            .filter(|seg| !seg.trim().is_empty())
            .map(|seg| format!("src/{}.rs", seg.trim()))
            .collect();

        graph.insert(rel(&f.path), deps);
    }
    graph
}

fn rel(p: &std::path::Path) -> String {
    p.to_string_lossy().into()
}

