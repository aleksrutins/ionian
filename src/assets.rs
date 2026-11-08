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

#[cfg(test)]
mod tests {
    use super::assets;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "ionian-{name}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn copies_nested_assets_to_the_assets_output() {
        let root = temp_dir("assets");
        let out = temp_dir("assets-out");
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("nested/style.css"), "body {}").unwrap();

        let tasks = assets(root.clone(), &out)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(tasks.len(), 1);
        tasks[0].run().unwrap();
        assert_eq!(
            fs::read_to_string(out.join("assets/nested/style.css")).unwrap(),
            "body {}"
        );

        drop(tasks);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(out).unwrap();
    }
}
