# `itron`

<a href="https://docs.rs/itron/"><img src="https://docs.rs/itron/badge.svg" alt="docs.rs"></a> <a href="https://kawadakk.github.io/itron-rs/doc/itron_asp3/index.html" label="Per-kernel documentation"><img src="https://kawadakk.github.io/itron-rs/doc/badge.svg"></a> <a href="https://crates.io/crates/itron"><img src="https://img.shields.io/crates/v/itron"></a> <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue">

A [Rust] interface to interact with operating systems based on [μITRON] or its derivative.

[Rust]: https://www.rust-lang.org/
[μITRON]: http://ertl.jp/ITRON/SPEC/mitron4-e.html

This is not an officially supported project.

## Examples

`Cargo.toml`:

```toml
[dependencies]
itron = { version = "0.1.0", features = ["asp3", "dcre"] }
```

Using the low-level binding:

```rust
use core::{mem, ptr};
use itron::abi::{acre_tsk, TA_ACT, T_CTSK, EXINF, ID};

fn create_task(param: isize) -> ID {
    extern "C" fn task_body(exinf: EXINF) {
        let _param = unsafe { exinf.assume_init() };
    }

    let er = unsafe {
        acre_tsk(&T_CTSK {
            tskatr: TA_ACT,
            exinf: mem::MaybeUninit::new(param),
            task: Some(task_body),
            itskpri: 4,
            stksz: 2048,
            stk: ptr::null_mut(),
        })
    };

    assert!(er >= 0, "could not create a task (error {})", er);
    er
}
```

Using the safe, high-level binding (experimental, requires `unstable` feature):

```rust
use itron::task;

fn create_task(param: isize) -> task::Task {
    let task_body = move || {
        let _param = param;
    };

    let new_task = task::Task::build()
        .start(create_task)
        .stack_size(2048)
        .initial_priority(4)
        .finish()
        .expect("could not create a task");

    new_task.as_ref().activate().unwrap();

    new_task
}
```
