//! Semaphores
use core::{fmt, marker::PhantomData, mem::MaybeUninit};

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
    time::Timeout,
    wait::QueueOrder,
};

define_error_kind! {
    /// Error type for [`SemaphoreRef::signal`].
    pub enum SignalError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        QueueOverflow,
    }
}

impl ErrorKind for SignalError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_QOVR => Some(Self::QueueOverflow(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`SemaphoreRef::wait`].
    pub enum WaitError {
        #[cfg(not(feature = "none"))]
        BadContext,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        Released,
        #[cfg(not(feature = "none"))]
        TerminateRequest,
        #[cfg(all(not(feature = "none"), feature = "dcre"))]
        Deleted,
    }
}

impl ErrorKind for WaitError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RLWAI => Some(Self::Released(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RASTER => Some(Self::TerminateRequest(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "dcre"))]
            abi::E_DLT => Some(Self::Deleted(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`SemaphoreRef::wait_timeout`].
    pub enum WaitTimeoutError {
        #[cfg(not(feature = "none"))]
        BadContext,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        Timeout,
        #[cfg(not(feature = "none"))]
        Released,
        #[cfg(not(feature = "none"))]
        TerminateRequest,
        #[cfg(all(not(feature = "none"), feature = "dcre"))]
        Deleted,
    }
}

impl ErrorKind for WaitTimeoutError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // E_PAR is considered critial, hence excluded
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_TMOUT => Some(Self::Timeout(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RLWAI => Some(Self::Released(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RASTER => Some(Self::TerminateRequest(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "dcre"))]
            abi::E_DLT => Some(Self::Deleted(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`SemaphoreRef::poll`].
    pub enum PollError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for PollError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`SemaphoreRef::initialize`].
    pub enum InitializeError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for InitializeError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`SemaphoreRef::info`].
    pub enum InfoError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for InfoError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`Semaphore::build`].
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub enum BuildError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(any())]
        AccessDenied,
        /// Ran out of semaphore IDs.
        #[cfg(not(feature = "none"))]
        OutOfMemory,
        /// Bad parameter.
        #[cfg(not(feature = "none"))]
        BadParam,
    }
}

#[cfg(feature = "dcre")]
impl ErrorKind for BuildError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // `E_MACV` is considered critical, hence excluded
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_NOID => Some(Self::OutOfMemory(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR | abi::E_RSATR => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`SemaphoreRef::delete`].
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub enum DeleteError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        BadState,
    }
}

#[cfg(feature = "dcre")]
impl ErrorKind for DeleteError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadState(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

/// Semaphore count.
pub type Count = u32;

/// Semaphore information returned by [`SemaphoreRef::info`].
#[derive(Debug, Clone, Copy)]
pub struct Info {
    #[cfg(not(feature = "none"))]
    raw: abi::T_RSEM,
}

impl Info {
    /// Get the semaphore's count.
    #[inline]
    pub fn count(&self) -> Count {
        match () {
            #[cfg(not(feature = "none"))]
            () => self.raw.semcnt,
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// Get the first waiting task's ID.
    #[inline]
    pub fn first_waiting_task_id(&self) -> Option<abi::NonNullID> {
        match () {
            #[cfg(not(feature = "none"))]
            () => abi::NonNullID::new(self.raw.wtskid),
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// A borrowed reference to a semaphore.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SemaphoreRef<'a> {
    id: abi::NonNullID,
    _phantom: PhantomData<&'a ()>,
}

impl fmt::Debug for SemaphoreRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Semaphore({})", self.id)
    }
}

/// # Object ID conversion
impl SemaphoreRef<'_> {
    /// Construct a `SemaphoreRef` from a raw object ID.
    ///
    /// # Safety
    ///
    /// See [Object ID Wrappers](crate#object-id-wrappers).
    #[inline]
    pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    /// Get the raw object ID.
    #[inline]
    pub const fn as_raw(self) -> abi::ID {
        self.id.get()
    }

    /// Get the raw object ID as [` abi::NonNullID`].
    #[inline]
    pub const fn as_raw_nonnull(self) -> abi::NonNullID {
        self.id
    }
}

/// # Management
impl SemaphoreRef<'_> {
    /// `del_sem`: Delete the semaphore.
    #[inline]
    #[doc(alias = "del_sem")]
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub unsafe fn delete(self) -> Result<(), Error<DeleteError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::del_sem(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ref_sem`: Get the semaphore's general information.
    #[inline]
    #[doc(alias = "ref_sem")]
    pub fn info(self) -> Result<Info, Error<InfoError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::ref_sem(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(Info {
                    raw: pri.assume_init(),
                })
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// # Semaphore Operations
impl SemaphoreRef<'_> {
    /// `sig_sem`: Increment the semaphore count by one.
    #[inline]
    #[doc(alias = "sig_sem")]
    pub fn signal(self) -> Result<(), Error<SignalError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::sig_sem(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `wai_sem`: Decrement the semaphore count by one. Blocks the current
    /// task if the new value is negative.
    #[inline]
    #[doc(alias = "wai_sem")]
    pub fn wait(self) -> Result<(), Error<WaitError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::wai_sem(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `twai_sem`: Decrement the semaphore count by one. Blocks the current
    /// task with timeout if the new value is negative.
    #[inline]
    #[doc(alias = "twai_sem")]
    pub fn wait_timeout(self, tmo: Timeout) -> Result<(), Error<WaitTimeoutError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::twai_sem(self.as_raw(), tmo.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `pol_sem`: Decrement the semaphore count by one. Fails and returns
    /// immediately if the new value is negative.
    #[inline]
    #[doc(alias = "pol_sem")]
    pub fn poll(self) -> Result<(), Error<PollError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::pol_sem(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ini_sem`: Initialize the semaphore.
    #[inline]
    #[doc(alias = "ini_sem")]
    pub fn initialize(self) -> Result<(), Error<InitializeError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::ini_sem(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

#[cfg(feature = "dcre")]
pub use self::owned::*;

#[cfg(feature = "dcre")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
mod owned {
    use super::*;

    /// The builder type for [semaphores](Semaphore). Created by [`Semaphore::build`].
    ///
    /// Its generic parameters are an implementation detail.
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct Builder {
        initial_default: bool,
        #[cfg(not(feature = "none"))]
        raw: abi::T_CSEM,
    }

    impl Semaphore {
        /// `acre_sem`: Create a builder for `Semaphore`.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use itron::semaphore::Semaphore;
        /// let binary_semaphore = Semaphore::build()
        ///     .finish()
        ///     .expect("failed to create a semaphore");
        ///
        /// binary_semaphore.as_ref().wait()
        ///    .expect("failed to perform a wait operation");
        /// binary_semaphore.as_ref().signal()
        ///    .expect("failed to perform a signal operation");
        /// ```
        ///
        /// ```rust,no_run
        /// use itron::{semaphore::Semaphore, wait::QueueOrder};
        /// let counting_semaphore = Semaphore::build()
        ///     .initial_count(4)
        ///     .max_count(8)
        ///     .queue_order(QueueOrder::TaskPriority)
        ///     .finish()
        ///     .expect("failed to create a semaphore");
        ///
        /// for _ in 0..4 {
        ///     counting_semaphore.as_ref().poll()
        ///        .expect("failed to perform a polling wait operation");
        /// }
        /// counting_semaphore.as_ref().poll()
        ///    .expect_err("unexpectedly succeeded to perform a polling wait operation");
        ///
        /// for _ in 0..8 {
        ///     counting_semaphore.as_ref().signal()
        ///        .expect("failed to perform a signal operation");
        /// }
        /// counting_semaphore.as_ref().signal()
        ///    .expect_err("unexpectedly succeeded to perform a signal operation");
        /// ```
        #[inline]
        #[doc(alias = "acre_sem")]
        pub fn build() -> Builder {
            Builder {
                initial_default: true,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CSEM {
                    sematr: abi::TA_NULL,
                    isemcnt: 0,
                    maxsem: 1,
                },
            }
        }
    }

    impl Builder {
        /// Specify the initial count. Defaults to `max_count` when unspecified.
        #[inline]
        pub fn initial_count(self, value: Count) -> Builder {
            Builder {
                initial_default: false,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CSEM {
                    isemcnt: value,
                    ..self.raw
                },
            }
        }

        /// Specify the maximum count. Defaults to `1` when unspecified.
        #[inline]
        pub fn max_count(self, value: Count) -> Builder {
            Builder {
                #[cfg(not(feature = "none"))]
                raw: abi::T_CSEM {
                    maxsem: value,
                    ..self.raw
                },
                ..self
            }
        }

        /// Specify the queue order. Defaults to `Fifo` when unspecified.
        #[inline]
        pub fn queue_order(self, value: QueueOrder) -> Builder {
            Builder {
                #[cfg(not(feature = "none"))]
                raw: abi::T_CSEM {
                    sematr: value.as_raw_atr(),
                    ..self.raw
                },
                ..self
            }
        }
    }

    impl Builder {
        /// Create a semaphore using the specified parameters.
        pub fn finish(mut self) -> Result<Semaphore, Error<BuildError>> {
            #[cfg(not(feature = "none"))]
            if self.initial_default {
                self.raw.isemcnt = self.raw.maxsem;
            }
            match () {
                #[cfg(not(feature = "none"))]
                () => unsafe {
                    let id = Error::err_if_negative(abi::acre_sem(&self.raw))?;
                    // Safety: We own the semaphore we create
                    Ok(Semaphore::from_raw_nonnull(abi::NonNullID::new_unchecked(
                        id,
                    )))
                },
                #[cfg(feature = "none")]
                () => unimplemented!(),
            }
        }
    }

    /// An owned semaphore.
    ///
    /// [Deletes] the semaphore automatically when dropped. The destructor will
    /// panic if the deletion fails.
    ///
    /// [Deletes]: SemaphoreRef::delete
    #[derive(PartialEq, Eq)]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct Semaphore(SemaphoreRef<'static>);

    impl fmt::Debug for Semaphore {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Drop for Semaphore {
        #[inline]
        fn drop(&mut self) {
            unsafe { self.0.delete().unwrap() };
        }
    }

    impl Semaphore {
        /// Construct a `Semaphore` from a raw object ID.
        ///
        /// # Safety
        ///
        /// See [Object ID Wrappers](crate#object-id-wrappers).
        #[inline]
        pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
            Self(unsafe { SemaphoreRef::from_raw_nonnull(id) })
        }

        /// Consume and "leak" `self`, returning a reference `SemaphoreRef<'a>`.
        #[inline]
        pub const fn leak<'a>(self) -> SemaphoreRef<'a> {
            let out = self.0;
            core::mem::forget(self);
            out
        }

        /// Get the raw object ID.
        #[inline]
        pub const fn as_raw(&self) -> abi::ID {
            self.0.as_raw()
        }

        /// Get the raw object ID as [` abi::NonNullID`].
        #[inline]
        pub const fn as_raw_nonnull(&self) -> abi::NonNullID {
            self.0.as_raw_nonnull()
        }

        /// Borrow `Semaphore` as [`SemaphoreRef`].
        ///
        /// Use this to perform operations on semaphores because most of the
        /// methods are implemented on `SemaphoreRef` but not `Semaphore`.
        #[inline]
        pub const fn as_ref(&self) -> SemaphoreRef<'_> {
            self.0
        }
    }
}
