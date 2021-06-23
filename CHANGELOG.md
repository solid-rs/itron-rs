All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/kawadakk/itron-rs/compare/0.1.2...main
[0.1.2]: https://github.com/kawadakk/itron-rs/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/kawadakk/itron-rs/compare/0.1.0...0.1.1
