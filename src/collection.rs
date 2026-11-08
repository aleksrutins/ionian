use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow, bail};
use glob::glob;
use mkdirp::mkdirp;
use serde::Serialize;
use tera::{Context, Tera, Value};

use crate::{config::Collection, task::Task};

#[derive(Serialize)]
struct ContentEntry {
    filename: String,
    frontmatter: Value,
}

pub fn register_globals(collections: &[Collection], root: &Path, tera: &mut Tera) -> Result<()> {
    let mut content = std::collections::BTreeMap::new();

    for collection in collections {
        if content.contains_key(&collection.name) {
            bail!("duplicate content collection name: {}", collection.name);
        }

        let pattern = root.join(&collection.glob);
        let mut entries = Vec::new();
        for entry in glob(
            pattern
                .to_str()
                .ok_or_else(|| anyhow!("invalid collection glob: {:?}", pattern))?,
        )? {
            let input = entry?;
            if !input.is_file() {
                continue;
            }

            let relative = input
                .strip_prefix(root)
                .map_err(|_| anyhow!("collection file is outside site root: {:?}", input))?;
            let source = fs::read_to_string(&input)?;
            let frontmatter = parse_frontmatter(&source, &input)?;
            entries.push(ContentEntry {
                filename: relative.to_string_lossy().into_owned(),
                frontmatter: frontmatter.value,
            });
        }
        content.insert(collection.name.clone(), entries);
    }

    tera.global_context().insert("content", &content);
    Ok(())
}

pub fn compile_all<'a>(
    collections: &'a [Collection],
    root: &'a Path,
    out: &'a Path,
    tera: &'a Tera,
) -> Result<Box<dyn Iterator<Item = Result<Box<dyn Task + 'a>>> + 'a>> {
    let mut tasks = Vec::new();

    for collection in collections {
        let pattern = root.join(&collection.glob);
        for entry in glob(
            pattern
                .to_str()
                .ok_or_else(|| anyhow!("invalid collection glob: {:?}", pattern))?,
        )? {
            let input = entry?;
            if input.is_file() {
                tasks.push(Box::new(RenderCollectionTask::new(
                    input,
                    root,
                    out,
                    &collection.render,
                    tera,
                )?) as Box<dyn Task>);
            }
        }
    }

    Ok(Box::new(tasks.into_iter().map(Ok)))
}

struct RenderCollectionTask<'a> {
    input: PathBuf,
    output: PathBuf,
    render: &'a str,
    tera: &'a Tera,
}

impl<'a> RenderCollectionTask<'a> {
    fn new(
        input: PathBuf,
        root: &Path,
        out: &Path,
        render: &'a str,
        tera: &'a Tera,
    ) -> Result<Self> {
        let relative = input
            .strip_prefix(root)
            .map_err(|_| anyhow!("collection file is outside site root: {:?}", input))?;

        Ok(Self {
            output: out.join(relative).with_extension("html"),
            input,
            render,
            tera,
        })
    }
}

impl Task for RenderCollectionTask<'_> {
    fn desc(&self) -> String {
        self.input.to_string_lossy().into_owned()
    }

    fn run(&self) -> Result<()> {
        let source = fs::read_to_string(&self.input)?;
        let frontmatter = parse_frontmatter(&source, &self.input)?;

        mkdirp(
            self.output
                .parent()
                .ok_or_else(|| anyhow!("malformed output path: {:?}", self.output))?,
        )?;
        let mut output = fs::File::create(&self.output)?;
        self.tera.render_component_to(
            self.render,
            &frontmatter.context,
            Some(&frontmatter.body),
            false,
            &mut output,
        )?;
        output.flush()?;
        Ok(())
    }
}

struct ParsedFrontmatter {
    context: Context,
    value: Value,
    body: String,
}

fn parse_frontmatter(source: &str, path: &Path) -> Result<ParsedFrontmatter> {
    let (delimiter, format) = if source.starts_with("---") {
        ("---", FrontmatterFormat::Yaml)
    } else if source.starts_with("+++") {
        ("+++", FrontmatterFormat::Toml)
    } else {
        bail!("content file has no frontmatter: {:?}", path);
    };

    let rest = &source[delimiter.len()..];
    let end = rest
        .find(&format!("\n{delimiter}"))
        .ok_or_else(|| anyhow!("unterminated frontmatter: {:?}", path))?;
    let metadata = rest[..end].trim_start_matches('\n');
    let body_start = end + 1 + delimiter.len();
    let body = rest[body_start..]
        .strip_prefix("\r\n")
        .or_else(|| rest[body_start..].strip_prefix('\n'))
        .unwrap_or(&rest[body_start..])
        .to_string();

    match format {
        FrontmatterFormat::Yaml => {
            let value = serde_yaml::from_str::<serde_yaml::Value>(metadata)?;
            Ok(ParsedFrontmatter {
                context: Context::from_serialize(&value)?,
                value: Value::try_from_serializable(&value)?,
                body,
            })
        }
        FrontmatterFormat::Toml => {
            let value = toml::from_str::<toml::Value>(metadata)?;
            Ok(ParsedFrontmatter {
                context: Context::from_serialize(&value)?,
                value: Value::try_from_serializable(&value)?,
                body,
            })
        }
    }
}

enum FrontmatterFormat {
    Yaml,
    Toml,
}
