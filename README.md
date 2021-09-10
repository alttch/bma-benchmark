# bma-benchmark

Benchmark for Rust and humans

## What is this for

I like testing different libraries, crates and algorithms. I do benchmarks on
prototypes almost every day and decided to make a simple dedicated crate for
that. Here we go: <https://crates.io/crates/bma-benchmark>

The benchmark engine is very simple to launch and outputs all required data in
a pretty colored readable format.

## How to use

Let us create a simple benchmark, using crate macros only:

```rust
#[macro_use]
extern crate bma_benchmark;

use std::sync::Mutex;

let n = 10_000_000;
let mutex = Mutex::new(0);
benchmark_start!();
for _ in 0..n {
	let _a = mutex.lock().unwrap();
}
benchmark_print!(n);
```

![Simple benchmark result](https://raw.githubusercontent.com/alttch/bma-benchmark/main/simple.png)

Pretty cool, isn't it? Let us create more complex staged benchmark and compare
e.g. Mutex vs RwLock. Staged benchmarks display a comparison table, if the
etalon stage is specified, the table also contains speed difference for all
others.

```rust
#[macro_use]
extern crate bma_benchmark;

use std::sync::{Mutex, RwLock};

let n = 10_000_000;
let mutex = Mutex::new(0);
let rwlock = RwLock::new(0);
staged_benchmark_start!("mutex");
for _ in 0..n {
	let _a = mutex.lock().unwrap();
}
staged_benchmark_finish_current!(n);
staged_benchmark_start!("rwlock-read");
for _ in 0..n {
	let _a = rwlock.read().unwrap();
}
staged_benchmark_finish_current!(n);
staged_benchmark_print_for!("rwlock-read");
```

![Simple benchmark result](https://raw.githubusercontent.com/alttch/bma-benchmark/main/staged.png)

Need anything more complex? Check the crate docs and use structures manually.

Enjoy!
