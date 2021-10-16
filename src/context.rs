use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{config::Config, event::Worker, fs::Cache};

pub struct Context {
    pub config: Config,
    pub worker: Arc<Mutex<Worker>>,
    pub cache: Cache,
}

impl Context {
    pub fn new() -> crossterm::Result<Self> {
        Ok(Self {
            config: Config::try_load(),
            worker: Arc::new(Mutex::new(Worker::new())),
            cache: Cache::new(),
        })
    }
}
