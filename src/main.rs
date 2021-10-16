#![allow(dead_code)]

mod config;
mod context;
mod event;
mod fs;
mod ui;

use context::Context;

use structopt::StructOpt;
use tokio::time::sleep;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, StructOpt)]
#[structopt(name = "suha", about = "A cross platform terminal file manager.")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new(120u64).await?;

    let opts = Opt::from_args();
    let file_path: PathBuf = match opts.file {
        Some(path) => path,
        // Home if it's there, otherwise cwd
        None => match home::home_dir() {
            Some(path) => path,
            None => std::env::current_dir()?,
        },
    };

    app.run(file_path).await
}

pub struct App {
    context: Context,
    fps: u64,
}

impl App {
    pub async fn new(fps: u64) -> crossterm::Result<App> {
        let context = Context::new()?;
        Ok(App { context, fps })
    }

    pub async fn run(&mut self, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize cache from path
        self.context
            .cache
            .populate_to_root(&file_path, &self.context.config)?;

        if let Err(e) = self.event_loop(file_path).await {
            eprintln!("{}", e)
        };

        self.cleanup().await?;
        Ok(())
    }

    async fn event_loop(&mut self, file_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.render(&file_path).await?;

            if self.handle_event().await {
                break;
            };

            // Draw `fps` frames / second
            sleep(Duration::from_millis(1_000 / self.fps)).await;
        }

        Ok(())
    }

    async fn cleanup(&mut self) -> crossterm::Result<()> {
        self.context.painter.cleanup().await
    }

    async fn render(&mut self, file_path: &Path) -> crossterm::Result<()> {
        self.context
            .painter
            .render(&self.context.cache, file_path)
            .await
    }

    async fn handle_event(&mut self) -> bool {
        self.context.worker.handle_event()
    }
}
