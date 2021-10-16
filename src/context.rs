use crate::{config::Config, event::Worker, fs::Cache, ui::Painter};

pub struct Context {
    pub config: Config,
    pub worker: Worker,
    pub painter: Painter,
    pub cache: Cache,
}

impl Context {
    pub fn new() -> crossterm::Result<Self> {
        Ok(Self {
            config: Config::try_load(),
            worker: Worker::new(),
            painter: Painter::new()?,
            cache: Cache::new(),
        })
    }
}
