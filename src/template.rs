use std::{
    fs::File,
    io::Write,
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
            output: out
                .join(
                    template
                        .strip_prefix("pages/")
                        .ok_or_else(|| anyhow!("malformed input path: {}", template))?,
                )
                .with_extension(if template.ends_with(".xml") { "xml" } else { "html" }),
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

#[cfg(test)]
mod tests {
    use super::CompileTask;
    use crate::task::Task;
    use std::{
        fs,
        path::Path,
        time::{SystemTime, UNIX_EPOCH},
    };
    use tera::Tera;

    #[test]
    fn maps_page_templates_to_html_output() {
        let tera = Tera::default();
        let task = CompileTask::new(&tera, "pages/about/index.html", Path::new("_build")).unwrap();

        assert_eq!(task.output, Path::new("_build/about/index.html"));
    }

    #[test]
    fn renders_a_template_to_its_output() {
        let out = std::env::temp_dir().join(format!(
            "ionian-template-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut tera = Tera::default();
        tera.add_raw_template("pages/index.html", "Hello").unwrap();
        let task = CompileTask::new(&tera, "pages/index.html", &out).unwrap();

        task.run().unwrap();
        assert_eq!(fs::read_to_string(out.join("index.html")).unwrap(), "Hello");
        fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn rejects_templates_outside_pages() {
        let tera = Tera::default();

        assert!(CompileTask::new(&tera, "layouts/base.html", Path::new("_build")).is_err());
    }
}
