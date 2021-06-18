#![no_std]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![cfg_attr(feature = "nightly", feature(decl_macro))]
#![doc = include_str!("lib.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::match_single_binding)] // the `cfg` matching pattern

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
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "unstable")))]
pub mod wait;

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
