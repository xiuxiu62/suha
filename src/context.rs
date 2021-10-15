use crate::{config::Config, event::Worker, fs::Cache};

#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub worker: Worker,
    pub cache: Cache,
}

impl Context {
    pub fn new() -> Self {
        Self {
            config: Config::try_load(),
            worker: Worker::new(),
            cache: Cache::new(),
        }
    }
}
