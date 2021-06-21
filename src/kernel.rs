//! Miscellaneous functions that are not associated to specific kernel objects.
#[cfg(any())]
use crate::error::Kind;
use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind},
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

// TODO: rot_rdq
// TODO: mrot_rdq
// TODO: get_lod
// TODO: mget_lod
// TODO: get_nth
// TODO: mget_nth
// TODO: loc_cpu
// TODO: unl_cpu
// TODO: dis_dsp
// TODO: ena_dsp

/// `sns_ctx`: Get a flag indicating whether the current thread is in a task
/// context.
#[inline]
#[doc(alias = "sns_ctx")]
pub fn is_task_context() -> bool {
    match () {
        #[cfg(not(feature = "none"))]
        () => (unsafe { abi::sns_ctx() } == 0),
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `sns_loc`: Get a flag indicating whether the CPU lock state is active.
#[inline]
#[doc(alias = "sns_loc")]
pub fn is_cpu_lock_active() -> bool {
    match () {
        #[cfg(not(feature = "none"))]
        () => (unsafe { abi::sns_loc() } != 0),
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `sns_dsp`: Get a flag indicating whether dispatching is disabled.
#[inline]
#[doc(alias = "sns_dsp")]
pub fn is_dispatching_disabled() -> bool {
    match () {
        #[cfg(not(feature = "none"))]
        () => (unsafe { abi::sns_dsp() } != 0),
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `sns_dpn`: Get a flag indicating whether the dispatch pending state is
/// active.
#[inline]
#[doc(alias = "sns_dpn")]
pub fn is_dispatch_pending_active() -> bool {
    match () {
        #[cfg(not(feature = "none"))]
        () => (unsafe { abi::sns_dpn() } != 0),
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `sns_ker`: Get a flag indicating whether the kernel is in an operational
/// state.
///
/// If this function returns `false`, all kernel API functions except for
/// `sns_ker` are unsafe to call.
#[inline]
#[doc(alias = "sns_ker")]
pub fn is_operational() -> bool {
    match () {
        #[cfg(not(feature = "none"))]
        () => (unsafe { abi::sns_ker() } == 0),
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
