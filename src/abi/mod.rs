//! C API
//!
//! Note: `doc(cfg(...))` is not used in this module as it's not very
//! useful here.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
mod error;
mod intr;
mod mempool;
mod sync;
mod system;
mod task;
mod time;
mod types;
pub use self::{error::*, intr::*, mempool::*, sync::*, system::*, task::*, time::*, types::*};
