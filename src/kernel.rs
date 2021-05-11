//! Miscellaneous functions that are not associated to specific kernel objects.
use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
};

define_error_kind! {
    /// Error type for [`exit`].
    pub enum ExitError {
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for ExitError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // `E_SYS` is a critical error, so it's excluded from here
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

/// `sns_ker`: Get a flag indicating whether the kernel is in an operational
/// state.
///
/// If this function returns `false`, all kernel API functions except for
/// `sns_ker` are unsafe to call.
#[inline]
#[doc(alias = "sns_ker")]
pub fn is_kernel_operational() -> bool {
    match () {
        #[cfg(not(feature = "none"))]
        () => (unsafe { abi::sns_key() } == 0),
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `ext_ker`: Terminate the kernel.
///
/// This function will not return if it succeeds.
#[inline]
#[doc(alias = "ext_ker")]
pub fn exit() -> Error<ExitError> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe { Error::new_unchecked(ErrorCode::new_unchecked(abi::ext_ker())) },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}
