use std::process::exit;

use anyhow::Result;
use console::style;

pub fn info(message: &str) {
    println!("{} {}", style("info ").blue(), message);
}

pub fn warn(message: &str) {
    println!("{} {}", style("warn ").yellow(), message);
}

pub fn error(message: &str) {
    println!("{} {}", style("error").red(), message);
}

pub fn fatal<T>(res: Result<T>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            error(&format!("{:?}", e));
            Err::<T, anyhow::Error>(e).unwrap();
            exit(1);
        }
    }
}
