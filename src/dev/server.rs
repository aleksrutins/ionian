use std::path::{Path, PathBuf};

use anyhow::Result;
use warp::filters::fs::dir;

pub fn run(out: PathBuf, port: u16) -> Result<()> {
    let fileserver = dir(out);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(warp::serve(fileserver).run(([127, 0, 0, 1], port)));

    Ok(())
}
