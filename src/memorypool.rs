//! Memory pools

use core::{fmt, marker::PhantomData, mem::MaybeUninit};

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
    time::Timeout,
};

define_error_kind! {
    /// Error type for [`MemoryPoolRef::get`].
    pub enum GetError {
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

impl ErrorKind for GetError {
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
    /// Error type for [`MemoryPoolRef::get_timeout`].
    pub enum GetTimeoutError {
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

impl ErrorKind for GetTimeoutError {
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
    /// Error type for [`MemoryPoolRef::try_get`].
    pub enum TryGetError {
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

impl ErrorKind for TryGetError {
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
    /// Error type for [`MemoryPoolRef::release`].
    pub enum ReleaseError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        /// The supplied memory block does not originate from the memory pool.
        #[cfg(not(feature = "none"))]
        BadParam,
    }
}

impl ErrorKind for ReleaseError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`MemoryPoolRef::initialize`].
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
    /// Error type for [`MemoryPoolRef::info`].
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
    /// Error type for [`MemoryPool::build`].
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub enum BuildError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(any())]
        AccessDenied,
        /// The specified parameter is not supported by the kernel.
        ///
        ///  - On TOPPERS/HRP3, automatic allocation of a memory pool data
        ///    storage is not supported (`E_NOSPT`, HRPS0199).
        ///
        #[cfg(any())]
        NotSupported,
        /// Ran out of memory or memory pool IDs, or the specified block size
        /// or capacity does not fit in `uint_t`.
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
    /// Error type for [`MemoryPoolRef::delete`].
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

/// A pointer to a memory block.
pub type Block = *mut u8;

/// Memory pool information returned by [`MemoryPoolRef::info`].
#[derive(Debug, Clone, Copy)]
pub struct Info {
    #[cfg(not(feature = "none"))]
    raw: abi::T_RMPF,
}

impl Info {
    /// Get the number of free memory blocks.
    #[inline]
    pub fn free_block_count(&self) -> usize {
        match () {
            #[cfg(not(feature = "none"))]
            () => self.raw.fblkcnt as usize,
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

/// A borrowed reference to a memory pool.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPoolRef<'a> {
    id: abi::NonNullID,
    _phantom: PhantomData<&'a ()>,
}

impl fmt::Debug for MemoryPoolRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MemoryPool({})", self.id)
    }
}

/// # Object ID conversion
impl MemoryPoolRef<'_> {
    /// Construct a `MemoryPoolRef` from a raw object ID.
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
impl MemoryPoolRef<'_> {
    /// `del_mpf`: Delete the memory pool.
    ///
    /// # Safety
    ///
    /// See [Object ID Wrappers](crate#object-id-wrappers).
    #[inline]
    #[doc(alias = "del_mpf")]
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub unsafe fn delete(self) -> Result<(), Error<DeleteError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::del_mpf(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ref_mpf`: Get the memory pool's general information.
    #[inline]
    #[doc(alias = "ref_mpf")]
    pub fn info(self) -> Result<Info, Error<InfoError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::ref_mpf(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(Info {
                    raw: pri.assume_init(),
                })
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// # Memory Pool Operations
impl MemoryPoolRef<'_> {
    /// `get_mpf`: Acquire a memory block. Blocks the current
    /// task if no free memory blocks are available.
    #[inline]
    #[doc(alias = "get_mpf")]
    pub fn get(self) -> Result<Block, Error<GetError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut out = MaybeUninit::uninit();
                Error::err_if_negative(abi::get_mpf(self.as_raw(), out.as_mut_ptr()))?;
                Ok(out.assume_init())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `tget_mpf`: Acquire a memory block. Blocks the current
    /// task with timeout if no free memory blocks are available.
    #[inline]
    #[doc(alias = "tget_mpf")]
    pub fn get_timeout(self, tmo: Timeout) -> Result<Block, Error<GetTimeoutError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut out = MaybeUninit::uninit();
                Error::err_if_negative(abi::tget_mpf(
                    self.as_raw(),
                    out.as_mut_ptr(),
                    tmo.as_raw(),
                ))?;
                Ok(out.assume_init())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `pget_mpf`: Acquire a memory block. Fails and returns immediately if no
    /// free memory blocks are available.
    #[inline]
    #[doc(alias = "pget_mpf")]
    pub fn try_get(self) -> Result<Block, Error<TryGetError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut out = MaybeUninit::uninit();
                Error::err_if_negative(abi::pget_mpf(self.as_raw(), out.as_mut_ptr()))?;
                Ok(out.assume_init())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `rel_mpf`: Return a memory block to the memory pool.
    #[inline]
    #[doc(alias = "rel_mpf")]
    pub fn release(self, block: Block) -> Result<(), Error<ReleaseError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::rel_mpf(self.as_raw(), block))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ini_mpf`: Initialize the memory pool.
    #[inline]
    #[doc(alias = "ini_mpf")]
    pub fn initialize(self) -> Result<(), Error<InitializeError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::ini_mpf(self.as_raw()))?;
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

    /// The builder type for [memory pools](MemoryPool).
    /// Created by [`MemoryPool::build`].
    ///
    /// Its generic parameters are an implementation detail.
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    #[must_use = "`Builder` creates nothing unless you call `.finish()`"]
    pub struct Builder<BlockSize, BlockCount> {
        block_size: BlockSize,
        block_count: BlockCount,
        blkcnt_overflow: bool,
        blksz_overflow: bool,
        #[cfg(not(feature = "none"))]
        raw: abi::T_CMPF,
    }

    /// Builder field hole types
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub mod builder_hole {
        pub struct __block_size_is_not_specified__;
        pub struct __block_count_is_not_specified__;
    }

    impl MemoryPool {
        /// `acre_mpf`: Create a builder for `MemoryPool`.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// use itron::memorypool::MemoryPool;
        /// let pool = MemoryPool::build()
        ///     .block_size(32)
        ///     .block_count(4)
        ///     .finish()
        ///     .expect("failed to create a memory pool");
        ///
        /// let block = pool.as_ref().get()
        ///    .expect("failed to allocate a block");
        /// pool.as_ref().release(block)
        ///    .expect("failed to deallocate a block");
        /// ```
        ///
        #[inline]
        #[doc(alias = "acre_mpf")]
        pub fn build() -> Builder<
            builder_hole::__block_size_is_not_specified__,
            builder_hole::__block_count_is_not_specified__,
        > {
            Builder {
                block_size: builder_hole::__block_size_is_not_specified__,
                block_count: builder_hole::__block_count_is_not_specified__,
                blkcnt_overflow: false,
                blksz_overflow: false,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMPF {
                    mpfatr: abi::TA_NULL,
                    blkcnt: 0,
                    blksz: 0,
                    mpf: core::ptr::null_mut(),
                    mpfmb: core::ptr::null_mut(),
                },
            }
        }
    }

    impl<BlockSize, BlockCount> Builder<BlockSize, BlockCount> {
        /// (**Mandatory**) Specify the block size.
        #[inline]
        pub fn block_size(self, value: usize) -> Builder<(), BlockCount> {
            let (blksz, blksz_overflow) = match value.try_into() {
                Ok(x) => (x, false),
                Err(_) => (0, true),
            };
            Builder {
                block_size: (),
                block_count: self.block_count,
                blksz_overflow,
                blkcnt_overflow: self.blkcnt_overflow,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMPF { blksz, ..self.raw },
            }
        }

        /// (**Mandatory**) Specify the capacity, measured in number of blocks.
        #[inline]
        pub fn block_count(self, value: usize) -> Builder<BlockSize, ()> {
            let (blkcnt, blkcnt_overflow) = match value.try_into() {
                Ok(x) => (x, false),
                Err(_) => (0, true),
            };
            Builder {
                block_size: self.block_size,
                block_count: (),
                blksz_overflow: self.blksz_overflow,
                blkcnt_overflow,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMPF { blkcnt, ..self.raw },
            }
        }

        /// Specify the queue order. Defaults to `Fifo` when unspecified.
        #[inline]
        pub fn queue_order(self, value: QueueOrder) -> Self {
            Builder {
                #[cfg(not(feature = "none"))]
                raw: abi::T_CMPF {
                    mpfatr: value.as_raw_atr(),
                    ..self.raw
                },
                ..self
            }
        }
    }

    impl Builder<(), ()> {
        /// Create a memory pool using the specified parameters.
        #[allow(unused_mut)]
        pub fn finish(mut self) -> Result<MemoryPool, Error<BuildError>> {
            match () {
                #[cfg(not(feature = "none"))]
                () => unsafe {
                    if self.blksz_overflow || self.blkcnt_overflow {
                        // Safety: `E_NOMEM` is handled by `BuildError`
                        // (Warning: This is not true for `cfg(feature = "none")`.)
                        return Err(Error::new_unchecked(ErrorCode::new_unchecked(abi::E_NOMEM)));
                    }

                    let id = Error::err_if_negative(abi::acre_mpf(&self.raw))?;
                    // Safety: We own the memory pool we create
                    Ok(MemoryPool::from_raw_nonnull(abi::NonNullID::new_unchecked(
                        id,
                    )))
                },
                #[cfg(feature = "none")]
                () => unimplemented!(),
            }
        }
    }

    /// An owned memory pool.
    ///
    /// [Deletes] the memory pool automatically when dropped. The destructor will
    /// panic if the deletion fails.
    ///
    /// [Deletes]: MemoryPoolRef::delete
    #[derive(PartialEq, Eq)]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct MemoryPool(MemoryPoolRef<'static>);

    impl fmt::Debug for MemoryPool {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Drop for MemoryPool {
        #[inline]
        fn drop(&mut self) {
            unsafe { self.0.delete().unwrap() };
        }
    }

    impl MemoryPool {
        /// Construct a `MemoryPool` from a raw object ID.
        ///
        /// # Safety
        ///
        /// See [Object ID Wrappers](crate#object-id-wrappers).
        #[inline]
        pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
            Self(unsafe { MemoryPoolRef::from_raw_nonnull(id) })
        }

        /// Consume and "leak" `self`, returning a reference `MemoryPoolRef<'a>`.
        #[inline]
        pub const fn leak<'a>(self) -> MemoryPoolRef<'a> {
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

        /// Borrow `MemoryPool` as [`MemoryPoolRef`].
        ///
        /// Use this to perform operations on memory pools because most of the
        /// methods are implemented on `MemoryPoolRef` but not `MemoryPool`.
        #[inline]
        pub const fn as_ref(&self) -> MemoryPoolRef<'_> {
            self.0
        }
    }
}
