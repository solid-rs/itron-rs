#![no_std]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![cfg_attr(feature = "nightly", feature(decl_macro))]
// Can't be filtered by `cfg_attr`. I'm sure <https://github.com/rust-lang/rust/pull/83366>
// is going to be stabilized very soon.]
#![feature(extended_key_value_attributes)]
#![doc = include_str!("lib.md")]
#![deny(unsafe_op_in_unsafe_fn)]

#[macro_use]
pub mod error;

pub mod abi;
pub mod closure;
pub mod kernel;
pub mod task;

/// Temporal quantification
pub mod time {
    mod duration;
    mod timeout;
    pub use self::{duration::*, timeout::*};
    // `use ::*` doesn't work with `pub macro`. This could be a bug.
    #[cfg(feature = "nightly")]
    pub use self::{duration::duration, timeout::timeout};
}
