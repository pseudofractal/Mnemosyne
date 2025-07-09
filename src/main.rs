use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;
use std::path::PathBuf;

mod config;
mod deps;
mod fs_walk;
mod git;
mod output;
mod skip;

#[derive(Parser)]
#[command(
    version,
    about = "Generate Mnemosyne manifest for your project to feed to LLMs."
)]
struct Cli {
    #[arg(default_value = ".")]
    root: PathBuf,

    /// Override output file name
    #[arg(long, default_value = ".mnemosyne.json")]
    output: String,

    /// Extra ignore glob (can repeat)
    #[arg(long)]
    ignore: Vec<String>,

    ///Name of Mnemosyne specific ignore file
    #[arg(long, default_value = ".mnemosyne.ignore")]
    ignore_file: String,

    /// Whether to build the dependency graph for each file
    #[arg(long, default_value = "false")]
    dependency_graph: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::Config::load(
        &cli.root,
        &cli.output,
        &cli.ignore,
        Some(&cli.ignore_file),
        cli.dependency_graph,
    )?;
    let files = fs_walk::collect(&cfg)?;
    let graph = if cfg.dependency_graph {
        Some(deps::build(&files))
    } else {
        None
    };
    output::write_manifest(&cfg, files, graph)?;
    git::ensure_gitignore(&cfg.root, &[&cfg.output_file, &cfg.ignore_filename])?;
    println!("{}", format!("🎉 {} analysed", cfg.project_name()).green());
    Ok(())
}
