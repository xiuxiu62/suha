use crate::context::Context;

use tokio::time::{sleep, Duration};

use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub struct App {
    context: Context,
    fps: u64,
}

impl App {
    pub async fn new(fps: u64) -> crossterm::Result<App> {
        let context = Context::new()?;
        Ok(App { context, fps })
    }

    pub async fn run(&mut self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let config = &self.context.config;
        self.context.cache.populate_to_root(&file_path, config)?;

        if let Err(e) = self.event_loop(file_path).await {
            eprintln!("{}", e)
        };

        self.cleanup().await
    }

    async fn event_loop(&mut self, file_path: PathBuf) -> crossterm::Result<()> {
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
        let cache = &self.context.cache;
        self.context.painter.render(cache, file_path).await
    }

    async fn handle_event(&mut self) -> bool {
        self.context.worker.handle_event()
    }
}
