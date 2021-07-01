//! Dataqueues

use core::{fmt, marker::PhantomData, mem::MaybeUninit};

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
    time::Timeout,
};

define_error_kind! {
    /// Error type for [`DataqueueRef::send`].
    pub enum SendError {
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

impl ErrorKind for SendError {
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
    /// Error type for [`DataqueueRef::send_timeout`].
    pub enum SendTimeoutError {
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

impl ErrorKind for SendTimeoutError {
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
    /// Error type for [`DataqueueRef::try_send`].
    pub enum TrySendError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        Timeout,
    }
}

impl ErrorKind for TrySendError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_TMOUT => Some(Self::Timeout(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`DataqueueRef::send_forced`].
    pub enum SendForcedError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        /// The queue length is zero.
        #[cfg(not(feature = "none"))]
        ZeroSized,
    }
}

impl ErrorKind for SendForcedError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ILUSE => Some(Self::ZeroSized(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`DataqueueRef::recv`].
    pub enum RecvError {
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

impl ErrorKind for RecvError {
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
    /// Error type for [`DataqueueRef::recv_timeout`].
    pub enum RecvTimeoutError {
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

impl ErrorKind for RecvTimeoutError {
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
    /// Error type for [`DataqueueRef::try_recv`].
    pub enum TryRecvError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        Timeout,
    }
}

impl ErrorKind for TryRecvError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_TMOUT => Some(Self::Timeout(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`DataqueueRef::initialize`].
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
    /// Error type for [`DataqueueRef::info`].
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
    /// Error type for [`Dataqueue::build`].
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub enum BuildError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(any())]
        AccessDenied,
        /// Ran out of memory or dataqueue IDs, or the specified capacity
        /// does not fit in `uint_t`.
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
            abi::E_NOID | abi::E_NOMEM => Some(Self::OutOfMemory(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR | abi::E_RSATR => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`DataqueueRef::delete`].
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

/// The unit of data that can be sent through a dataqueue.
///
/// # Rationale
///
/// Although the data element type used by the kernel API is signed, in Rust,
/// unsigned integer types are preferred to represent raw data.
pub type DataElement = usize;

/// Dataqueue information returned by [`DataqueueRef::info`].
#[derive(Debug, Clone, Copy)]
pub struct Info {
    #[cfg(not(feature = "none"))]
    raw: abi::T_RDTQ,
}

impl Info {
    /// Get the number of data items contained in the dataqueue.
    #[inline]
    pub fn len(&self) -> usize {
        match () {
            // Since `sdtqcnt` represents a number of objects in memory, the
            // conversion should not cause an overflow
            #[cfg(not(feature = "none"))]
            () => self.raw.sdtqcnt as usize,
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// Get a flag indicating whether the dataqueue is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the first waiting sender's task ID.
    #[inline]
    pub fn first_waiting_sending_task_id(&self) -> Option<abi::NonNullID> {
        match () {
            #[cfg(not(feature = "none"))]
            () => abi::NonNullID::new(self.raw.stskid),
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// Get the first waiting receiver's task ID.
    #[inline]
    pub fn first_waiting_receiving_task_id(&self) -> Option<abi::NonNullID> {
        match () {
            #[cfg(not(feature = "none"))]
            () => abi::NonNullID::new(self.raw.rtskid),
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// A borrowed reference to a dataqueue.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DataqueueRef<'a> {
    id: abi::NonNullID,
    _phantom: PhantomData<&'a ()>,
}

impl fmt::Debug for DataqueueRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dataqueue({})", self.id)
    }
}

/// # Object ID conversion
impl DataqueueRef<'_> {
    /// Construct a `DataqueueRef` from a raw object ID.
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
impl DataqueueRef<'_> {
    /// `del_dtq`: Delete the dataqueue.
    ///
    /// # Safety
    ///
    /// See [Object ID Wrappers](crate#object-id-wrappers).
    #[inline]
    #[doc(alias = "del_dtq")]
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub unsafe fn delete(self) -> Result<(), Error<DeleteError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::del_dtq(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ref_dtq`: Get the dataqueue's general information.
    #[inline]
    #[doc(alias = "ref_dtq")]
    pub fn info(self) -> Result<Info, Error<InfoError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::ref_dtq(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(Info {
                    raw: pri.assume_init(),
                })
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// # Dataqueue Operations
impl DataqueueRef<'_> {
    /// `snd_dtq`: Send a data element to the dataqueue. Blocks the current task
    /// if the dataqueue is full.
    #[inline]
    #[doc(alias = "snd_dtq")]
    pub fn send(self, data_element: DataElement) -> Result<(), Error<SendError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::snd_dtq(self.as_raw(), data_element as isize))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `tsnd_dtq`: Send a data element to the dataqueue. Blocks the current
    /// task with timeout if the dataqueue is full.
    #[inline]
    #[doc(alias = "tsnd_dtq")]
    pub fn send_timeout(
        self,
        data_element: DataElement,
        tmo: Timeout,
    ) -> Result<(), Error<SendTimeoutError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::tsnd_dtq(
                    self.as_raw(),
                    data_element as isize,
                    tmo.as_raw(),
                ))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `psnd_dtq`: Send a data element to the dataqueue. Fails and returns an
    /// error if the dataqueue is full.
    #[inline]
    #[doc(alias = "psnd_dtq")]
    pub fn try_send(self, data_element: DataElement) -> Result<(), Error<TrySendError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::psnd_dtq(self.as_raw(), data_element as isize))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `fsnd_dtq`: Send a data element to the dataqueue. Removes ("pushes out")
    /// the oldest element if the dataqueue is full.
    #[inline]
    #[doc(alias = "fsnd_dtq")]
    pub fn send_forced(self, data_element: DataElement) -> Result<(), Error<SendForcedError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::fsnd_dtq(self.as_raw(), data_element as isize))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `rcv_dtq`: Receive a data element from the dataqueue. Blocks the current
    /// task if the dataqueue is empty.
    ///
    /// # Rationale
    ///
    /// This method is named `recv` instead of `receive` following the suit of
    /// `std::sync::mpsc::Receiver::recv` and `std::net::UdpSocket::recv`.
    #[inline]
    #[doc(alias = "rcv_dtq")]
    pub fn recv(self) -> Result<DataElement, Error<RecvError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut out = MaybeUninit::<isize>::uninit();
                Error::err_if_negative(abi::rcv_dtq(self.as_raw(), out.as_mut_ptr()))?;
                Ok(out.assume_init() as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `trcv_dtq`: Receive a data element to the dataqueue. Blocks the current
    /// task with timeout if the dataqueue is empty.
    #[inline]
    #[doc(alias = "trcv_dtq")]
    pub fn recv_timeout(self, tmo: Timeout) -> Result<DataElement, Error<RecvTimeoutError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut out = MaybeUninit::<isize>::uninit();
                Error::err_if_negative(abi::trcv_dtq(
                    self.as_raw(),
                    out.as_mut_ptr(),
                    tmo.as_raw(),
                ))?;
                Ok(out.assume_init() as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `prcv_dtq`: Receive a data element to the dataqueue. Fails and returns
    /// an error if the dataqueue is empty.
    #[inline]
    #[doc(alias = "prcv_dtq")]
    pub fn try_recv(self) -> Result<DataElement, Error<TryRecvError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut out = MaybeUninit::<isize>::uninit();
                Error::err_if_negative(abi::prcv_dtq(self.as_raw(), out.as_mut_ptr()))?;
                Ok(out.assume_init() as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ini_sem`: Initialize the dataqueue.
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
    use crate::wait::QueueOrder;
    use core::convert::TryInto;

    /// The builder type for [dataqueues](Dataqueue). Created by [`Dataqueue::build`].
    ///
    /// Its generic parameters are an implementation detail.
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    #[must_use = "`Builder` creates nothing unless you call `.finish()`"]
    pub struct Builder<Capacity> {
        #[allow(dead_code)]
        capacity: Capacity,
        capacity_overflow: bool,
        #[cfg(not(feature = "none"))]
        raw: abi::T_CDTQ,
    }

    /// Builder field hole types
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub mod builder_hole {
        pub struct __capacity_is_not_specified__;
    }

    impl Dataqueue {
        /// `acre_dtq`: Create a builder for `Dataqueue`.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use itron::dataqueue::Dataqueue;
        /// let dataqueue = Dataqueue::build()
        ///     .capacity(2)
        ///     .finish()
        ///     .expect("failed to create a dataqueue");
        ///
        /// dataqueue.as_ref().send(1)
        ///    .expect("failed to send a data element");
        /// dataqueue.as_ref().send(2)
        ///    .expect("failed to send a data element");
        /// dataqueue.as_ref().try_send(3)
        ///    .expect_err("unexpectedly succeeded to send a data element");
        /// ```
        #[inline]
        #[doc(alias = "acre_dtq")]
        pub fn build() -> Builder<builder_hole::__capacity_is_not_specified__> {
            Builder {
                capacity: builder_hole::__capacity_is_not_specified__,
                capacity_overflow: false,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CDTQ {
                    dtqatr: abi::TA_NULL,
                    dtqcnt: 0,
                    dtqmb: core::ptr::null_mut(),
                },
            }
        }
    }

    // TODO: abi::T_CDTQ::dtqmb

    impl<Capacity> Builder<Capacity> {
        /// (**Mandatory**) Specify the dataqueue's capacity, measured in
        /// number of data elements.
        #[inline]
        pub fn capacity(self, value: usize) -> Builder<()> {
            let (capacity, capacity_overflow) = match value.try_into() {
                Ok(x) => (x, false),
                Err(_) => (0, true),
            };
            Builder {
                // FIXME: Use the struct update syntax when rust-lang/rfcs#2528
                //        is implemented
                capacity: (),
                capacity_overflow,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CDTQ {
                    dtqcnt: capacity,
                    ..self.raw
                },
            }
        }

        /// Specify the queue order. Defaults to `Fifo` when unspecified.
        #[inline]
        pub fn queue_order(self, value: QueueOrder) -> Self {
            Builder {
                #[cfg(not(feature = "none"))]
                raw: abi::T_CDTQ {
                    dtqatr: value.as_raw_atr(),
                    ..self.raw
                },
                ..self
            }
        }
    }

    impl Builder<()> {
        /// Create a dataqueue using the specified parameters.
        #[allow(unused_mut)]
        pub fn finish(mut self) -> Result<Dataqueue, Error<BuildError>> {
            match () {
                #[cfg(not(feature = "none"))]
                () => unsafe {
                    if self.capacity_overflow {
                        // Safety: `E_NOMEM` is handled by `BuildError`
                        // (Warning: This is not true for `cfg(feature = "none")`.)
                        return Err(Error::new_unchecked(ErrorCode::new_unchecked(abi::E_NOMEM)));
                    }

                    let id = Error::err_if_negative(abi::acre_dtq(&self.raw))?;
                    // Safety: We own the dataqueue we create
                    Ok(Dataqueue::from_raw_nonnull(abi::NonNullID::new_unchecked(
                        id,
                    )))
                },
                #[cfg(feature = "none")]
                () => unimplemented!(),
            }
        }
    }

    /// An owned dataqueue.
    ///
    /// [Deletes] the dataqueue automatically when dropped. The destructor will
    /// panic if the deletion fails.
    ///
    /// [Deletes]: DataqueueRef::delete
    #[derive(PartialEq, Eq)]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct Dataqueue(DataqueueRef<'static>);

    impl fmt::Debug for Dataqueue {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Drop for Dataqueue {
        #[inline]
        fn drop(&mut self) {
            unsafe { self.0.delete().unwrap() };
        }
    }

    impl Dataqueue {
        /// Construct a `Dataqueue` from a raw object ID.
        ///
        /// # Safety
        ///
        /// See [Object ID Wrappers](crate#object-id-wrappers).
        #[inline]
        pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
            Self(unsafe { DataqueueRef::from_raw_nonnull(id) })
        }

        /// Consume and "leak" `self`, returning a reference `DataqueueRef<'a>`.
        #[inline]
        pub const fn leak<'a>(self) -> DataqueueRef<'a> {
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

        /// Borrow `Dataqueue` as [`DataqueueRef`].
        ///
        /// Use this to perform operations on dataqueues because most of the
        /// methods are implemented on `DataqueueRef` but not `Dataqueue`.
        #[inline]
        pub const fn as_ref(&self) -> DataqueueRef<'_> {
            self.0
        }
    }
}
