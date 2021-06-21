//! C API
//!
//! Note: `doc(cfg(...))` is not used in this module as different kernels might
//! provide different items with conflicting names. This module's documentation
//! might not be useful unless you ran `cargo doc` with an appropriate kernel
//! selection.
//!
//! <i>This documentation has been built with the following Cargo features:
// Make sure the commas are inserted at the right places!
#![cfg_attr(feature = "asp3", doc = "`asp3`")]
#![cfg_attr(feature = "fmp3", doc = "`fmp3`")]
#![cfg_attr(feature = "solid_asp3", doc = "`solid_asp3`")]
#![cfg_attr(feature = "solid_fmp3", doc = "`solid_fmp3`")]
#![cfg_attr(feature = "none", doc = "`none`")]
#![cfg_attr(feature = "dcre", doc = ", `dcre`")]
#![cfg_attr(feature = "rstr_task", doc = ", `rstr_task`")]
#![cfg_attr(feature = "messagebuf", doc = ", `messagebuf`")]
#![cfg_attr(feature = "ovrhdr", doc = ", `ovrhdr`")]
#![cfg_attr(feature = "subprio", doc = ", `subprio`")]
#![cfg_attr(feature = "pi_mutex", doc = ", `pi_mutex`")]
//! </i>
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
