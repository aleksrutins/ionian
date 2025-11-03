use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result, anyhow};
use glob::glob;
use mkdirp::mkdirp;

use crate::{log, task::Task};

pub fn assets<'a>(
    root: PathBuf,
    out: &'a Path,
) -> Result<Box<dyn Iterator<Item = Result<Box<dyn Task + 'a>>> + 'a>> {
    if log::fatal(fs::exists(&root).map_err(Error::from)) {
        Ok(Box::new(glob(root.join("**/*.*").to_str().unwrap())?.map(
            move |f| {
                match f {
                    Ok(entry) => Ok(Box::new(CopyAssetTask {
                        input: entry.clone(),
                        output: out.join("assets").join(
                            entry
                                .canonicalize()
                                .unwrap()
                                .strip_prefix(root.canonicalize().unwrap())
                                .unwrap_or(&entry),
                        ),
                    }) as Box<dyn Task>),
                    Err(e) => Err(e.into()),
                }
            },
        )))
    } else {
        Ok(Box::new(vec![].into_iter()))
    }
}

pub struct CopyAssetTask {
    pub input: PathBuf,
    pub output: PathBuf,
}

impl Task for CopyAssetTask {
    fn desc(&self) -> String {
        self.input.to_str().unwrap_or("(unknown file)").to_string()
    }

    fn run(&self) -> anyhow::Result<()> {
        mkdirp(
            self.output
                .parent()
                .ok_or(anyhow!("malformed output path"))?,
        )?;
        fs::copy(&self.input, &self.output)?;
        Ok(())
    }
}
