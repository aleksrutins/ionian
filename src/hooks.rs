use std::collections::HashMap;

use anyhow::{Result, anyhow};

use crate::{
    config::{Config, During, Hook},
    task::Task,
};

pub fn pre_build(config: &'_ Config) -> impl Iterator<Item = Result<Box<dyn Task + '_>>> + '_ {
    config
        .hooks
        .iter()
        .filter(|h| h.during == During::PreBuild)
        .map(|hook| Ok(Box::new(RunHookTask { hook }) as Box<dyn Task>))
}

pub fn post_build(config: &'_ Config) -> impl Iterator<Item = Result<Box<dyn Task + '_>>> + '_ {
    config
        .hooks
        .iter()
        .filter(|h| h.during == During::PostBuild)
        .map(|hook| Ok(Box::new(RunHookTask { hook }) as Box<dyn Task>))
}

pub struct RunHookTask<'a> {
    pub hook: &'a Hook,
}

impl<'a> Task for RunHookTask<'a> {
    fn desc(&self) -> String {
        self.hook.name.clone()
    }

    fn run(&self) -> Result<()> {
        let list = deno_task_shell::parser::parse(&self.hook.command)?;

        let env_vars = std::env::vars_os().collect::<HashMap<_, _>>();
        let cwd = std::env::current_dir()?;

        let exit_code = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(async {
                deno_task_shell::execute(
                    list,
                    env_vars,
                    cwd,
                    Default::default(),
                    Default::default(),
                )
                .await
            });

        if exit_code != 0 {
            Err(anyhow!(
                "hook {} exited with code {}",
                self.hook.name,
                exit_code
            ))
        } else {
            Ok(())
        }
    }
}
