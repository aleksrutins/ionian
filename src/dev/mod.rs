use std::path::PathBuf;

use anyhow::Result;
use notify::Watcher;

use crate::{compile, progress};

pub mod server;
mod watch;

pub fn run(root: PathBuf, out: PathBuf, port: u16) -> Result<()> {
    compile::compile_all(root.clone(), out.clone())?;

    progress::bar().println("\x1b[31mdev\x1b[0m starting watcher");
    let mut watcher = watch::watcher(root.clone(), out.clone())?;
    watcher.watch(&root, notify::RecursiveMode::Recursive)?;

    progress::bar().println(&format!(
        "\x1b[31mdev\x1b[0m starting server on 127.0.0.1:{}",
        port
    ));
    server::run(out, port)?;

    Ok(())
}
