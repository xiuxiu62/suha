use crate::{config::Config, event::Worker, fs::Cache};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub event_worker: Worker,
    pub session_cache: Arc<Mutex<Cache>>,
}

impl Context {
    pub fn new() -> Self {
        let config = Config::try_load();
        let session_cache = Arc::from(Mutex::from(Cache::new()));
        let event_worker = Worker::new();

        Self {
            config,
            session_cache,
            event_worker,
        }
    }
}
