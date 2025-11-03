use std::{path::PathBuf, process::exit};

use anyhow::Result;
use tera::Tera;

use crate::{config, log, plan::Tasks, progress};

pub fn compile_all(root: PathBuf, out: PathBuf) -> Result<()> {
    let config = config::read(&root)?;
    let tasks = Tasks::compute(&config, &root, &out)?;

    progress::run(&tasks.pre_build)?;

    let tera = match Tera::new(root.join("**/*.html").to_str().unwrap()) {
        Ok(tera) => tera,
        Err(e) => {
            log::error(&format!("parsing error: {:?}", e));
            exit(1);
        }
    };

    let build = tasks.compute_build(&tera)?;

    progress::run(&build)?;

    Ok(())
}
