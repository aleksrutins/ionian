use std::time::Duration;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;

use crate::{log, task::Task};

lazy_static! {
    static ref PROGRESS: ProgressBar = {
        let p = ProgressBar::new(0);
        p.set_style(
            ProgressStyle::with_template("{spinner} {msg} {wide_bar:.cyan/blue} {pos:>7}/{len:7}")
                .unwrap()
                .progress_chars("##-"),
        );
        p.enable_steady_tick(Duration::from_millis(50));
        p
    };
}

pub fn run<'a>(tasks: &'a [Box<dyn Task + 'a>], print_list: bool) -> Result<()> {
    PROGRESS.set_position(0);
    PROGRESS.set_length(tasks.len() as u64);

    if print_list {
        println!("𜸛 build")
    }
    for task in tasks {
        PROGRESS.set_message(task.desc());
        task.run()?;
        PROGRESS.inc(1);
        if print_list {
            println!("𜸨 {}", task.desc())
        }
    }

    PROGRESS.reset();
    if print_list {
        println!("𜸽 done \u{1F389}")
    }
    Ok(())
}

pub fn done() {
    PROGRESS.finish_with_message("done \u{1F389}");
}

pub fn idle(msg: &'static str) {
    done();
    PROGRESS.set_message(msg);
}

pub fn bar() -> &'static ProgressBar {
    return &PROGRESS;
}
