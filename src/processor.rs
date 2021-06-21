//! Multiprocessing
use core::mem::MaybeUninit;

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
};

define_error_kind! {
    /// Error type for [`current`].
    pub enum CurrentIdError {
        /// The CPU lock state is active.
        #[cfg(not(feature = "none"))]
        BadContext,
    }
}

impl ErrorKind for CurrentIdError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

/// Refers to a single processor in a multi-processor system. The stored
/// processor ID is not guaranteed to be valid but is guaranteed to be non-null.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Processor {
    raw: abi::NonNullID,
}

impl Processor {
    /// Construct `Processor` from a raw processor ID.
    #[inline]
    pub const fn from_raw_nonnull(raw: abi::NonNullID) -> Self {
        Self { raw }
    }

    /// Get a raw processor ID.
    #[inline]
    pub const fn as_raw(self) -> abi::ID {
        self.raw.get()
    }

    /// Get a raw processor ID as [`abi::NonNullID`].
    #[inline]
    pub const fn as_raw_nonnull(self) -> abi::NonNullID {
        self.raw
    }
}

/// `get_pid`: Get the current processor's ID.
#[inline]
#[doc(alias = "get_pid")]
pub fn current() -> Result<Option<abi::NonNullID>, Error<CurrentIdError>> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe {
            let mut out = MaybeUninit::uninit();
            Error::err_if_negative(abi::get_pid(out.as_mut_ptr()))?;
            Ok(abi::NonNullID::new(out.assume_init()))
        },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}
