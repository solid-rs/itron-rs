All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

- **Changed:** `itron::task::Task::{set_priority → set_base_priority}`

## [0.1.9] - 2021-11-19

- Documentation update

## [0.1.8] - 2021-10-01

- **Fixed:** `itron::abi::exd_tsk` shouldn't be exposed unless `cfg(feature = "exd_tsk")`
- **Fixed:** Require `Send`-ness for the `impl IntoClosure` passed to `task::Builder::start`

## [0.1.7] - 2021-07-01

- **Fixed:** The macros in `itron::macros::match_kernel` did not match anything

## [0.1.6] - 2021-07-01

- **Added:** `itron::macros::{match_kernel, tt_is_kernel}` now support OR patterns
- **Fixed:** The macros in `itron::macros::*` now locate `tt_call` correctly

## [0.1.5] - 2021-07-01

- **Added:** `itron::processor::Processor` now implements `TryFrom<usize>`
- **Added:** `itron::macros`
- **Added:** `itron::{dataqueue, messagebuffer}::Info::is_empty`

## [0.1.4] - 2021-06-30

- **Changed:** `itron::task::Builder::{stack → stack_size}`
- **Changed:** `itron::{processor, task::{Task::activate_on, Builder::initial_processor}}`, etc. are now exposed on uniprocessor kernels
- **Added:** `itron::abi::{acre_cyc, acre_alm, acre_isr}`
- **Added:** `itron::abi::acre_mpf` when `cfg(all(feature = "solid_asp3", feature = "dcre"))`
- **Added:** `itron::abi::exd_tsk`
- **Added:** `itron::task::Builder::finish_and_activate`
- **Added:** `itron::task::TaskRef::migrate`
- **Added:** `itron::memorypool::MemoryPool[Ref]`
- **Added:** `itron::messagebuffer::MessageBuffer[Ref]`

## [0.1.3] - 2021-06-23

- **Added:** `itron::abi::E_OK`
- **Added:** `itron::dataqueue::Dataqueue[Ref]`
- **Changed:** `itron::processor::current` now returns `Result<Processor, _>` instead of `Result<Option<abi::NonNullID>, _>`.

## [0.1.2] - 2021-06-21

- **Added:** `itron::{mutex::TryLockError, semaphore::PollError}::Timeout`

## [0.1.1] - 2021-06-21

- **Added:** `itron::abi::{TA_TPRI, TA_WMUL, TA_CLR, TA_RTSK, TA_CEILING, TWF_*, TTW_*}`
- **Added:** `itron::semaphore::Semaphore[Ref]`
- **Added:** `itron::mutex::Mutex[Ref]`
- **Added:** `itron::processor`
- **Added:** `itron::abi::{*_mbf}` (message buffers) and Cargo feature `messagebuf`
- **Added:** `itron::abi::{*_ovr}` (overrun handlers) and Cargo feature `ovrhdr`
- **Added:** `itron::abi::chg_spr` (change task subpriority) and Cargo feature `subprio`
- **Added:** `itron::abi::TA_INHERIT` (priority inheritance mutexes) and Cargo feature `pi_mutex`
- **Added:** Cargo features `fmp3`, `solid_asp3`, and `solid_fmp3`

## 0.1.0 - 2021-06-15

- Initial release.

[Unreleased]: https://github.com/solid-rs/itron-rs/compare/0.1.9...main
[0.1.9]: https://github.com/solid-rs/itron-rs/compare/0.1.8...0.1.9
[0.1.8]: https://github.com/solid-rs/itron-rs/compare/0.1.7...0.1.8
[0.1.7]: https://github.com/solid-rs/itron-rs/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/solid-rs/itron-rs/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/solid-rs/itron-rs/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/solid-rs/itron-rs/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/solid-rs/itron-rs/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/solid-rs/itron-rs/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/solid-rs/itron-rs/compare/0.1.0...0.1.1
