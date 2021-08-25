use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::event::Worker;
use crate::fs::Cache;

#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub event_worker: Worker,
    pub session_cache: Arc<Mutex<Cache>>,
}

impl Context {
    pub fn new(config_file: &str) -> Self {
        let config = Config::load(config_file);
        let session_cache = Arc::from(Mutex::from(Cache::new()));
        let event_worker = Worker::new();

        Self {
            config,
            session_cache,
            event_worker,
        }
    }
}
