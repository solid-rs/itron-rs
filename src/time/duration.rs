use crate::abi;
use core::{convert::TryFrom, time::Duration as StdDuration};

use super::Timeout;

/// A valid relative time value ([`abi::RELTIM`]).
#[cfg_attr(
    feature = "nightly",
    doc = "[`duration!`] can be used to construct a `Duration` in a concise syntax."
)]
#[cfg_attr(
    not(feature = "nightly"),
    doc = "If `nightly` feature is enabled, \
    `duration!` can be used to construct a `Duration` in a concise syntax."
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Duration {
    value: abi::RELTIM,
}

impl Duration {
    /// The zero duration, which causes polling.
    // Safety: It's a valid duration value
    pub const ZERO: Self = unsafe { Self::from_raw(0) };

    /// Construct a new `Duration` from a raw value.
    ///
    /// # Safety
    ///
    /// `value` must be a valid duration value. This crate treats `E_PAR` caused
    /// by invalid duration values as a [critical error].
    ///
    /// [critical error]: crate::error
    #[inline]
    pub const unsafe fn from_raw(value: abi::RELTIM) -> Self {
        Self { value }
    }

    #[inline]
    pub const fn as_raw(self) -> abi::RELTIM {
        self.value
    }

    /// Construct a new `Duration` from the specified number of seconds.
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

    /// Construct a new `Duration` from the specified number of milliseconds.
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

    /// Construct a new `Duration` from the specified number of microseconds.
    ///
    /// Returns `None` if the specified duration is not representable.
    #[inline]
    pub const fn from_micros(micros: u64) -> Option<Self> {
        match () {
            () => {
                if micros > abi::TMAX_RELTIM as u64 {
                    None
                } else {
                    // Safety: It's a valid duration value
                    Some(unsafe { Self::from_raw(micros as u32) })
                }
            }
        }
    }

    /// Construct a new `Duration` from the specified number of nanoseconds.
    ///
    /// Returns `None` if the specified duration is not representable.
    #[inline]
    pub fn from_nanos(nanos: u128) -> Option<Self> {
        // TODO: make it `const fn`
        u64::try_from(nanos / 1000).ok().and_then(Self::from_micros)
    }
}

impl TryFrom<StdDuration> for Duration {
    type Error = super::TryFromDurationError;

    #[inline]
    fn try_from(d: StdDuration) -> Result<Self, Self::Error> {
        Self::from_nanos(d.as_nanos()).ok_or(super::TryFromDurationError(()))
    }
}

impl TryFrom<Timeout> for Duration {
    type Error = super::TryFromDurationError;

    #[inline]
    fn try_from(d: Timeout) -> Result<Self, Self::Error> {
        match () {
            () => {
                // In TOPPERS 3rd gen kernel, both types use the same range
                if d.is_finite() {
                    // Safety: It's a valid timeout value
                    Ok(unsafe { Self::from_raw(d.as_raw()) })
                } else {
                    Err(super::TryFromDurationError(()))
                }
            }
        }
    }
}

/// Construct a [`Duration`] value in a concise syntax. Panics if the specified
/// duration cannot be represented by `Duration`.
///
/// # Examples
///
/// ```
/// use itron::time::{duration, Duration};
/// assert_eq!(Duration::ZERO, duration!(0));
/// assert_eq!(Duration::from_millis(42).unwrap(), duration!(ms: 42));
/// ```
///
/// Panics if the value is out of range:
///
/// ```should_panic
/// # use itron::time::duration;
/// let _ = duration!(s: 0x7ffffffffffffff * 2);
/// ```
///
/// Once [`inline_const`] lands, it will be possible to do the check at
/// compile-time:
///
/// ```compile_fail
/// #![feature(inline_const)]
/// # use itron::time::duration;
/// let _ = const { duration!(s: 0x7ffffffffffffff * 2) };
/// ```
///
/// Literal values are validated at compile-time regardless of whether
/// `const { ... }` is used or not:
///
/// ```compile_fail
/// # use itron::time::duration;
/// let _ = duration!(s: 0xfffffffffffffff);
/// ```
///
/// ```should_panic
/// # use itron::time::duration;
/// // Wrap the expression with `( ... )` to avoid the above behavior and
/// // cause a runtime panic.
/// let _ = duration!(s: (0xfffffffffffffff));
/// ```
///
/// [`inline_const`]: https://rust-lang.github.io/rfcs/2920-inline-const.html
#[cfg(feature = "nightly")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "nightly")))]
pub macro duration {
    // Compile-time checked literals
    ($kind:tt: $value:literal) => {{
        const VALUE: $crate::time::Duration = $crate::time::duration!($kind: ($value));
        VALUE
    }},

    // Seconds
    (s: $value:expr) => {
        $crate::time::expect_valid_duration($crate::time::Duration::from_secs($value))
    },
    // Milliseconds
    (ms: $value:expr) => {
        $crate::time::expect_valid_duration($crate::time::Duration::from_millis($value))
    },
    // Microseconds
    (us: $value:expr) => {
        $crate::time::expect_valid_duration($crate::time::Duration::from_micros($value))
    },
    // Microseconds
    (μs: $value:expr) => {
        $crate::time::expect_valid_duration($crate::time::Duration::from_micros($value))
    },
    // Nanoseconds
    (ns: $value:expr) => {
        $crate::time::expect_valid_duration($crate::time::Duration::from_nanos($value))
    },

    // Zero
    (0) => { $crate::time::Duration::ZERO },
}

/// Panics if the specified `Option<Duration>` is `None`. Used by `duration!`.
#[cfg(feature = "nightly")]
#[doc(hidden)]
#[track_caller]
#[inline]
pub const fn expect_valid_duration(x: Option<Duration>) -> Duration {
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
