use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::{
    Event, EventKind,
    event::{AccessKind, AccessMode},
};

use crate::{compile, log, progress};

pub fn watcher(root: PathBuf, out: PathBuf) -> Result<notify::RecommendedWatcher> {
    let gitignore = {
        let mut builder = GitignoreBuilder::new(&root);
        builder.add(root.join(".gitignore"));
        builder.add_line(None, "_build/**/*")?;
        builder.add_line(None, ".git/**/*")?;
        builder.build()
    }?;

    Ok(notify::recommended_watcher(
        move |res: notify::Result<Event>| match res {
            Ok(evt) => {
                if evt.kind != EventKind::Access(AccessKind::Close(AccessMode::Write)) {
                    return;
                }

                let rebuild = evt.paths.iter().fold(false, |v, p| {
                    let relative = p
                        .strip_prefix(env::current_dir().unwrap().join(&root))
                        .unwrap_or(p);
                    v || !gitignore.matched(relative, false).is_ignore()
                });
                if !rebuild {
                    return;
                }

                compile::compile_all(root.clone(), out.clone())
                    .unwrap_or_else(|e| log::error(&format!("error in build: {:?}", e)));

                progress::idle("watching files");
            }
            Err(e) => log::error(&format!("error in watch: {:?}", e)),
        },
    )?)
}
