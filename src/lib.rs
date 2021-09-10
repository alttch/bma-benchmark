use colored::Colorize;
use std::fmt;
use std::time::Instant;

pub struct Benchmark {
    started: Instant,
    iterations: u32,
}

impl Default for Benchmark {
    fn default() -> Self {
        Self::new0()
    }
}

impl fmt::Display for Benchmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_for(self.iterations))
    }
}

impl Benchmark {
    pub fn new0() -> Self {
        Self {
            started: Instant::now(),
            iterations: 0,
        }
    }

    pub fn new(iterations: u32) -> Self {
        Self {
            started: Instant::now(),
            iterations,
        }
    }

    pub fn print(&self) {
        self.print_for(self.iterations);
    }

    pub fn print_for(&self, iterations: u32) {
        println!("{}", self.to_string_for(iterations));
    }

    pub fn to_string_for(&self, iterations: u32) -> String {
        let elapsed = self.started.elapsed().as_secs_f64();
        format!(
            "{}\n{}:\n {} secs ({} msecs)\n {} iters/s",
            (0..30).map(|_| "-").collect::<String>().black(),
            "Elapsed".black(),
            format!("{:.3}", elapsed).green(),
            format!("{:.3}", elapsed * 1000.0).cyan(),
            format!("{:.0}", f64::from(iterations) / elapsed).yellow()
        )
    }

    pub fn increment(&mut self) {
        self.iterations += 1;
    }
}
