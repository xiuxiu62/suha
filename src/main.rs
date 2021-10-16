#![allow(dead_code)]

mod app;
mod config;
mod context;
mod event;
mod fs;
mod ui;

use app::App;

use structopt::StructOpt;

use std::{error::Error, path::PathBuf};

#[derive(Debug, StructOpt)]
#[structopt(name = "suha", about = "A cross platform terminal file manager.")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opt::from_args();
    let file_path: PathBuf = match opts.file {
        Some(path) => path,
        // Home if it's there, otherwise cwd
        None => match home::home_dir() {
            Some(path) => path,
            None => std::env::current_dir()?,
        },
    };

    let mut app = App::new(file_path, 60u64).await?;
    app.run().await
}
