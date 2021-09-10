#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;

use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use prettytable::Table;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use terminal_size::{terminal_size, Height, Width};

lazy_static! {
    pub static ref DEFAULT_BENCHMARK: Mutex<Benchmark> = Mutex::new(Benchmark::new0());
    pub static ref DEFAULT_STAGE_BENCHMARK: Mutex<StageBenchmark> =
        Mutex::new(StageBenchmark::new());
}

macro_rules! result_separator {
    () => {
        separator("--- Benchmark results ")
    };
}

macro_rules! format_number {
    ($n: expr) => {
        $n.to_formatted_string(&Locale::fr)
    };
}

#[macro_export]
macro_rules! staged_benchmark_start {
    ($name: expr) => {
        bma_benchmark::DEFAULT_STAGE_BENCHMARK
            .lock()
            .unwrap()
            .start($name);
    };
}

#[macro_export]
macro_rules! staged_benchmark_finish {
    ($name: expr, $iterations: expr) => {
        bma_benchmark::DEFAULT_STAGE_BENCHMARK
            .lock()
            .unwrap()
            .finish($name, $iterations);
    };
}

#[macro_export]
macro_rules! staged_benchmark_reset {
    () => {
        bma_benchmark::DEFAULT_STAGE_BENCHMARK
            .lock()
            .unwrap()
            .reset();
    };
}

#[macro_export]
macro_rules! staged_benchmark_print {
    () => {
        bma_benchmark::DEFAULT_STAGE_BENCHMARK
            .lock()
            .unwrap()
            .print();
    };
}

#[macro_export]
macro_rules! staged_benchmark_print_for {
    ($etalon: expr) => {
        bma_benchmark::DEFAULT_STAGE_BENCHMARK
            .lock()
            .unwrap()
            .print_for($etalon);
    };
}

#[macro_export]
macro_rules! staged_benchmark_finish_current {
    ($iterations: expr) => {
        bma_benchmark::DEFAULT_STAGE_BENCHMARK
            .lock()
            .unwrap()
            .finish_current($iterations);
    };
}

#[macro_export]
macro_rules! benchmark_start {
    () => {
        bma_benchmark::DEFAULT_BENCHMARK.lock().unwrap().reset();
    };
}

#[macro_export]
macro_rules! benchmark_print {
    ($iterations: expr) => {
        bma_benchmark::DEFAULT_BENCHMARK
            .lock()
            .unwrap()
            .print_for($iterations);
    };
}

pub struct BenchmarkResult {
    pub elapsed: Duration,
    pub iterations: u32,
    pub speed: u64,
}

pub struct StageBenchmark {
    benchmarks: BTreeMap<String, Benchmark>,
    current_stage: Option<String>,
}

impl Default for StageBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl StageBenchmark {
    pub fn new() -> Self {
        Self {
            benchmarks: BTreeMap::new(),
            current_stage: None,
        }
    }

    pub fn start(&mut self, name: &str) {
        let benchmark = Benchmark::new0();
        if self.benchmarks.insert(name.to_owned(), benchmark).is_some() {
            panic!("Benchmark stage {} is already started", name);
        }
        self.current_stage = Some(name.to_owned());
        println!("{}", format!("!!! stage started: {} ", name).black());
    }

    pub fn finish(&mut self, name: &str, iterations: u32) {
        let benchmark = self
            .benchmarks
            .get_mut(name)
            .expect(&format!("Benchmark stage {} not found", name));
        benchmark.finish_for(iterations);
        println!(
            "{}",
            format!(
                "*** stage completed: {} ({} iters)",
                name,
                format_number!(iterations)
            )
            .black()
        );
    }

    pub fn finish_current(&mut self, iterations: u32) {
        let current_stage = self
            .current_stage
            .take()
            .expect("No active benchmark stage");
        self.finish(&current_stage, iterations);
    }

    pub fn reset(&mut self) {
        self.benchmarks.clear();
    }

    pub fn _result_table_for(&self, eta: Option<&str>) -> Table {
        let mut header = vec!["stage", "sec", "msec", "iters/s"];
        let eta_speed = eta.map(|v| {
            header.push("diff");
            self.benchmarks.get(v).unwrap().result().speed
        });
        let mut table = ctable(Some(header), false);
        for (stage, benchmark) in &self.benchmarks {
            let result = benchmark.result();
            let elapsed = result.elapsed.as_secs_f64();
            let mut cells = vec![
                cell!(stage),
                cell!(format!("{:.3}", elapsed).green()),
                cell!(format!("{:.3}", elapsed * 1000.0).cyan()),
                cell!(format_number!(result.speed).yellow()),
            ];
            eta_speed.map(|r| {
                if result.speed != r {
                    let diff = result.speed as f64 / r as f64;
                    if diff > 1.0001 || diff < 0.9999 {
                        if diff > 1.0 {
                            cells.push(cell!(format!("+{:.2} %", ((diff - 1.0) * 100.0)).green()));
                        } else {
                            cells.push(cell!(format!("-{:.2} %", ((1.0 - diff) * 100.0)).red()));
                        }
                    }
                }
            });
            table.add_row(prettytable::Row::new(cells));
        }
        table
    }

    pub fn result_table(&self) -> Table {
        self._result_table_for(None)
    }

    pub fn result_table_for(&self, eta: &str) -> Table {
        self._result_table_for(Some(eta))
    }

    pub fn print(&self) {
        println!("{}", result_separator!());
        self.result_table().printstd();
    }

    pub fn print_for(&self, eta: &str) {
        println!("{}", result_separator!());
        self.result_table_for(eta).printstd();
    }
}

pub struct Benchmark {
    started: Instant,
    iterations: u32,
    elapsed: Option<Duration>,
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
            elapsed: None,
        }
    }

    pub fn new(iterations: u32) -> Self {
        Self {
            started: Instant::now(),
            iterations,
            elapsed: None,
        }
    }

    pub fn reset(&mut self) {
        self.started = Instant::now();
    }

    pub fn finish(&mut self) {
        self.elapsed = Some(self.started.elapsed());
    }

    pub fn finish_for(&mut self, iterations: u32) {
        self.elapsed = Some(self.started.elapsed());
        self.iterations = iterations;
    }

    pub fn set_iterations(&mut self, iterations: u32) {
        self.iterations = iterations;
    }

    pub fn print(&self) {
        self.print_for(self.iterations);
    }

    pub fn print_for(&self, iterations: u32) {
        println!("{}", self.to_string_for(iterations));
    }

    pub fn result(&self) -> BenchmarkResult {
        let elapsed = self.elapsed.unwrap_or_else(|| self.started.elapsed());
        BenchmarkResult {
            elapsed,
            iterations: self.iterations,
            speed: (f64::from(self.iterations) / elapsed.as_secs_f64()) as u64,
        }
    }

    pub fn result_for(&self, iterations: u32) -> BenchmarkResult {
        let elapsed = self.elapsed.unwrap_or_else(|| self.started.elapsed());
        BenchmarkResult {
            elapsed,
            iterations,
            speed: (f64::from(iterations) / elapsed.as_secs_f64()) as u64,
        }
    }

    pub fn to_string_for(&self, iterations: u32) -> String {
        let result = self.result_for(iterations);
        let elapsed = result.elapsed.as_secs_f64();
        format!(
            "{}\nElapsed:\n {} secs ({} msecs)\n {} iters/s",
            result_separator!(),
            format!("{:.3}", elapsed).green(),
            format!("{:.3}", elapsed * 1000.0).cyan(),
            format_number!(result.speed).yellow()
        )
    }

    pub fn increment(&mut self) {
        self.iterations += 1;
    }
}

fn ctable(titles: Option<Vec<&str>>, raw: bool) -> prettytable::Table {
    let mut table = prettytable::Table::new();
    let format = prettytable::format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(
            &[prettytable::format::LinePosition::Title],
            prettytable::format::LineSeparator::new('-', '-', '-', '-'),
        )
        .padding(0, 1)
        .build();
    table.set_format(format);
    if let Some(tt) = titles {
        let mut titlevec: Vec<prettytable::Cell> = Vec::new();
        for t in tt {
            if raw {
                titlevec.push(prettytable::Cell::new(t));
            } else {
                titlevec.push(prettytable::Cell::new(t).style_spec("Fb"));
            }
        }
        table.set_titles(prettytable::Row::new(titlevec));
    };
    table
}

fn separator(title: &str) -> colored::ColoredString {
    let size = terminal_size();
    let width = if let Some((Width(w), Height(_))) = size {
        w
    } else {
        40
    };
    (title.to_owned()
        + &(0..width - title.len() as u16)
            .map(|_| "-")
            .collect::<String>())
        .black()
}
