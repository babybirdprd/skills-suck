mod compiler;
mod indexer;
mod models;

use crate::compiler::Compiler;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the output file
    #[arg(short, long, default_value = "AGENTS.md")]
    output: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile skills into the output file (default behavior)
    Compile {
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let root = std::env::current_dir()?;

    let output_path = match &cli.command {
        Some(Commands::Compile { output: Some(o) }) => PathBuf::from(o),
        Some(Commands::Compile { output: None }) => PathBuf::from(&cli.output),
        None => PathBuf::from(&cli.output),
    };

    let compiler = Compiler::new(&root, &output_path);
    compiler.run()?;

    Ok(())
}
