use std::sync::{Arc, Mutex};

use crate::event::Worker;
use crate::fs::Cache;

#[derive(Debug, Clone)]
pub struct Flags {
    pub show_hidden: bool,
    pub show_icons: bool,
}

impl Flags {
    pub fn new(show_hidden: bool, show_icons: bool) -> Self {
        Self {
            show_hidden,
            show_icons,
        }
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub flags: Flags,
    pub event_worker: Worker,
    pub session_cache: Arc<Mutex<Cache>>,
}

impl Context {
    pub fn new(flags: Flags) -> Self {
        let session_cache = Arc::from(Mutex::from(Cache::new()));
        let event_worker = Worker::new();

        Self {
            flags,
            session_cache,
            event_worker,
        }
    }
}
