use crate::{config::Config, event::Worker, fs::Cache};

pub struct Context {
    pub config: Config,
    pub worker: Worker,
    pub cache: Cache,
}

impl Context {
    pub fn new() -> crossterm::Result<Self> {
        Ok(Self {
            config: Config::try_load(),
            worker: Worker::new(),
            cache: Cache::new(),
        })
    }
}
