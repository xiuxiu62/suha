#![allow(dead_code)]

mod config;
mod context;
mod event;
mod fs;
mod run;
// mod ui;

use crate::run::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "suha", about = "A cross platform terminal file manager.")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fps: u64 = 120;

    let opts = Opt::from_args();
    let file_path: PathBuf = match opts.file {
        Some(path) => path,
        // Home if it's there, otherwise cwd
        None => match home::home_dir() {
            Some(path) => path,
            None => std::env::current_dir()?,
        },
    };

    let mut stdout = setup()?;
    match run(&mut stdout, file_path, fps).await {
        Ok(()) => cleanup(&mut stdout)?,
        Err(e) => {
            cleanup(&mut stdout)?;
            eprintln!("{}", e)
        }
    };

    Ok(())
}
