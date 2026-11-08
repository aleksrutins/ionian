use std::{fs, path::Path};

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
pub struct Config {
    #[serde(default)]
    pub hooks: Vec<Hook>,
    #[serde(default)]
    pub collections: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub name: String,
    pub glob: String,
    pub render: String,
}

pub fn read(root: &Path) -> Result<Config> {
    if let Ok(true) = fs::exists(root.join("ionian.toml"))
        && let Ok(s) = fs::read_to_string(root.join("ionian.toml"))
    {
        return Ok(toml::from_str(&s)?);
    }

    Ok(Config {
        hooks: vec![],
        collections: vec![],
    })
}
