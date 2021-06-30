//! Message buffers

use core::{convert::TryInto, fmt, marker::PhantomData, mem::MaybeUninit};

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
    time::Timeout,
};

define_error_kind! {
    /// Error type for [`MessageBufferRef::send`].
    pub enum SendError {
        #[cfg(not(feature = "none"))]
        BadContext,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The message is too large (`E_PAR`, NGKI3364).
        #[cfg(not(feature = "none"))]
        BadParam,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        Released,
        #[cfg(not(feature = "none"))]
        TerminateRequest,
        #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
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
            abi::E_PAR => Some(Self::BadParam(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RLWAI => Some(Self::Released(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RASTER => Some(Self::TerminateRequest(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
            abi::E_DLT => Some(Self::Deleted(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MessageBufferRef::send_timeout`].
    pub enum SendTimeoutError {
        #[cfg(not(feature = "none"))]
        BadContext,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The message is too large (`E_PAR`, NGKI3364).
        #[cfg(not(feature = "none"))]
        BadParam,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        Timeout,
        #[cfg(not(feature = "none"))]
        Released,
        #[cfg(not(feature = "none"))]
        TerminateRequest,
        #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
        Deleted,
    }
}

impl ErrorKind for SendTimeoutError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // `E_PAR` for invalid timeout is considered critial
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR => Some(Self::BadParam(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_TMOUT => Some(Self::Timeout(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RLWAI => Some(Self::Released(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RASTER => Some(Self::TerminateRequest(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
            abi::E_DLT => Some(Self::Deleted(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MessageBufferRef::try_send`].
    pub enum TrySendError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The message is too large (`E_PAR`, NGKI3364).
        #[cfg(not(feature = "none"))]
        BadParam,
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
            #[cfg(not(feature = "none"))]
            abi::E_PAR => Some(Self::BadParam(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_TMOUT => Some(Self::Timeout(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MessageBufferRef::recv`].
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
        #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
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
            #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
            abi::E_DLT => Some(Self::Deleted(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MessageBufferRef::recv_timeout`].
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
        #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
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
            #[cfg(all(not(feature = "none"), not(feature = "asp3"), feature = "dcre"))]
            abi::E_DLT => Some(Self::Deleted(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MessageBufferRef::try_recv`].
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
    /// Error type for [`MessageBufferRef::initialize`].
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
    /// Error type for [`MessageBufferRef::info`].
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
    /// Error type for [`MessageBuffer::build`].
    #[cfg(all(feature = "dcre", not(feature = "asp3")))]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(all(feature = "dcre", not(feature = "asp3")))))]
    pub enum BuildError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(any())]
        AccessDenied,
        #[cfg(any())]
        NotSupported,
        /// Ran out of memory or message buffer IDs, or the specified block size
        /// or capacity does not fit in `uint_t`.
        #[cfg(not(feature = "none"))]
        OutOfMemory,
        /// Bad parameter.
        #[cfg(not(feature = "none"))]
        BadParam,
    }
}

#[cfg(all(feature = "dcre", not(feature = "asp3")))]
impl ErrorKind for BuildError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // `E_MACV` is considered critical, hence excluded
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_NOID | abi::E_NOMEM => Some(Self::OutOfMemory(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR | abi::E_RSATR => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MessageBufferRef::delete`].
    #[cfg(all(feature = "dcre", not(feature = "asp3")))]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(all(feature = "dcre", not(feature = "asp3")))))]
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

#[cfg(all(feature = "dcre", not(feature = "asp3")))]
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

/// Message buffer information returned by [`MessageBufferRef::info`].
#[derive(Debug, Clone, Copy)]
pub struct Info {
    #[cfg(not(feature = "none"))]
    raw: abi::T_RMBF,
}

impl Info {
    /// Get the number of free bytes.
    #[inline]
    pub fn free_byte_count(&self) -> usize {
        match () {
            #[cfg(not(feature = "none"))]
            () => self.raw.fmbfsz,
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// Get the number of stored messages.
    #[inline]
    pub fn len(&self) -> usize {
        match () {
            #[cfg(not(feature = "none"))]
            () => self.raw.smbfcnt as usize,
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
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

/// Literally anything.
pub trait Unknown {}

impl<T: ?Sized> Unknown for T {}

/// A borrowed reference to a message buffer.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MessageBufferRef<'a> {
    id: abi::NonNullID,
    _phantom: PhantomData<&'a ()>,
}

impl fmt::Debug for MessageBufferRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MessageBuffer({})", self.id)
    }
}

/// # Object ID conversion
impl MessageBufferRef<'_> {
    /// Construct a `MessageBufferRef` from a raw object ID.
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
impl MessageBufferRef<'_> {
    /// `del_mbf`: Delete the message buffer.
    ///
    /// # Safety
    ///
    /// See [Object ID Wrappers](crate#object-id-wrappers).
    #[inline]
    #[doc(alias = "del_mbf")]
    #[cfg(all(feature = "dcre", not(feature = "asp3")))]
    #[cfg_attr(
        feature = "doc_cfg",
        doc(cfg(all(feature = "dcre", not(feature = "asp3"))))
    )]
    pub unsafe fn delete(self) -> Result<(), Error<DeleteError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::del_mbf(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ref_mbf`: Get the message buffer's general information.
    #[inline]
    #[doc(alias = "ref_mbf")]
    pub fn info(self) -> Result<Info, Error<InfoError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::ref_mbf(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(Info {
                    raw: pri.assume_init(),
                })
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// # Message Buffer Operations
impl MessageBufferRef<'_> {
    /// `snd_mbf`: Send a message to the message buffer. Blocks the current task
    /// if the message buffer is full.
    #[inline]
    #[doc(alias = "snd_mbf")]
    pub fn send(self, message: &(impl Unknown + ?Sized)) -> Result<(), Error<SendError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                // If the message size doesn't fit in `uint_t`, it's guaranteed
                // to exceed the message buffer's maximum message size.
                // Safety: `E_PAR` is handled by `SendError`
                // (Warning: This is not true for `cfg(feature = "none")`.)
                let size = core::mem::size_of_val(message)
                    .try_into()
                    .ok()
                    .ok_or(Error::new_unchecked(ErrorCode::new_unchecked(abi::E_PAR)))?;
                Error::err_if_negative(abi::snd_mbf(
                    self.as_raw(),
                    message as *const _ as *const u8,
                    size,
                ))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `tsnd_mbf`: Send a message to the message buffer. Blocks the current
    /// task with timeout if the message buffer is full.
    #[inline]
    #[doc(alias = "tsnd_mbf")]
    pub fn send_timeout(
        self,
        message: &(impl Unknown + ?Sized),
        tmo: Timeout,
    ) -> Result<(), Error<SendTimeoutError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                // If the message size doesn't fit in `uint_t`, it's guaranteed
                // to exceed the message buffer's maximum message size.
                // Safety: `E_PAR` is handled by `SendTimeoutError`
                // (Warning: This is not true for `cfg(feature = "none")`.)
                let size = core::mem::size_of_val(message)
                    .try_into()
                    .ok()
                    .ok_or(Error::new_unchecked(ErrorCode::new_unchecked(abi::E_PAR)))?;
                Error::err_if_negative(abi::tsnd_mbf(
                    self.as_raw(),
                    message as *const _ as *const u8,
                    size,
                    tmo.as_raw(),
                ))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `psnd_mbf`: Send a message to the message buffer. Fails and returns an
    /// error if the message buffer is full.
    #[inline]
    #[doc(alias = "psnd_mbf")]
    pub fn try_send(self, message: &(impl Unknown + ?Sized)) -> Result<(), Error<TrySendError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                // If the message size doesn't fit in `uint_t`, it's guaranteed
                // to exceed the message buffer's maximum message size.
                // Safety: `E_PAR` is handled by `TrySendError`
                // (Warning: This is not true for `cfg(feature = "none")`.)
                let size = core::mem::size_of_val(message)
                    .try_into()
                    .ok()
                    .ok_or(Error::new_unchecked(ErrorCode::new_unchecked(abi::E_PAR)))?;
                Error::err_if_negative(abi::psnd_mbf(
                    self.as_raw(),
                    message as *const _ as *const u8,
                    size,
                ))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `rcv_mbf`: Receive a message from the message buffer. Blocks the current
    /// task if the message buffer is empty.
    ///
    /// On success, the received message will be written to `*out`, and the
    /// message size, measured in bytes, will be returned.
    ///
    /// # Rationale
    ///
    /// This method is named `recv` instead of `receive` following the suit of
    /// `std::sync::mpsc::Receiver::recv` and `std::net::UdpSocket::recv`.
    ///
    /// # Safety
    ///
    /// `*out` must be large enough to fit the message. After this method
    /// overwrites `*out` with the received message, the resultant content of
    /// `*out` must be valid for its type.
    #[inline]
    #[doc(alias = "rcv_mbf")]
    pub unsafe fn recv(self, out: &mut (impl Unknown + ?Sized)) -> Result<usize, Error<RecvError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let size =
                    Error::err_if_negative(abi::rcv_mbf(self.as_raw(), out as *mut _ as *mut u8))?;
                Ok(size as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `trcv_mbf`: Receive a message to the message buffer. Blocks the current
    /// task with timeout if the message buffer is empty.
    ///
    /// # Safety
    ///
    /// See [`Self::recv`].
    #[inline]
    #[doc(alias = "trcv_mbf")]
    pub unsafe fn recv_timeout(
        self,
        out: &mut (impl Unknown + ?Sized),
        tmo: Timeout,
    ) -> Result<usize, Error<RecvTimeoutError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let size = Error::err_if_negative(abi::trcv_mbf(
                    self.as_raw(),
                    out as *mut _ as *mut u8,
                    tmo.as_raw(),
                ))?;
                Ok(size as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `prcv_mbf`: Receive a message to the message buffer. Fails and returns
    /// an error if the message buffer is empty.
    ///
    /// # Safety
    ///
    /// See [`Self::recv`].
    #[inline]
    #[doc(alias = "prcv_mbf")]
    pub unsafe fn try_recv(
        self,
        out: &mut (impl Unknown + ?Sized),
    ) -> Result<usize, Error<TryRecvError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let size =
                    Error::err_if_negative(abi::prcv_mbf(self.as_raw(), out as *mut _ as *mut u8))?;
                Ok(size as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ini_mbf`: Initialize the message buffer.
    #[inline]
    #[doc(alias = "ini_mbf")]
    pub fn initialize(self) -> Result<(), Error<InitializeError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::ini_mbf(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

// Note that TOPPERS/ASP3 doesn't support dynamic creation of message buffers
// because message buffers and dynamic creation are provided by separate
// extensions.
#[cfg(all(feature = "dcre", not(feature = "asp3")))]
pub use self::owned::*;

#[cfg(all(feature = "dcre", not(feature = "asp3")))]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
mod owned {
    use super::*;

    /// The builder type for [message buffers](MessageBuffer).
    /// Created by [`MessageBuffer::build`].
    ///
    /// Its generic parameters are an implementation detail.
    #[cfg_attr(
        feature = "doc_cfg",
        doc(cfg(all(feature = "dcre", not(feature = "asp3"))))
    )]
    #[must_use = "`Builder` creates nothing unless you call `.finish()`"]
    pub struct Builder<Capacity, MaxMessageSize> {
        capacity: Capacity,
        max_message_size: MaxMessageSize,
        maxmsz_overflow: bool,
        #[cfg(not(feature = "none"))]
        raw: abi::T_CMBF,
    }

    /// Builder field hole types
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub mod builder_hole {
        pub struct __block_size_is_not_specified__;
        pub struct __block_count_is_not_specified__;
    }

    impl MessageBuffer {
        /// `acre_mbf`: Create a builder for `MessageBuffer`.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use itron::messagebuffer::MessageBuffer;
        /// let buffer = MessageBuffer::build()
        ///     .capacity(128)
        ///     .max_message_size(64)
        ///     .finish()
        ///     .expect("failed to create a message buffer");
        ///
        /// buffer.as_ref().send(&[0u8; 32])
        ///    .expect("failed to send a message");
        ///
        /// let mut got_message = [0u8; 64];
        /// let got_message_len = unsafe {
        ///     buffer
        ///         .as_ref()
        ///         .recv(&mut got_message)
        ///         .expect("failed to receive a message")
        /// };
        /// assert_eq!(got_message_len, 32);
        /// ```
        ///
        #[inline]
        #[doc(alias = "acre_mbf")]
        pub fn build() -> Builder<
            builder_hole::__block_size_is_not_specified__,
            builder_hole::__block_count_is_not_specified__,
        > {
            Builder {
                capacity: builder_hole::__block_size_is_not_specified__,
                max_message_size: builder_hole::__block_count_is_not_specified__,
                maxmsz_overflow: false,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMBF {
                    mbfatr: abi::TA_NULL,
                    mbfsz: 0,
                    maxmsz: 0,
                    mbfmb: core::ptr::null_mut(),
                },
            }
        }
    }

    impl<Capacity, MaxMessageSize> Builder<Capacity, MaxMessageSize> {
        /// (**Mandatory**) Specify the capacity, measured in bytes.
        #[inline]
        pub fn capacity(self, value: usize) -> Builder<(), MaxMessageSize> {
            Builder {
                capacity: (),
                max_message_size: self.max_message_size,
                maxmsz_overflow: self.maxmsz_overflow,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMBF {
                    mbfsz: value,
                    ..self.raw
                },
            }
        }

        /// (**Mandatory**) Specify the maximum message size, measured in bytes.
        #[inline]
        pub fn max_message_size(self, value: usize) -> Builder<Capacity, ()> {
            let (maxmsz, maxmsz_overflow) = match value.try_into() {
                Ok(x) => (x, false),
                Err(_) => (0, true),
            };
            Builder {
                capacity: self.capacity,
                max_message_size: (),
                maxmsz_overflow,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMBF { maxmsz, ..self.raw },
            }
        }
    }

    impl Builder<(), ()> {
        /// Create a message buffer using the specified parameters.
        #[allow(unused_mut)]
        pub fn finish(mut self) -> Result<MessageBuffer, Error<BuildError>> {
            match () {
                #[cfg(not(feature = "none"))]
                () => unsafe {
                    if self.maxmsz_overflow {
                        // Safety: `E_NOMEM` is handled by `BuildError`
                        // (Warning: This is not true for `cfg(feature = "none")`.)
                        return Err(Error::new_unchecked(ErrorCode::new_unchecked(abi::E_NOMEM)));
                    }

                    let id = Error::err_if_negative(abi::acre_mbf(&self.raw))?;
                    // Safety: We own the message buffer we create
                    Ok(MessageBuffer::from_raw_nonnull(
                        abi::NonNullID::new_unchecked(id),
                    ))
                },
                #[cfg(feature = "none")]
                () => unimplemented!(),
            }
        }
    }

    /// An owned message buffer.
    ///
    /// [Deletes] the message buffer automatically when dropped. The destructor will
    /// panic if the deletion fails.
    ///
    /// [Deletes]: MessageBufferRef::delete
    #[derive(PartialEq, Eq)]
    #[cfg_attr(
        feature = "doc_cfg",
        doc(cfg(all(feature = "dcre", not(feature = "asp3"))))
    )]
    pub struct MessageBuffer(MessageBufferRef<'static>);

    impl fmt::Debug for MessageBuffer {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Drop for MessageBuffer {
        #[inline]
        fn drop(&mut self) {
            unsafe { self.0.delete().unwrap() };
        }
    }

    impl MessageBuffer {
        /// Construct a `MessageBuffer` from a raw object ID.
        ///
        /// # Safety
        ///
        /// See [Object ID Wrappers](crate#object-id-wrappers).
        #[inline]
        pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
            Self(unsafe { MessageBufferRef::from_raw_nonnull(id) })
        }

        /// Consume and "leak" `self`, returning a reference `MessageBufferRef<'a>`.
        #[inline]
        pub const fn leak<'a>(self) -> MessageBufferRef<'a> {
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

        /// Borrow `MessageBuffer` as [`MessageBufferRef`].
        ///
        /// Use this to perform operations on message buffers because most of the
        /// methods are implemented on `MessageBufferRef` but not `MessageBuffer`.
        #[inline]
        pub const fn as_ref(&self) -> MessageBufferRef<'_> {
            self.0
        }
    }
}
