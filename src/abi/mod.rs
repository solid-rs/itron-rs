//! C API
//!
//! Note: `doc(cfg(...))` is not used in this module as it's not very
//! useful here.
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
mod error;
mod system;
mod task;
mod types;
pub use self::{error::*, system::*, task::*, types::*};

// TODO: TOPPERS/ASP3 時間管理機能
// TODO: TOPPERS/ASP3 メモリプール管理機能
// TODO: TOPPERS/ASP3 同期・通信機能
// TODO: TOPPERS/ASP3 タスク終了機能
// TODO: TOPPERS/ASP3 タスク付属同期機能
