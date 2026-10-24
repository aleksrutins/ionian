use std::{path::PathBuf, process::exit};

use anyhow::{Result, anyhow};
use markdown::Options;
use tera::{Context, Error, Kwargs, State, Tera, TeraResult, Value};
use tera_contrib::{
    base64::{b64_decode, b64_encode},
    dates, format,
    json::json_encode,
    slug::slug,
    urlencode::urlencode,
};

use crate::{config, log, plan::Tasks, progress};

fn markdown_filter(kwargs: Kwargs, _: &State) -> TeraResult<Value> {
    Ok(markdown::to_html_with_options(
        kwargs
            .get("body")?
            .ok_or(Error::message("expected markdown to receive body"))?,
        &Options::gfm(),
    )
    .map(|s| Value::safe_string(&s))
    .map_err(|e| Error::message(e.reason))?)
}

pub fn compile_all(root: PathBuf, out: PathBuf) -> Result<()> {
    let config = config::read(&root)?;
    let tasks = Tasks::compute(&config, &root, &out)?;

    progress::run(&tasks.pre_build)?;

    let mut context = Context::new();

    let mut tera = Tera::default();
    tera.autoescape_on(Vec::<&str>::new());
    tera.register_filter("date", dates::date);
    tera.register_filter("is_before", dates::is_before);
    tera.register_filter("is_after", dates::is_after);
    tera.register_function("now", dates::now);
    tera.register_filter("format", format::format);
    tera.register_filter("json_encode", json_encode);
    tera.register_filter("slug", slug);
    tera.register_filter("urlencode", urlencode);
    tera.register_filter("b64_encode", b64_encode);
    tera.register_filter("b64_decode", b64_decode);
    tera.register_function("markdown", markdown_filter);

    tera.add_raw_template(
        "_internal.markdown.html",
        "
        {% component ionian.markdown() %}
        {{ markdown(body=body) | safe }}
        {% endcomponent %}
        ",
    )
    .unwrap();

    if let Err(e) = tera.load_from_glob(root.join("**/*.{html,md}").to_str().unwrap()) {
        log::error(&format!("parsing error: {:?}", e));
        exit(1);
    }

    let build = tasks.compute_build(&tera, &mut context)?;

    progress::run(&build)?;

    Ok(())
}
