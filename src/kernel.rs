//! Miscellaneous functions that are not associated to specific kernel objects.
use crate::abi;

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
