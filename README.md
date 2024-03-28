# bma-benchmark

Benchmark for Rust and humans

## What is this for

A lightweight and simple benchmarking library for Rust.

## How to use

Let us create a simple benchmark, using the crate macros only:

```rust,ignore
#[macro_use]
extern crate bma_benchmark;

use std::sync::Mutex;

let n = 100_000_000;
let mutex = Mutex::new(0);
warmup!();
benchmark_start!();
std::hint::black_box(move || {
    for _ in 0..n {
        let _a = mutex.lock().unwrap();
    }
  })();
benchmark_print!(n);
```

The same can also be done with a single "benchmark" macro (black box is applied
automatically):

```rust,ignore
#[macro_use]
extern crate bma_benchmark;

use std::sync::Mutex;

let mutex = Mutex::new(0);
benchmark!(100_000_000, {
    let _a = mutex.lock().unwrap();
    });
```

![Simple benchmark result](https://raw.githubusercontent.com/alttch/bma-benchmark/main/simple.png)

Let us create a more complicated staged benchmark and compare e.g. Mutex vs
RwLock. Staged benchmarks display a comparison table. If the reference stage is
specified, the table also contains speed difference for all others.

```rust,ignore
#[macro_use]
extern crate bma_benchmark;

use std::sync::{Mutex, RwLock};

let n = 10_000_000;
let mutex = Mutex::new(0);
let rwlock = RwLock::new(0);
warmup!();
staged_benchmark_start!("mutex");
std::hint::black_box(move || {
    for _ in 0..n {
        let _a = mutex.lock().unwrap();
    }
  })();
staged_benchmark_finish_current!(n);
staged_benchmark_start!("rwlock-read");
std::hint::black_box(move || {
    for _ in 0..n {
        let _a = rwlock.read().unwrap();
    }
  })();
staged_benchmark_finish_current!(n);
staged_benchmark_print_for!("rwlock-read");
```

The same can also be done with a couple of *staged_benchmark* macros (black box
is applied automatically):

```rust,ignore
#[macro_use]
extern crate bma_benchmark;

use std::sync::{Mutex, RwLock};

let n = 10_000_000;
let mutex = Mutex::new(0);
let rwlock = RwLock::new(0);
warmup!();
staged_benchmark!("mutex", n, {
    let _a = mutex.lock().unwrap();
});
staged_benchmark!("rwlock-read", n, {
    let _a = rwlock.read().unwrap();
});
staged_benchmark_print_for!("rwlock-read");
```

Or split into functions with *benchmark_stage* attributes:

```rust,ignore
use std::sync::{Mutex, RwLock};

#[macro_use]
extern crate bma_benchmark;

#[benchmark_stage(i=10_000_000)]
fn benchmark_mutex(mutex: Mutex<u64>) {
    let _a = mutex.lock().unwrap();
}

#[benchmark_stage(i=10_000_000,name="rwlock-read")]
fn benchmark_rwlock(rwlock: RwLock<u64>) {
    let _a = rwlock.read().unwrap();
}

let mutex = Mutex::new(0);
let rwlock = RwLock::new(0);
benchmark_mutex(mutex);
benchmark_rwlock(rwlock);
staged_benchmark_print_for!("rwlock-read");
```

![Simple benchmark result](https://raw.githubusercontent.com/alttch/bma-benchmark/main/staged.png)

## Errors

The macros *benchmark_print*, *staged_benchmark_finish* and
*staged_benchmark_finish_current* accept error count as an additional
parameter.

For code blocks, macros *benchmark_check* and *staged_benchmark_check* can be
used. In this case, a statement MUST return true for the normal execution and
false for errors:

```rust.ignore
#[macro_use]
extern crate bma_benchmark;

use std::sync::Mutex;

let mutex = Mutex::new(0);
benchmark_check!(10_000_000, {
    mutex.lock().is_ok()
    });
```

The *benchmark_stage* attribute has got **check** option, which behaves
similarly. If used, the function body MUST (not return but) END with a bool as
well.

If any errors are reported, additional columns appear, success count, error
count and error rate:

![Simple benchmark result](https://raw.githubusercontent.com/alttch/bma-benchmark/main/errors.png)

## Latency benchmarks

(warming up and applying a black box is not recommended for latency benchmarks)

```rust,ignore
use bma_benchmark::LatencyBenchmark;

let mut lb = LatencyBenchmark::new();
for _ in 0..1000 {
    lb.op_start();
    // do something
    lb.op_finish();
}
lb.print();
```

```ignore
latency (μs) avg: 883, min: 701, max: 1_165
```

## Performance measurements

(warming up and applying a black box is not recommended for performance
measurements)

```rust,ignore
use bma_benchmark::Perf;

let file_path = "largefile";
let mut perf = Perf::new();
for _ in 0..10 {
    perf.start();
    let mut file = File::open(file_path).unwrap();
    perf.checkpoint("open");
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    perf.checkpoint("read");
    hasher.update(&buffer);
    perf.checkpoint("hash");
}
perf.print();
```

![Perf](https://raw.githubusercontent.com/alttch/bma-benchmark/main/perf1.png)

Need anything more sophisticated? Check the crate docs and use its structures
directly.

Enjoy!
