#![no_std]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![cfg_attr(feature = "nightly", feature(decl_macro))]
#![doc = include_str!("lib.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::match_single_binding)] // the `cfg` matching pattern

/// Changelog (`CHANGELOG.md`)
///
#[doc = include_str!("../CHANGELOG.md")]
pub mod _changelog_ {}

pub mod abi;

macro_rules! unstable_module {
    {$(
        $( #[macro_use $($unused:tt)*] )*
        $( #[doc = $doc:tt] )*
        $( #[cfg( $($cfg:tt)* )] )?
        pub mod $name:ident $semicolon_or_brace:tt
    )*} => {$(
        $( #[macro_use $($unused)*] )*
        $( #[doc = $doc] )*
        #[cfg(all(feature = "unstable", $($($cfg)*)?))]
        #[cfg_attr(
            feature = "doc_cfg",
            doc(cfg(all(feature = "unstable", $($($cfg)*)?)))
        )]
        pub mod $name $semicolon_or_brace
    )*};
}

unstable_module! {
    #[macro_use]
    pub mod error;
    pub mod closure;
    pub mod dataqueue;
    pub mod eventflag;
    pub mod interrupt;
    pub mod kernel;
    pub mod memorypool;
    #[cfg(all(feature = "asp3", feature = "messagebuf"))]
    pub mod messagebuffer;
    pub mod mutex;
    pub mod prioritydataqueue;
    pub mod semaphore;
    pub mod task;
    pub mod wait;

    /// Temporal quantification
    pub mod time {
        mod duration;
        mod systime;
        mod timeout;
        pub use self::{duration::*, systime::*, timeout::*};
        // `use ::*` doesn't work with `pub macro`. This could be a bug.
        #[cfg(feature = "nightly")]
        pub use self::{duration::duration, timeout::timeout};
    }
}
