use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum During {
    PreBuild,
    PostBuild,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Dev {
    Watch(Vec<String>),
    Disabled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook {
    pub name: String,
    pub during: During,
    pub dev: Dev,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentCollection {
    pub name: String,
    pub out: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hooks: Vec<Hook>,
    pub content: Vec<ContentCollection>,
}

pub fn read(root: &Path) -> Result<Config> {
    if let Ok(true) = fs::exists(root.join("ionian.toml"))
        && let Ok(s) = fs::read_to_string(root.join("ionian.toml"))
    {
        return Ok(toml::from_str(&s)?);
    }

    Ok(Config {
        hooks: vec![],
        content: vec![],
    })
}
