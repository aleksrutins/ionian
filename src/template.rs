use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use mkdirp::mkdirp;
use tera::Tera;

use crate::task::Task;

pub fn compile_all<'a>(
    tera: &'a Tera,
    prefix: &'a str,
    out: &'a Path,
) -> impl Iterator<Item = Result<Box<dyn Task + 'a>>> + 'a {
    tera.get_template_names()
        .filter(move |n| n.starts_with(prefix))
        .map(|t| Ok(Box::new(CompileTask::new(tera, t, out)?) as Box<dyn Task>))
}

pub struct CompileTask<'a> {
    pub tera: &'a Tera,
    pub input: &'a str,
    pub output: PathBuf,
}

impl<'a> CompileTask<'a> {
    pub fn new(tera: &'a Tera, template: &'a str, out: &'a Path) -> Result<Self> {
        Ok(CompileTask {
            tera,
            input: template,
            output: out.join(
                template
                    .strip_prefix("pages/")
                    .ok_or_else(|| anyhow!("malformed input path: {}", t))?,
            ),
        })
    }
}

impl<'a> Task for CompileTask<'a> {
    fn desc(&self) -> String {
        self.input.to_string()
    }

    fn run(&self) -> Result<()> {
        mkdirp(
            self.output
                .parent()
                .ok_or_else(|| anyhow!("malformed output path: {:?}", self.output))?,
        )?;
        let out = File::create(&self.output)?;
        self.tera
            .render_to(self.input, &tera::Context::new(), out)?;
        Ok(())
    }
}
