//! C API
//!
//! Note: `doc(cfg(...))` is not used in this module as it's not very
//! useful here.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
mod error;
mod task;
mod types;
pub use self::{error::*, task::*, types::*};
