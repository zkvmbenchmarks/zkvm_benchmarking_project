use std::time::{Instant, Duration};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use sysinfo::{System, SystemExt, ProcessExt};

pub struct Benchmarker {
    start_time: Option<Instant>,
    peak_memory: Arc<AtomicU64>,
    monitoring_thread: Option<thread::JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl Benchmarker {
    pub fn new() -> Benchmarker {
        let mut system = System::new_all();
        system.refresh_all();
        Benchmarker { 
            start_time: None,
            peak_memory: Arc::<AtomicU64>::new(0.into()),
            monitoring_thread: None,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start_benchmark(&mut self) {
        self.start_time = Some(Instant::now());
        self.peak_memory.store(0, Ordering::SeqCst);
        self.is_running.store(true, Ordering::SeqCst);
        
        let peak_memory = self.peak_memory.clone();
        let is_running = self.is_running.clone();
        let pid = sysinfo::get_current_pid().unwrap();
        
        self.monitoring_thread = Some(thread::spawn(move || {
            let mut sys = System::new_all();
            while is_running.load(Ordering::SeqCst) {
                sys.refresh_all();
                if let Some(process) = sys.processes().get(&pid) {
                    let current_memory = process.memory();
                    peak_memory.fetch_max(current_memory, Ordering::SeqCst);
                }
                thread::sleep(Duration::from_millis(10));
            }
        }));
    }

    pub fn end_benchmark(&mut self) -> Option<(Duration, u64)> {
        self.is_running.store(false, Ordering::SeqCst);
        
        if let Some(handle) = self.monitoring_thread.take() {
            let _ = handle.join();
        }

        match self.start_time {
            Some(start) => {
                let peak = self.peak_memory.load(Ordering::SeqCst);
                Some((start.elapsed(), peak))
            },
            None => None,
        }
    }
}

impl Drop for Benchmarker {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.monitoring_thread.take() {
            let _ = handle.join();
        }
    }
}