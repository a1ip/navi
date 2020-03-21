use crate::flows;
use crate::flows::core::Variant;
use crate::structures::option::Command::{Best, Fn, Preview, Query, Repo, Search, Widget};
use crate::structures::option::{Config, RepoCommand};
use anyhow::Context;
use std::error::Error;

pub fn handle_config(mut config: Config) -> Result<(), Box<dyn Error>> {
    match config.cmd.as_mut() {
        None => flows::core::main(Variant::Core, config, true),
        Some(c) => match c {
            Preview { line } => Ok(flows::preview::main(&line[..])?),
            Query { query } => flows::query::main(query.clone(), config),
            Best { query, args } => flows::best::main(query.clone(), args.to_vec(), config),
            Search { query } => flows::search::main(query.clone(), config),
            Widget { shell } => flows::shell::main(&shell[..]),
            Fn { func, args } => flows::func::main(func.clone(), args.to_vec()),
            Repo { cmd } => match cmd {
                RepoCommand::Add { uri } => Ok(flows::repo::add(uri.clone())
                    .with_context(|| format!("Failed to import cheatsheets from {}", uri))?),
                RepoCommand::Browse => {
                    Ok(flows::repo::browse().context("Failed to browse featured cheatsheets")?)
                }
            },
        },
    }
}
