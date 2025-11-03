use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;

use crate::task::Task;

lazy_static! {
    static ref PROGRESS: ProgressBar = {
        let p = ProgressBar::new(0);
        p.set_style(
            ProgressStyle::with_template("{msg} {wide_bar:.cyan/blue} {pos:>7}/{len:7}")
                .unwrap()
                .progress_chars("##-"),
        );
        p
    };
}

pub fn run<'a>(tasks: &'a [Box<dyn Task + 'a>]) -> Result<()> {
    PROGRESS.set_length(tasks.len() as u64);

    for task in tasks {
        PROGRESS.set_message(task.desc());
        task.run()?;
        PROGRESS.inc(1);
        PROGRESS.tick();
    }

    PROGRESS.reset();
    Ok(())
}

pub fn done() {
    PROGRESS.finish_with_message("done \u{1F389}");
}

pub fn idle(msg: &'static str) {
    PROGRESS.reset();
    PROGRESS.set_length(1);
    PROGRESS.set_position(0);
    PROGRESS.set_message(msg);
}

pub fn bar() -> &'static ProgressBar {
    return &PROGRESS;
}
