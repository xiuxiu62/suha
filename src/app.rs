use crate::{
    context::Context,
    event::{Command, SendResult},
    ui::Painter,
};

use tokio::time::{sleep, Duration};

use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub struct App {
    context: Context,
    painter: Painter,
    current_file: PathBuf,
    fps: u64,
}

impl App {
    pub async fn new(file_path: PathBuf, fps: u64) -> crossterm::Result<App> {
        let context = Context::new()?;
        let painter = Painter::new(context.worker.clone())?;

        Ok(App {
            context,
            painter,
            current_file: file_path,
            fps,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let config = &self.context.config;
        self.context
            .cache
            .populate_to_root(&self.current_file, config)?;

        if let Err(e) = self.event_loop().await {
            eprintln!("{}", e)
        };

        self.cleanup().await?;
        Ok(())
    }

    async fn event_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.update().await;
            self.render().await?;

            if self.handle_event().await? {
                break;
            };

            // Draw `fps` frames / second
            sleep(Duration::from_millis(1_000 / self.fps)).await;
        }

        Ok(())
    }

    async fn update(&mut self) {
        self.painter.update().await;
    }

    async fn render(&mut self) -> crossterm::Result<()> {
        let cache = &self.context.cache;
        self.painter.render(cache, &self.current_file).await
    }

    async fn handle_event(&mut self) -> SendResult<bool, Command> {
        self.context.worker.lock().await.handle_event().await
    }

    async fn cleanup(&mut self) -> crossterm::Result<()> {
        self.painter.cleanup().await
    }
}
