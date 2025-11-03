use std::{fs, path::PathBuf, process::exit};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use tera::Tera;

use crate::{assets::assets, config, hooks, log, task::Task, template};

pub fn compile_all(root: PathBuf, out: PathBuf) -> Result<()> {
    let tera = match Tera::new(root.join("**/*.html").to_str().unwrap()) {
        Ok(tera) => tera,
        Err(e) => {
            log::error(&format!("parsing error: {:?}", e));
            exit(1);
        }
    };

    let config = config::read(&root)?;
    let tasks = hooks::pre_build(&config)
        .chain(assets(root.join("assets"), &out)?)
        .chain(template::compile_all(&tera, "pages/", &out))
        .chain(hooks::post_build(&config))
        .collect::<Result<Vec<_>>>()?;

    let progress = ProgressBar::new(tasks.len() as u64);

    progress.set_style(
        ProgressStyle::with_template("{msg} {wide_bar:.cyan/blue} {pos:>7}/{len:7}")
            .unwrap()
            .progress_chars("##-"),
    );

    for task in tasks {
        progress.set_message(task.desc());
        task.run()?;
        progress.inc(1);
        progress.tick();
    }

    progress.finish_with_message("done \u{1F389}");
    Ok(())
}
