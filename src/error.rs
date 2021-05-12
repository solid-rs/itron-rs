//! Error types.
//!
//! Errors indicate exceptional outcomes of system calls. Like most
//! traditional operating systems, they are represented by error codes in
//! μITRON.
//!
//! # Error Kind Types
//!
//! TODO: describe error kind types
//!
//! # Critical Errors
//!
//! The following errors may be escalated to panics or undefined behaviors:
//!
//!  - Kernel integrity errors (some cases of `E_SYS` in the TOPPERS
//!    kernels).
//!
//!  - Memory access permission errors (`E_MACV`) caused by pointers that are
//!    supposed to be accessible by the current thread (e.g., pointers referring
//!    to local variables).
//!
//!  - `E_PAR` caused by invalid timeout values.
//!
use core::{fmt, marker::PhantomData, num::NonZeroIsize};

#[allow(unused_imports)]
use crate::abi;

/// Target-specific error value that can be categorized as one of the error
/// kinds represented by `Kind`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error<Kind = ()> {
    code: ErrorCode,
    _phantom: PhantomData<Kind>,
}

/// Trait for [error kind types].
///
/// [error kind types]: super#error-kind-types
pub trait ErrorKind: Copy {
    /// Categorize the specified error code.
    ///
    /// Returns `None` if the error code is invalid in this context.
    fn from_error_code(code: ErrorCode) -> Option<Self>;
}

impl<Kind: ErrorKind> Error<Kind> {
    /// Construct `Error`.
    ///
    /// # Safety
    ///
    ///  `Kind::from_error_code(code)` must return `Some(_)`. Otherwise,
    /// [`Self::kind`] will cause an undefined behavior.
    #[inline]
    pub unsafe fn new_unchecked(code: ErrorCode) -> Self {
        debug_assert!(Kind::from_error_code(code).is_some());

        Self {
            code,
            _phantom: PhantomData,
        }
    }

    /// Return `Ok(code)` if `code >= 0`; `Err(new_unchecked(code))` otherwise.
    ///
    /// # Safety
    ///
    /// See [`Self::new_unchecked`].
    #[inline]
    pub(crate) unsafe fn err_if_negative(code: isize) -> Result<isize, Self> {
        if let Some(e) = ErrorCode::new(code) {
            // Safety: Upheld by the caller
            Err(unsafe { Self::new_unchecked(e) })
        } else {
            Ok(code)
        }
    }

    /// Get the error kind.
    #[inline]
    pub fn kind(self) -> Kind {
        // Safety: Upheld by `new_unchecked`'s caller
        unsafe {
            Kind::from_error_code(self.code).unwrap_or_else(|| core::hint::unreachable_unchecked())
        }
    }

    /// Get the error code.
    #[inline]
    pub fn code(self) -> ErrorCode {
        self.code
    }
}

impl<Kind> fmt::Debug for Error<Kind> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.code, f)
    }
}

/// Raw error code.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ErrorCode(NonZeroIsize);

impl ErrorCode {
    /// Construct an `ErrorCode`.
    ///
    /// Returns `None` if the specified value is not negative.
    #[inline]
    pub const fn new(code: isize) -> Option<Self> {
        if code >= 0 {
            None
        } else {
            if let Some(x) = NonZeroIsize::new(code) {
                Some(Self(x))
            } else {
                None
            }
        }
    }

    /// Construct an `ErrorCode` without checking if `code` is a valid error
    /// code.
    //
    /// # Safety
    ///
    /// If `code` is not negative, this function causes an undefined
    /// behavior.
    #[inline]
    pub const unsafe fn new_unchecked(code: isize) -> Self {
        Self(unsafe { NonZeroIsize::new_unchecked(code) })
    }

    /// Get the numerical value.
    #[inline]
    pub const fn get(self) -> isize {
        self.0.get()
    }
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match () {
            #[cfg(feature = "asp3")]
            () => match self.get() {
                abi::E_SYS => Some("E_SYS"),
                abi::E_NOSPT => Some("E_NOSPT"),
                abi::E_RSFN => Some("E_RSFN"),
                abi::E_RSATR => Some("E_RSATR"),
                abi::E_PAR => Some("E_PAR"),
                abi::E_ID => Some("E_ID"),
                abi::E_CTX => Some("E_CTX"),
                abi::E_MACV => Some("E_MACV"),
                abi::E_OACV => Some("E_OACV"),
                abi::E_ILUSE => Some("E_ILUSE"),
                abi::E_NOMEM => Some("E_NOMEM"),
                abi::E_NOID => Some("E_NOID"),
                abi::E_NORES => Some("E_NORES"),
                abi::E_OBJ => Some("E_OBJ"),
                abi::E_NOEXS => Some("E_NOEXS"),
                abi::E_QOVR => Some("E_QOVR"),
                abi::E_RLWAI => Some("E_RLWAI"),
                abi::E_TMOUT => Some("E_TMOUT"),
                abi::E_DLT => Some("E_DLT"),
                abi::E_CLS => Some("E_CLS"),
                abi::E_RASTER => Some("E_RASTER"),
                abi::E_WBLK => Some("E_WBLK"),
                abi::E_BOVR => Some("E_BOVR"),
                abi::E_COMM => Some("E_COMM"),
                _ => None,
            },
            #[cfg(feature = "none")]
            () => None,
        };

        if let Some(name) = name {
            f.write_str(name)
        } else {
            write!(f, "{}", self.get())
        }
    }
}

impl<Kind: ErrorKind> From<Error<Kind>> for ErrorCode {
    #[inline]
    fn from(x: Error<Kind>) -> Self {
        x.code()
    }
}

/// Placeholder for error kind variants.
///
/// **Do not refer to this type in your code!**
/// This type exists purely for documentation purposes. This type is replaced
/// with the inhabited type [`Kind`] if the variant is valid for the current
/// target kernel or the uninhabited type [`Never`] otherwise. This technique
/// lets us match against error kinds that do not exist in some kernels without
/// causing a compilation error.
///
/// ```
/// #![feature(exhaustive_patterns)]
/// use itron::error::{Kind, Never};
///
/// enum ExampleError {
///     Error1(Kind),
///     Error2(Never),
///     // displayed as the following in the doc
///     //   Error1(MaybeKind),
///     //   Error2(MaybeKind),
/// }
///
/// # fn a(error: ExampleError) {
/// // Portable code that handles all kinds
/// match error {
///     // `_` = don't care; the arm is just ignored if the
///     //       error kind does not exist in this kernel
///     ExampleError::Error1(_) => println!("error1"),
///     ExampleError::Error2(_) => println!("error2"),
/// }
///
/// // Portable code that handles some kinds
/// match error {
///     ExampleError::Error2(_) => println!("error2"),
///     _ => println!("other"),
/// }
///
/// // Non-portable code that handles all kinds
/// match error {
///     // `Kind(_)` = assume that the error kind exists;
///     //             raise a compile error if this assumption is broken
///     ExampleError::Error1(Kind(_)) => println!("error1"),
///
///     // (no arm) = assume that the error kind does not exist;
///     //            raise a compile error if this assumption is broken
///     // (This requires `#[feature(exhaustive_patterns)]` for now.)
/// }
/// # }
/// ```
pub type MaybeKind = ();

/// Type for error kinds that are valid in the current target kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kind(pub __Unstable);

impl Kind {
    #[inline]
    pub(crate) fn from_error_code(_: ErrorCode) -> Self {
        Self(__Unstable)
    }
}

/// I haven't decided what to put in [`Kind`].
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct __Unstable;

/// Indicates that the error kind never occurs in the current target kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Never {}

/// An internal macro to define error kind types.
macro_rules! define_error_kind {
    (
        $( #[$meta:meta] )*
        pub enum $name:ident {
            $(
                $( #[doc = $doc:literal] )*
                #[cfg( $($cfg:tt)* )]
                $variant:ident
            ),*
            $(,)*
        }
    ) => {
        $( #[$meta] )*
        ///
        /// This type is an [error kind type].
        ///
        /// [error kind type]: crate::error#error-kind-types
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $(
                $( #[doc = $doc] )*
                // TODO: Replace this doc comment with `doc(cfg(...))`, which
                //       currently does not work
                #[doc = concat!("\n\n<i>Requires</i>: `cfg(", stringify!($($cfg)*), ")`")]
                #[cfg(doc)]
                $variant(crate::error::MaybeKind),

                #[cfg(all(not(doc), $($cfg)* ))]
                $variant(crate::error::Kind),

                #[cfg(all(not(doc), not( $($cfg)* )))]
                $variant(crate::error::Never),
            )*
        }
    };
}
