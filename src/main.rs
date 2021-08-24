#![allow(dead_code)]

mod event;
mod fs;
mod option;
mod run;
// mod ui;

use std::env;
use std::path::PathBuf;

use structopt::{clap::Shell, StructOpt};

use run::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "suha", about = "A cross platform terminal file manager.")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

const FPS: u64 = 120;
const DEFAULT_PATH: &'static str = "/home/xiuxiu";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate `bash` completions in "target" directory
    Opt::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

    let opts = Opt::from_args();
    let path: PathBuf = match opts.file {
        Some(path) => path,
        None => PathBuf::from(DEFAULT_PATH),
    };

    let mut stdout = setup()?;
    match run(&mut stdout, path.as_path(), FPS).await {
        Ok(()) => cleanup(&mut stdout)?,
        Err(e) => {
            cleanup(&mut stdout)?;
            eprintln!("{}", e)
        }
    };

    Ok(())
}
