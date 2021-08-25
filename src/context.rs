use std::sync::{Arc, Mutex};

use crate::{event::Worker, fs::Cache, option::DisplayOptions};

#[derive(Debug)]
pub struct Context {
    pub options: DisplayOptions,
    pub event_worker: Worker,
    pub session_cache: Arc<Mutex<Cache>>,
}

impl Context {
    pub fn new() -> Self {
        let options = DisplayOptions::new(false, true);
        let session_cache = Arc::from(Mutex::from(Cache::new()));
        let event_worker = Worker::new();

        Self {
            options,
            session_cache,
            event_worker,
        }
    }
}
