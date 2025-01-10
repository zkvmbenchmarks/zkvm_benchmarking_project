use std::time::{Instant, Duration};

pub struct Benchmarker {
    start_time: Option<Instant>,
}

impl Benchmarker {
    pub fn new() -> Benchmarker {
        Benchmarker { 
            start_time: None,
        }
    }

    pub fn start_benchmark(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn end_benchmark(&mut self) -> Option<Duration> {
        match self.start_time {
            Some(start) => Some(start.elapsed()),
            None => None,
        }
    }
}