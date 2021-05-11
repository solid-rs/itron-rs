#![no_std]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
// Can't be filtered by `cfg_attr`. I'm sure <https://github.com/rust-lang/rust/pull/83366>
// is going to be stabilized very soon.]
#![feature(extended_key_value_attributes)]
#![doc = include_str!("lib.md")]
#![deny(unsafe_op_in_unsafe_fn)]

#[macro_use]
pub mod error;

pub mod abi;
pub mod kernel;
pub mod task;
