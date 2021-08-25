#![allow(dead_code)]

mod config;
mod context;
mod event;
mod fs;
mod run;
// mod ui;

use std::{
    env,
    path::{Path, PathBuf},
};

use structopt::StructOpt;

use run::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "suha", about = "A cross platform terminal file manager.")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fps: u64 = 120;
    let default_file: PathBuf = env::current_dir().unwrap().canonicalize().unwrap();
    let default_conf: PathBuf = default_file.clone().join(Path::new("../config.toml"));

    let opts = Opt::from_args();
    let file_path: PathBuf = match opts.file {
        Some(path) => path,
        None => PathBuf::from(default_file),
    };

    let mut stdout = setup()?;
    match run(&mut stdout, file_path, default_conf, fps).await {
        Ok(()) => cleanup(&mut stdout)?,
        Err(e) => {
            cleanup(&mut stdout)?;
            eprintln!("{}", e)
        }
    };

    Ok(())
}
