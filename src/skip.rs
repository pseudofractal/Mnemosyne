use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::path::Path;

pub static EXT: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "tif", "tiff", "svg", "heic", "psd",
        "ai", "mp3", "wav", "flac", "ogg", "m4a", "aac", "mp3", "wav", "flac", "ogg", "m4a", "aac",
        "mp4", "m4v", "mov", "avi", "mkv", "webm", "ttf", "otf", "woff", "woff2", "zip", "tar",
        "gz", "bz2", "xz", "lz", "lzma", "7z", "rar", "iso", "exe", "dll", "so", "dylib", "a",
        "lib", "o", "obj", "class", "jar", "war", "ear", "wasm", "pdf", "doc", "docx", "odt",
        "xls", "xlsx", "ods", "ppt", "pptx", "odp", "bin", "dat", "pdb", "dSYM", "dmg", "pyc",
        "pyo", "pyd", "tmp", "temp", "bak",
    ]
    .into_iter()
    .collect()
});

pub static MIME: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        // Broad Categories
        "image/",
        "audio/",
        "video/",
        "font/",
        // Compressed / Packaged
        "application/zip",
        "application/x-bzip2",
        "application/x-xz",
        "application/x-7z-compressed",
        "application/x-rar-compressed",
        "application/gzip",
        "application/x-archive",
        // Native Executables and Libraries
        "application/x-executable",
        "application/x-mach-binary",
        "application/x-sharedlib",
        "application/wasm",
        "application/octet-stream",
        // Docs / Office
        "application/pdf",
        "application/vnd.ms-",
        "application/vnd.openxmlformats-officedocument",
    ]
    .into_iter()
    .collect()
});

pub static DIR: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        /* VCS */
        ".git",
        ".hg",
        ".svn",
        /* build systems & package managers */
        "node_modules",
        "target",
        "build",
        "dist",
        "vendor",
        "coverage",
        ".cargo",
        ".gradle",
        ".mvn",
        ".venv",
        /* IDE / editor junk */
        ".idea",
        ".vscode",
        ".classpath",
        ".settings",
        ".pytest_cache",
        ".mypy_cache",
        /* CI / automation configs */
        ".github",
        ".gitlab",
        ".circleci",
        /* OS / tool caches */
        "__pycache__",
        ".cache",
        ".DS_Store",
    ]
    .into_iter()
    .collect()
});

pub fn file(path: &Path, first_chunk: &[u8]) -> bool {
    // File Extensions Check
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if EXT.contains(&ext.to_ascii_lowercase().as_str()) {
            return true;
        }
    }
    // Mime Type Check
    if let Some(kind) = infer::get(first_chunk) {
        let mime = kind.mime_type();
        if MIME.iter().any(|blocked| mime.starts_with(blocked)) {
            return true;
        }
    }
    // Raw Binary Check ()
    first_chunk.contains(&0)
}

pub fn directory(path: &Path) -> bool {
    path.iter()
        .any(|c| c.to_str().map(|s| DIR.contains(s)).unwrap_or(false))
}
