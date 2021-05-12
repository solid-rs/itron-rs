use crate::abi;
use core::{convert::TryFrom, time::Duration as StdDuration};

use super::Duration;

/// A valid timeout value ([`abi::TMO`]).
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

    #[inline]
    pub const fn is_finite(self) -> bool {
        self.value != Self::FOREVER.value
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
pub struct TryFromDurationError(pub(super) ());

impl TryFrom<StdDuration> for Timeout {
    type Error = TryFromDurationError;

    #[inline]
    fn try_from(d: StdDuration) -> Result<Self, Self::Error> {
        Self::from_nanos(d.as_nanos()).ok_or(TryFromDurationError(()))
    }
}

impl TryFrom<Duration> for Timeout {
    type Error = TryFromDurationError;

    #[inline]
    fn try_from(d: Duration) -> Result<Self, Self::Error> {
        match () {
            () => {
                // In TOPPERS 3rd gen kernel, both types use the same range
                // Safety: It's a valid timeout value
                Ok(unsafe { Self::from_raw(d.as_raw()) })
            }
        }
    }
}

/// Construct a [`Timeout`] value in a concise syntax. Panics if the specified
/// duration cannot be represented by `Timeout`.
///
/// # Examples
///
/// ```
/// use itron::time::{timeout, Timeout};
/// assert_eq!(Timeout::ZERO, timeout!(0));
/// assert_eq!(Timeout::FOREVER, timeout!(infinity));
/// assert_eq!(Timeout::from_millis(42).unwrap(), timeout!(ms: 42));
/// ```
///
/// Panics if the value is out of range:
///
/// ```should_panic
/// # use itron::time::timeout;
/// let _ = timeout!(s: 0x7ffffffffffffff * 2);
/// ```
///
/// Once [`inline_const`] lands, it will be possible to do the check at
/// compile-time:
///
/// ```compile_fail
/// #![feature(inline_const)]
/// # use itron::time::timeout;
/// let _ = const { timeout!(s: 0x7ffffffffffffff * 2) };
/// ```
///
/// Literal values are validated at compile-time regardless of whether
/// `const { ... }` is used or not:
///
/// ```compile_fail
/// # use itron::time::timeout;
/// let _ = timeout!(s: 0xfffffffffffffff);
/// ```
///
/// ```should_panic
/// # use itron::time::timeout;
/// // Wrap the expression with `( ... )` to avoid the above behavior and
/// // cause a runtime panic.
/// let _ = timeout!(s: (0xfffffffffffffff));
/// ```
///
/// [`inline_const`]: https://rust-lang.github.io/rfcs/2920-inline-const.html
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "nightly")))]
pub macro timeout {
    // Compile-time checked literals
    ($kind:tt: $value:literal) => {{
        const VALUE: $crate::time::Timeout = $crate::time::timeout!($kind: ($value));
        VALUE
    }},

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
    // Microseconds
    (μs: $value:expr) => {
        $crate::time::expect_valid_timeout($crate::time::Timeout::from_micros($value))
    },
    // Nanoseconds
    (ns: $value:expr) => {
        $crate::time::expect_valid_timeout($crate::time::Timeout::from_nanos($value))
    },

    // Infinity
    (infinity) => { $crate::time::Timeout::FOREVER },

    // Zero
    (0) => { $crate::time::Timeout::ZERO },
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
