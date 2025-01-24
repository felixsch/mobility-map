use std::collections::HashMap;
use std::time::Instant;
use tracing::{event, info, Level};

pub struct Timer {
    instant: Instant,
    extra_info: HashMap<&'static str, usize>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            instant: Instant::now(),
            extra_info: HashMap::new(),
        }
    }

    pub fn start_ticking(&mut self) {
        event!(Level::TRACE, "timer started");
        self.instant = Instant::now();
    }

    pub fn push_info(&mut self, key: &'static str, value: usize) {
        self.extra_info.insert(key, value);
    }

    pub fn show_duration(self) {
        let total = self.instant.elapsed().as_secs();
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        let seconds = total % 60;

        info!("Total duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
        for (key, value) in &self.extra_info {
            info!(" {}: {}", key, value)
        }
    }
}
