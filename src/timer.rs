use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

pub struct CountdownTimer {
    start: Instant,
    duration: Duration,
}

impl CountdownTimer {
    pub fn new(duration: Duration) -> CountdownTimer {
        CountdownTimer {
            start: Instant::now(),
            duration,
        }
    }

    pub fn remaining(&self) -> Duration {
        self.duration.checked_sub(self.elapsed()).unwrap_or(Duration::new(0, 0))
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn is_finished(&self) -> bool {
        self.remaining() == Duration::new(0, 0)
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

pub struct AverageTimer {
    count: u32,
    total: Duration,
}

impl AverageTimer {
    pub fn new() -> AverageTimer {
        AverageTimer {
            count: 0,
            total: Duration::new(0, 0),
        }
    }

    pub fn add(&mut self, duration: Duration) {
        self.count += 1;
        self.total += duration;
    }

    pub fn average(&self) -> Duration {
        if self.count == 0 {
            Duration::new(0, 0)
        } else {
            self.total / self.count
        }
    }

    pub fn total(&self) -> Duration {
        self.total
    }

    pub fn interpolate(&self, remaining: u32) -> Duration {
        if self.count == 0 {
            Duration::new(0, 0)
        } else {
            self.total / self.count * remaining
        }
    }
}

pub struct BenchmarkTimer {
    times: HashMap<String, Instant>,
    finished: HashMap<String, (u32, Duration)>,
}

impl BenchmarkTimer {
    pub fn new() -> BenchmarkTimer {
        BenchmarkTimer {
            times: HashMap::new(),
            finished: HashMap::new(),
        }
    }

    pub fn start(&mut self, name: &str) {
        self.times.insert(name.to_string(), Instant::now());
    }

    pub fn stop(&mut self, name: &str) {
        let start = self.times.remove(name).unwrap();
        let duration = start.elapsed();
        let (count, total) = self
            .finished
            .entry(name.to_string())
            .or_insert((0, Duration::new(0, 0)));
        *count += 1;
        *total += duration;
    }

    pub fn average(&self, name: &str) -> Duration {
        let (count, total) = self.finished.get(name).unwrap();
        if *count == 0 {
            Duration::new(0, 0)
        } else {
            *total / *count
        }
    }

    pub fn print(&self) {
        println!("Benchmark results:");
        for (name, (count, total)) in &self.finished {
            println!(
                "{} - count: {} - total: {}ms - average: {}ms",
                name,
                count,
                total.as_millis(),
                self.average(name).as_millis()
            );
        }
    }
}
