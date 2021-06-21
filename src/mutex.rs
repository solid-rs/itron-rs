//! Mutexes
use core::{fmt, marker::PhantomData, mem::MaybeUninit};

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
    time::Timeout,
};

define_error_kind! {
    /// Error type for [`MutexRef::lock`].
    pub enum LockError {
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
        /// The calling task's priority is higher than the mutex's priority
        /// ceiling.
        ///
        /// # Rationale
        ///
        /// The `EINVAL` error of `pthread_mutex_lock`. This error kind is
        /// designed to accomodate any precondition violations that may occur
        /// in yet-to-be-seen kernels to be supported.
        #[cfg(not(feature = "none"))]
        BadParam,
        /// The calling task already owns the mutex.
        #[cfg(not(feature = "none"))]
        Deadlock,
    }
}

impl ErrorKind for LockError {
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
            #[cfg(not(feature = "none"))]
            abi::E_ILUSE => Some(Self::BadParam(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::Deadlock(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MutexRef::lock_timeout`].
    pub enum LockTimeoutError {
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
        /// The calling task's priority is higher than the mutex's priority
        /// ceiling.
        ///
        /// # Rationale
        ///
        /// The `EINVAL` error of `pthread_mutex_lock`. This error kind is
        /// designed to accomodate any precondition violations that may occur
        /// in yet-to-be-seen kernels to be supported.
        #[cfg(not(feature = "none"))]
        BadParam,
        /// The calling task already owns the mutex.
        #[cfg(not(feature = "none"))]
        Deadlock,
    }
}

impl ErrorKind for LockTimeoutError {
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
            #[cfg(not(feature = "none"))]
            abi::E_ILUSE => Some(Self::BadParam(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::Deadlock(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MutexRef::try_lock`].
    pub enum TryLockError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        /// The calling task's priority is higher than the mutex's priority
        /// ceiling.
        ///
        /// # Rationale
        ///
        /// The `EINVAL` error of `pthread_mutex_lock`. This error kind is
        /// designed to accomodate any precondition violations that may occur
        /// in yet-to-be-seen kernels to be supported.
        #[cfg(not(feature = "none"))]
        BadParam,
        /// The calling task already owns the mutex.
        #[cfg(not(feature = "none"))]
        Deadlock,
    }
}

impl ErrorKind for TryLockError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ILUSE => Some(Self::BadParam(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::Deadlock(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MutexRef::unlock`].
    pub enum UnlockError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        /// The mutex is not the lastly-locked mutex currently owned by the
        /// calling task (TOPPERS third-generation kernels, `E_OBJ`). The mutex
        /// is not currently owned by the calling task (μITRON 4.0 and
        /// μT-Kernel, `E_ILUSE`).
        ///
        /// # Rationale
        ///
        /// The name was inspired by the FTP and SMTP error 503 (bad sequence of
        /// commands) and SOLID `SOLID_ERR_BADSEQUENCE`. A mutex is intended
        /// to be used in a specific sequence (lock followed by unlock). The
        /// TOPPERS third-generation kernels impose a more stringent requirement
        /// on the sequence: mutexes must be unlocked in a lock-reverse order.
        #[cfg(not(feature = "none"))]
        BadSequence,
    }
}

impl ErrorKind for UnlockError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadSequence(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MutexRef::initialize`].
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
    /// Error type for [`MutexRef::info`].
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
    /// Error type for [`Mutex::build`].
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub enum BuildError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(any())]
        AccessDenied,
        /// Ran out of mutex IDs.
        #[cfg(not(feature = "none"))]
        OutOfMemory,
        /// Bad parameter.
        ///
        ///  - The priority ceiling is out of range (NGKI2037, `E_PAR`).
        ///
        ///  - The priority ceiling refers to a priority value which is
        ///    configured to use subpriorities (NGKI3682, `E_ILUSE`).
        ///
        ///  - Unrecognized flags are specified (NGKI2025, `E_RSATR`).
        ///
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
            #[cfg(any(
                all(feature = "asp3", feature = "subprio"),
                feature = "fmp3",
                feature = "solid_fmp3"
            ))]
            abi::E_ILUSE => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MutexRef::delete`].
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

/// Mutex information returned by [`MutexRef::info`].
#[derive(Debug, Clone, Copy)]
pub struct Info {
    #[cfg(not(feature = "none"))]
    raw: abi::T_RMTX,
}

impl Info {
    /// Get the owning task's ID.
    #[inline]
    pub fn owning_task_id(&self) -> Option<abi::NonNullID> {
        match () {
            #[cfg(not(feature = "none"))]
            () => abi::NonNullID::new(self.raw.htskid),
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

/// A borrowed reference to a mutex.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MutexRef<'a> {
    id: abi::NonNullID,
    _phantom: PhantomData<&'a ()>,
}

impl fmt::Debug for MutexRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mutex({})", self.id)
    }
}

/// # Object ID conversion
impl MutexRef<'_> {
    /// Construct a `MutexRef` from a raw object ID.
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
impl MutexRef<'_> {
    /// `del_mtx`: Delete the mutex.
    ///
    /// # Safety
    ///
    /// See [Object ID Wrappers](crate#object-id-wrappers).
    #[inline]
    #[doc(alias = "del_mtx")]
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub unsafe fn delete(self) -> Result<(), Error<DeleteError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::del_mtx(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ref_mtx`: Get the mutex's general information.
    #[inline]
    #[doc(alias = "ref_mtx")]
    pub fn info(self) -> Result<Info, Error<InfoError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::ref_mtx(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(Info {
                    raw: pri.assume_init(),
                })
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// # Mutex Operations
impl MutexRef<'_> {
    /// `loc_mtx`: Lock the mutex.
    #[inline]
    #[doc(alias = "loc_mtx")]
    pub fn lock(self) -> Result<(), Error<LockError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::loc_mtx(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `tloc_mtx`: Lock the mutex with timeout.
    #[inline]
    #[doc(alias = "tloc_mtx")]
    pub fn lock_timeout(self, tmo: Timeout) -> Result<(), Error<LockTimeoutError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::tloc_mtx(self.as_raw(), tmo.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ploc_mtx`: Attempt to lock the mutex. Returns immediately if it's
    /// already locked.
    #[inline]
    #[doc(alias = "ploc_mtx")]
    pub fn try_lock(self) -> Result<(), Error<TryLockError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::ploc_mtx(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `unl_mtx`: Unlock the mutex.
    #[inline]
    #[doc(alias = "unl_mtx")]
    pub fn unlock(self) -> Result<(), Error<UnlockError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::unl_mtx(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ini_mtx`: Initialize the mutex.
    #[inline]
    #[doc(alias = "ini_mtx")]
    pub fn initialize(self) -> Result<(), Error<InitializeError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::ini_mtx(self.as_raw()))?;
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
    use crate::wait::QueueOrder;

    /// Specifies a priority protection protocol used by a [mutex](Mutex).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub enum PriorityProtection {
        /// No priority protection.
        None,
        /// The priority ceiling protocol.
        Ceiling {
            /// Specifies the priority ceiling.
            priority: crate::task::Priority,
        },
        #[cfg(any(feature = "none", all(feature = "solid_asp3", feature = "pi_mutex")))]
        #[cfg_attr(
            feature = "doc_cfg",
            doc(cfg(any(feature = "none", all(feature = "solid_asp3", feature = "pi_mutex"))))
        )]
        /// The priority inheritance protocol.
        Inherit,
    }

    impl PriorityProtection {
        /// Return `Some(Self::Inherit)` if it's supported by the target kernel.
        ///
        /// # Examples
        ///
        /// ```
        /// use itron::mutex::PriorityProtection;
        /// let priority_protection = PriorityProtection::inherit()
        ///     .unwrap_or(PriorityProtection::None);
        /// ```
        #[inline]
        #[allow(unreachable_code)]
        pub const fn inherit() -> Option<Self> {
            #[cfg(any(feature = "none", all(feature = "solid_asp3", feature = "pi_mutex")))]
            {
                return Some(Self::Inherit);
            }
            None
        }
    }

    /// The builder type for [mutexes](Mutex). Created by [`Mutex::build`].
    ///
    /// Its generic parameters are an implementation detail.
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct Builder {
        #[cfg(not(feature = "none"))]
        raw: abi::T_CMTX,
        priority_protection: PriorityProtection,
    }

    impl Mutex {
        /// `acre_mtx`: Create a builder for `Mutex`.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use itron::mutex::Mutex;
        /// let mutex = Mutex::build()
        ///     .finish()
        ///     .expect("failed to create a mutex");
        ///
        /// mutex.as_ref().lock()
        ///    .expect("failed to perform a lock operation");
        /// mutex.as_ref().unlock()
        ///    .expect("failed to perform a unlock operation");
        /// ```
        ///
        /// ```rust,no_run
        /// use itron::mutex::{Mutex, PriorityProtection};
        /// let mutex = Mutex::build()
        ///     .priority_protection(PriorityProtection::Ceiling { priority: 4 })
        ///     .finish()
        ///     .expect("failed to create a priority-ceiling mutex");
        ///
        /// mutex.as_ref().lock()
        ///    .expect("failed to perform a lock operation");
        /// mutex.as_ref().unlock()
        ///    .expect("failed to perform a unlock operation");
        /// ```
        #[inline]
        #[doc(alias = "acre_mtx")]
        pub fn build() -> Builder {
            Builder {
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMTX {
                    mtxatr: abi::TA_NULL,
                    ceilpri: 0,
                },
                priority_protection: PriorityProtection::None,
            }
        }
    }

    impl Builder {
        /// Specify the priority protection mechanism to use.
        /// Defaults to [`None`] when unspecified.
        ///
        /// [`None`]: PriorityProtection::None
        #[inline]
        pub fn priority_protection(self, value: PriorityProtection) -> Builder {
            Builder {
                #[cfg(not(feature = "none"))]
                priority_protection: value,
                ..self
            }
        }

        /// Specify the queue order. Defaults to `Fifo` when unspecified.
        /// Ignored if
        #[inline]
        pub fn queue_order(self, value: QueueOrder) -> Builder {
            Builder {
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMTX {
                    mtxatr: value.as_raw_atr(),
                    ..self.raw
                },
                ..self
            }
        }
    }

    impl Builder {
        /// Create a mutex using the specified parameters.
        #[allow(unused_mut)]
        pub fn finish(mut self) -> Result<Mutex, Error<BuildError>> {
            match self.priority_protection {
                #[cfg(not(feature = "none"))]
                PriorityProtection::None => {}
                #[cfg(not(feature = "none"))]
                PriorityProtection::Ceiling { priority } => {
                    self.raw.mtxatr = abi::TA_CEILING;
                    self.raw.ceilpri = priority;
                }
                #[cfg(all(feature = "solid_asp3", feature = "pi_mutex"))]
                PriorityProtection::Inherit => {
                    self.raw.mtxatr = abi::TA_INHERIT;
                }
                #[cfg(feature = "none")]
                _ => unimplemented!(),
            }
            match () {
                #[cfg(not(feature = "none"))]
                () => unsafe {
                    let id = Error::err_if_negative(abi::acre_mtx(&self.raw))?;
                    // Safety: We own the mutex we create
                    Ok(Mutex::from_raw_nonnull(abi::NonNullID::new_unchecked(id)))
                },
                #[cfg(feature = "none")]
                () => unimplemented!(),
            }
        }
    }

    /// An owned mutex.
    ///
    /// [Deletes] the mutex automatically when dropped. The destructor will
    /// panic if the deletion fails.
    ///
    /// [Deletes]: MutexRef::delete
    #[derive(PartialEq, Eq)]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct Mutex(MutexRef<'static>);

    impl fmt::Debug for Mutex {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Drop for Mutex {
        #[inline]
        fn drop(&mut self) {
            unsafe { self.0.delete().unwrap() };
        }
    }

    impl Mutex {
        /// Construct a `Mutex` from a raw object ID.
        ///
        /// # Safety
        ///
        /// See [Object ID Wrappers](crate#object-id-wrappers).
        #[inline]
        pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
            Self(unsafe { MutexRef::from_raw_nonnull(id) })
        }

        /// Consume and "leak" `self`, returning a reference `MutexRef<'a>`.
        #[inline]
        pub const fn leak<'a>(self) -> MutexRef<'a> {
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

        /// Borrow `Mutex` as [`MutexRef`].
        ///
        /// Use this to perform operations on mutexes because most of the
        /// methods are implemented on `MutexRef` but not `Mutex`.
        #[inline]
        pub const fn as_ref(&self) -> MutexRef<'_> {
            self.0
        }
    }
}
