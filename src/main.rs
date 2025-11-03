mod assets;
mod compile;
mod config;
mod dev;
mod hooks;
#[allow(dead_code)]
mod log;
mod plan;
mod progress;
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
        #[arg(short, long, default_value = "_build")]
        out: PathBuf,
    },

    Dev {
        #[arg(default_value = ".")]
        root: PathBuf,
        #[arg(short, long, default_value = "_build")]
        out: PathBuf,
        #[arg(short, long, default_value = "3000")]
        port: u16,
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

    log::fatal(match args.cmd {
        Commands::Build { root, out } => compile_all(root, out),
        Commands::Dev { root, out, port } => dev::run(root, out, port),
    });

    progress::done();
}
