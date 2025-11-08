use std::path::Path;

use anyhow::Result;
use tera::{Context, Tera};

use crate::{assets::assets, config::Config, hooks, task::Task, template};

pub struct Tasks<'a> {
    pub pre_build: Vec<Box<dyn Task + 'a>>,
    config: &'a Config,
    root: &'a Path,
    out: &'a Path,
}

impl<'a> Tasks<'a> {
    pub fn compute(config: &'a Config, root: &'a Path, out: &'a Path) -> Result<Self> {
        Ok(Self {
            config,
            root,
            out,
            pre_build: hooks::pre_build(config).collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn compute_build(
        &self,
        tera: &'a Tera,
        context: &mut Context,
    ) -> Result<Vec<Box<dyn Task + 'a>>> {
        assets(self.root.join("assets"), self.out)?
            .chain(template::compile_all(tera, "pages/", self.out))
            .chain(hooks::post_build(self.config))
            .collect::<Result<Vec<_>>>()
    }
}
