# Mnemosyne

> Because LLMs are crap without context.

Mnemosyne (Μνημοσύνη) named after the titaness of memory from Greek mythology, is a Rust CLI tool designed to help you create a comprehensive snapshot of your codebase for LLMs. It generates a `.mnemosyne.json` manifest that includes source chunks, checksums, dependency graphs, and more, all while keeping your git history intact and avoiding the clutter of binaries.

---

## Features

| What it does                          | Why you might care (or not)                                                                                 |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| **Single-file snapshot**              | Full path, SHA-256, size, token tally, 100-line chunks—ready for your crawler or chat agent.                |
| **Cruft-exterminator**                | Auto-skips VCS junk, `node_modules`, build artefacts, images, zips, and every other binary time-sink.       |
| **Config once, forget forever**       | Global `$XDG_CONFIG_HOME/mnemosyne/config.jsonc` + per-project ignore file. Change it one time and move on. |
| **Self-defending**                    | Adds its own output + ignore file to `.gitignore` so you don’t accidentally commit megabytes of JSON.       |
| **Parallel walk + sane progress bar** | Burns all CPU cores without LLM-inspirational quotes; just a straight counter.                              |

---

## Install

```bash
git clone https://github.com/pseudofractal/mnemosyne.git
cd mnemosyne
cargo build --release
cp target/release/mnemosyne ~/.local/bin/
```

Make sure `~/.local/bin` is on `$PATH`.

---

## Usage

```bash
mnemosyne [ROOT] [--output <file>] [--ignore <glob>]... [--ignore-file <name>]
```

### Quick hits

```bash
# Analyse current repo; writes .mnemosyne.json beside it
mnemosyne .

# Custom output and extra ignores
mnemosyne ~/code/agent --output brain.json --ignore "*.md" --ignore "tests/*"
```

---

## Configuration (optional)

```jsonc
{
  // global defaults live here
  "tree_ignore": [".git", "target"],
  "ignore_file_name": ".mnemosyne_ignore",
  "output_file_name": ".mnemosyne.json",
  "ignore_extensions": ["obb", "npy"],
  "ignore_mimetypes": ["image/*", "application/zip"],
  "verbose": true
}
```

Save as `$XDG_CONFIG_HOME/mnemosyne/config.jsonc`.\
Flags still override for your freedom to be intact.

---

## Output Anatomy

```jsonc
{
  "project_name": "cool-thing",
  "instruction": "...For LLMs only...",
  "env": { "os": "Arch BTW Linux rolling", "arch": "x86_64", "generated_at": "4200-04-20T04:20:00Z" },
  "files": [
    { "path": "src/lib.rs", "sha256": "badsha...", "tokens": 1234, "code": "while True:\n  print(\"Ree\")" }
  ],
  "dependency_graph": { "src/lib.rs": ["src/parser.rs", ...] },
  "directory_tree": "cool-thing\\n├── src\\n└── …",
  "ignored_files": ["*.png", ...]
}
```

Verbose on purpose because LLMs like details.

---

## Roadmap

- Better file recognition, add more file types and smarter heuristics.
- Allowing a whitelist of files to include.
- Using system's copy tool to automatcially copy the json file to user's clipboard.

---

## License

MIT.
