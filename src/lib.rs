#![no_std]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![cfg_attr(feature = "nightly", feature(decl_macro))]
// Can't be filtered by `cfg_attr`. I'm sure <https://github.com/rust-lang/rust/pull/83366>
// is going to be stabilized very soon.]
#![feature(extended_key_value_attributes)]
#![doc = include_str!("lib.md")]
#![deny(unsafe_op_in_unsafe_fn)]

#[macro_use]
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod error;

pub mod abi;

#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod closure;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod dataqueue;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod eventflag;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod interrupt;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod kernel;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod memorypool;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod mutex;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod prioritydataqueue;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod semaphore;
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod task;

/// Temporal quantification
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod time {
    mod duration;
    mod systime;
    mod timeout;
    pub use self::{duration::*, systime::*, timeout::*};
    // `use ::*` doesn't work with `pub macro`. This could be a bug.
    #[cfg(feature = "nightly")]
    pub use self::{duration::duration, timeout::timeout};
}
