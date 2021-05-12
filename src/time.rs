//! Temporal quantification
use crate::abi;
use core::{convert::TryFrom, time::Duration as StdDuration};

/// A valid timeout value.
///
/// In addition to finite durations, this type can represent the following
/// special values:
/// [`ZERO`] indicating zero or polling and [`FOREVER`] representing an
/// infinite duration. **`TMO_NBLK` is not a valid value for this type.**
///
/// [`ZERO`]: Self::ZERO
/// [`FOREVER`]: Self::FOREVER
#[cfg_attr(
    feature = "nightly",
    doc = "[`timeout!`] can be used to construct a `Timeout` in a concise syntax."
)]
#[cfg_attr(
    not(feature = "nightly"),
    doc = "If `nightly` feature is enabled, \
    `timeout!` can be used to construct a `Timeout` in a concise syntax."
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Timeout {
    value: abi::TMO,
}

impl Timeout {
    /// The zero timeout value, which causes polling.
    // Safety: It's a valid timeout value
    pub const ZERO: Self = unsafe { Self::from_raw(abi::TMO_POL) };

    /// The infinite timeout value.
    // Safety: It's a valid timeout value
    pub const FOREVER: Self = unsafe { Self::from_raw(abi::TMO_FEVR) };

    /// Construct a new `Timeout` from a raw value.
    ///
    /// # Safety
    ///
    /// `value` must be a valid timeout value. This crate treats `E_PAR` caused
    /// by invalid timeout values as a [critical error].
    ///
    /// [critical error]: crate::error
    #[inline]
    pub const unsafe fn from_raw(value: abi::TMO) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn as_raw(self) -> abi::TMO {
        self.value
    }

    /// Construct a new `Timeout` from the specified number of seconds.
    ///
    /// Returns `None` if the specified duration is not representable.
    #[inline]
    pub const fn from_secs(secs: u64) -> Option<Self> {
        if secs > u64::MAX / 1000_000 {
            None
        } else {
            Self::from_micros(secs * 1000_000)
        }
    }

    /// Construct a new `Timeout` from the specified number of milliseconds.
    ///
    /// Returns `None` if the specified duration is not representable.
    #[inline]
    pub const fn from_millis(millis: u64) -> Option<Self> {
        if millis > u64::MAX / 1000 {
            None
        } else {
            Self::from_micros(millis * 1000)
        }
    }

    /// Construct a new `Timeout` from the specified number of microseconds.
    ///
    /// Returns `None` if the specified duration is not representable.
    #[inline]
    pub const fn from_micros(micros: u64) -> Option<Self> {
        match () {
            () => {
                if micros > abi::TMAX_RELTIM as u64 {
                    None
                } else {
                    // Safety: It's a valid timeout value
                    Some(unsafe { Self::from_raw(micros as u32) })
                }
            }
        }
    }

    /// Construct a new `Timeout` from the specified number of nanoseconds.
    ///
    /// Returns `None` if the specified duration is not representable.
    #[inline]
    pub fn from_nanos(nanos: u128) -> Option<Self> {
        // TODO: make it `const fn`
        u64::try_from(nanos / 1000).ok().and_then(Self::from_micros)
    }
}

/// The error type returned when a checked duration conversion fails.
pub struct TryFromDurationError(());

impl TryFrom<StdDuration> for Timeout {
    type Error = TryFromDurationError;

    #[inline]
    fn try_from(d: StdDuration) -> Result<Self, Self::Error> {
        Self::from_nanos(d.as_nanos()).ok_or(TryFromDurationError(()))
    }
}

/// Construct a [`Timeout`] value in a concise syntax. Panics if the specified
/// duration cannot be represented by `Timeout`.
///
/// # Examples
///
/// ```
/// use itron::time::{timeout, Timeout};
/// assert_eq!(Timeout::from_millis(42).unwrap(), timeout!(ms: 42));
/// ```
///
/// Panics if the value is out of range:
///
/// ```should_panic
/// let _ = itron::time::timeout!(s: 0xffffffffffffffff);
/// ```
///
/// Once [`inline_const`] lands, it will be possible to do the check at
/// compile-time:
///
/// ```compile_fail
/// #![feature(inline_const)]
/// let _ = const { itron::time::timeout!(s: 0xffffffffffffffff) };
/// ```
///
/// [`inline_const`]: https://rust-lang.github.io/rfcs/2920-inline-const.html
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "nightly")))]
pub macro timeout {
    // Seconds
    (s: $value:expr) => {
        $crate::time::expect_valid_timeout($crate::time::Timeout::from_secs($value))
    },
    // Milliseconds
    (ms: $value:expr) => {
        $crate::time::expect_valid_timeout($crate::time::Timeout::from_millis($value))
    },
    // Microseconds
    (us: $value:expr) => {
        $crate::time::expect_valid_timeout($crate::time::Timeout::from_micros($value))
    },
}

/// Panics if the specified `Option<Timeout>` is `None`. Used by `timeout!`.
#[cfg(feature = "nightly")]
#[doc(hidden)]
#[track_caller]
#[inline]
pub const fn expect_valid_timeout(x: Option<Timeout>) -> Timeout {
    if let Some(x) = x {
        x
    } else {
        // Panics in `const fn` are unstable at the point of writing
        let zero = 0u32;
        #[allow(unconditional_panic)]
        let __the_specified_timeout_is_invalid_or_not_representable__ = 1 / zero;
        loop {}
    }
}
