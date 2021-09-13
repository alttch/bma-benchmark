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

let n = 100_000_000;
let mutex = Mutex::new(0);
benchmark_start!();
for _ in 0..n {
    let _a = mutex.lock().unwrap();
}
benchmark_print!(n);
```

The same can also be done with a single "benchmark" macro:

```rust
#[macro_use]
extern crate bma_benchmark;

use std::sync::Mutex;

let mutex = Mutex::new(0);
benchmark!(100_000_000, {
    let _a = mutex.lock().unwrap();
    });
```

![Simple benchmark result](https://raw.githubusercontent.com/alttch/bma-benchmark/main/simple.png)

Pretty cool, isn't it? Let us create a more complex staged benchmark and
compare e.g. Mutex vs RwLock. Staged benchmarks display a comparison table. If
the reference stage is specified, the table also contains speed difference for
all others.

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

The same can also be done with a couple of *staged_benchmark* macros:

```rust
#[macro_use]
extern crate bma_benchmark;

use std::sync::{Mutex, RwLock};

let n = 10_000_000;
let mutex = Mutex::new(0);
let rwlock = RwLock::new(0);
staged_benchmark!("mutex", n, {
    let _a = mutex.lock().unwrap();
});
staged_benchmark!("rwlock-read", n, {
    let _a = rwlock.read().unwrap();
});
staged_benchmark_print_for!("rwlock-read");
```

Or split into functions with *benchmark_stage* attributes:

```rust
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

```rust
#[macro_use]
extern crate bma_benchmark;

use std::sync::Mutex;

let mutex = Mutex::new(0);
benchmark!(100_000_000, {
    mutex.lock().is_ok()
    });
```

The *benchmark_stage* attribute has "check" option, which behaves similarly. If
used, the function body MUST (not return but) END with a bool as well.

If any benchmark stage has errors reported, additional two columns appear,
error count and error rate:

![Simple benchmark result](https://raw.githubusercontent.com/alttch/bma-benchmark/main/errors.png)

Need anything more complex? Check the crate docs and use structures manually.

Enjoy!
