use std::path::Path;

use anyhow::Result;
use tera::{Context, Tera};

use crate::{
    config::{self, Config},
    task::Task,
};

pub fn load_collections<'a>(
    cfg: &Config,
    tera: &'a Tera,
    ctx: &'a mut Context,
    prefix: &'a Path,
    out: &'a Path,
) -> impl Iterator<Item = Result<Box<dyn Task + 'a>>> + 'a {
    cfg.content.iter().map(|collection| )
}
