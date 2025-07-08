# mnemosyne

Clones your brain’s lazy disk-usage audit without the excuses.  
It walks a project, writes a single `.contents.txt`, and saves you from scrolling through endless diffs.

## Features
* Global JSON-with-comments config in `$XDG_CONFIG_HOME/mnemosyne/config.jsonc`
* Per-project ignore file (`.mnemosyne_ignore`)
* Verbose mode that skips the performative spam (`.git/ ignored`, not 42 pages of object blobs)
* Extension and MIME-type filters
* Unsafe default line-chopping at 1000 lines because you probably don’t need more
* Parallel traversal for machines that aren’t from 1997
* Sensible defaults so you can forget the flag salad

## Usage
```bash
cargo install --path .
mnemosyne --help
```

## Configuration
```jsonc
{
  tree_ignore: [".git", "target"],
  ignore_file_name: ".mnemosyne_ignore",
  output_file_name: ".contents.txt",
  ignore_extensions: ["png", "zip"],
  ignore_mimetypes: ["image/*", "application/zip"],
  verbose: true
}
```
Edit it once; never think about it again.

## Building
```bash
cargo build --release
```
The CI uses the same command. If it fails for you, it fails in GitHub; no surprises.

## License
MIT. If that bothers you, delete the repo and pretend this never happened.
