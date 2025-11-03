use anyhow::Result;

pub trait Task {
    fn desc(&self) -> String;
    fn run(&self) -> Result<()>;
}
