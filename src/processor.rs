//! Multiprocessing
#[allow(unused_imports)]
use core::{convert::TryFrom, fmt, mem::MaybeUninit};

#[allow(unused_imports)]
use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
};

define_error_kind! {
    /// Error type for [`current`].
    pub enum CurrentIdError {
        /// The CPU lock state is active.
        #[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
        BadContext,
    }
}

impl ErrorKind for CurrentIdError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

/// Refers to a single processor in a multi-processor system. The stored
/// processor ID is not guaranteed to be valid but is guaranteed to be non-null.
///
/// In a uniprocessor kernel, this is a zero-sized type.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Processor {
    #[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
    raw: abi::NonNullID,
    _private: (),
}

impl fmt::Debug for Processor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Processor({:?})", self.as_raw())
    }
}

#[cfg(not(any(feature = "fmp3", feature = "solid_fmp3")))]
#[cfg_attr(
    feature = "doc_cfg",
    doc(cfg(not(any(feature = "fmp3", feature = "solid_fmp3"))))
)]
impl Processor {
    /// The only procesor in a uniprocessor system.
    pub const UNIPROCESSOR: Self = Self { _private: () };

    /// Used by the `Debug` impl
    #[cfg(not(feature = "none"))]
    fn as_raw(self) -> () {
        ()
    }
}

#[cfg(any(feature = "fmp3", feature = "solid_fmp3", feature = "none"))]
#[cfg_attr(
    feature = "doc_cfg",
    doc(cfg(any(feature = "fmp3", feature = "solid_fmp3")))
)]
impl Processor {
    /// Construct `Processor` from a raw processor ID.
    #[inline]
    pub const fn from_raw(raw: abi::ID) -> Option<Self> {
        // `map` is not `const fn` yet
        if let Some(raw) = abi::NonNullID::new(raw) {
            Some(Self::from_raw_nonnull(raw))
        } else {
            None
        }
    }

    /// Construct `Processor` from a non-null raw processor ID.
    #[inline]
    pub const fn from_raw_nonnull(raw: abi::NonNullID) -> Self {
        match () {
            #[cfg(feature = "none")]
            () => {
                let _ = raw;
                Self::UNIPROCESSOR
            }
            #[cfg(not(feature = "none"))]
            () => Self { raw, _private: () },
        }
    }

    /// Get a raw processor ID.
    #[inline]
    pub const fn as_raw(self) -> abi::ID {
        self.as_raw_nonnull().get()
    }

    /// Get a raw processor ID as [`abi::NonNullID`].
    #[inline]
    pub const fn as_raw_nonnull(self) -> abi::NonNullID {
        match () {
            #[cfg(feature = "none")]
            () => unsafe { abi::NonNullID::new_unchecked(1) },
            #[cfg(not(feature = "none"))]
            () => self.raw,
        }
    }
}

/// The error type returned when a conversion from `usize` to [`Processor`]
/// fails.
///
/// This can occur because of a number of reasons:
///
///  - The specified value is zero, which represents a null value.
///
///  - The specified value does not fit in [`abi::ID`].
///
///  - The target kernel does not support multiple processors, and the supplied
///    value is not `1`.
///
/// Note that an attempt to create a `Processor` representing a non-existent
/// processor is not guaranteed to fail.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProcessorTryFromError(());

impl TryFrom<usize> for Processor {
    type Error = ProcessorTryFromError;

    #[cfg(not(any(feature = "fmp3", feature = "solid_fmp3")))]
    #[inline]
    fn try_from(x: usize) -> Result<Self, Self::Error> {
        if x == 1 {
            Ok(Self::UNIPROCESSOR)
        } else {
            Err(ProcessorTryFromError(()))
        }
    }

    #[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
    #[inline]
    fn try_from(x: usize) -> Result<Self, Self::Error> {
        Self::from_raw(abi::ID::try_from(x).map_err(|_| ProcessorTryFromError(()))?)
            .ok_or(ProcessorTryFromError(()))
    }
}

/// `get_pid`: Get the current processor's ID.
#[inline]
#[doc(alias = "get_pid")]
pub fn current() -> Result<Processor, Error<CurrentIdError>> {
    match () {
        #[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
        () => unsafe {
            let mut out = MaybeUninit::uninit();
            Error::err_if_negative(abi::get_pid(out.as_mut_ptr()))?;
            Ok(Processor::from_raw_nonnull(abi::NonNullID::new_unchecked(
                out.assume_init(),
            )))
        },

        #[cfg(not(any(feature = "fmp3", feature = "solid_fmp3", feature = "none")))]
        () => {
            // Uniprocessor
            Ok(Processor::UNIPROCESSOR)
        }

        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}
