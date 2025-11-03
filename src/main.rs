mod assets;
mod compile;
mod config;
mod hooks;
mod log;
mod task;
mod template;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::compile::compile_all;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build a static site
    Build {
        #[arg(default_value = ".")]
        root: PathBuf,
        #[arg(short, long, default_value = "./_build")]
        out: PathBuf,
    },
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Build { root, out } => log::fatal(compile_all(root, out)),
    }
}
