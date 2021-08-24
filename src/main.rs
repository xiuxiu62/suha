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
#[structopt(name = "suha")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

const FPS: u64 = 120;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate `bash` completions in "target" directory
    Opt::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");

    let opts = Opt::from_args();
    match opts.file {
        Some(path) => {
            let mut stdout = setup()?;
            if let Err(e) = run(&mut stdout, path.as_path(), FPS).await {
                eprintln!("{}", e)
            };
            cleanup(&mut stdout)?;
        }
        None => eprintln!("\nplease provide a valid file path"),
    }

    Ok(())
}
