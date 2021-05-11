//! Tasks
use core::{fmt, marker::PhantomData, mem::MaybeUninit};

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
};

define_error_kind! {
    /// Error type for [`TaskRef::activate`].
    pub enum ActivateError {
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

impl ErrorKind for ActivateError {
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
    /// Error type for [`TaskRef::cancel_activate_all`].
    pub enum CancelActivateAllError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for CancelActivateAllError {
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
    /// Error type for [`TaskRef::set_priority`].
    pub enum SetPriorityError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is dormant.
        #[cfg(not(feature = "none"))]
        BadState,
        /// Bad parameter.
        ///
        ///  - The task is a restricted task, for which changing the priority is
        ///    not supported (NGKI1186).
        ///
        ///  - The specified priority is out of range.
        ///
        ///  - The task owns a priority-ceiling mutex, and the specified priority
        ///    is higher than the mutex's priority ceiling.
        ///
        #[cfg(not(feature = "none"))]
        BadParam,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for SetPriorityError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadState(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR | abi::E_NOSPT | abi::E_ILUSE => {
                Some(Self::BadParam(Kind::from_error_code(code)))
            }
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::priority`].
    pub enum PriorityError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is dormant.
        #[cfg(not(feature = "none"))]
        BadState,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for PriorityError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // `E_MACV` is a critical error, so it's excluded from here
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadState(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::delete`].
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

/// Task priority value.
pub type Priority = abi::PRI;

/// A borrowed reference to a task.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TaskRef<'a> {
    id: abi::NonNullID,
    _phantom: PhantomData<&'a ()>,
}

impl fmt::Debug for TaskRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task({})", self.id)
    }
}

impl TaskRef<'_> {
    /// Construct a `TaskRef` from a raw object ID.
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

    /// `act_tsk`: Pend an activation request for the task.
    #[inline]
    #[doc(alias = "act_tsk")]
    pub fn activate(self) -> Result<(), Error<ActivateError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::act_tsk(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `can_act`: Cancel any pending activation requests for the task.
    /// Returns the number of cancelled requests.
    #[inline]
    #[doc(alias = "can_act")]
    pub fn cancel_activate_all(self) -> Result<usize, Error<CancelActivateAllError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let count = Error::err_if_negative(abi::can_act(self.as_raw()))?;
                Ok(count as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `chg_pri`: Change the task's priority.
    #[inline]
    #[doc(alias = "chg_pri")]
    pub fn set_priority(self, new_priority: Priority) -> Result<(), Error<SetPriorityError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::chg_pri(self.as_raw(), new_priority))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `get_pri`: Get the task's priority.
    #[inline]
    #[doc(alias = "get_pri")]
    pub fn priority(self) -> Result<Priority, Error<PriorityError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::get_pri(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(pri.assume_init())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `del_tsk`: Delete the task.
    #[inline]
    #[doc(alias = "del_tsk")]
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub unsafe fn delete(self) -> Result<(), Error<DeleteError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::del_tsk(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    // TODO: get_tst
    // TODO: ref_tsk
    // TODO: wup_tsk
    // TODO: can_wup
    // TODO: rel_wai
    // TODO: sus_tsk
    // TODO: rsm_tsk
    // TODO: ras_ter
    // TODO: ter_tsk
}

#[cfg(feature = "dcre")]
pub use self::owned::*;

#[cfg(feature = "dcre")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
mod owned {
    use super::*;

    /// An owned task.
    ///
    /// [Deletes] the task automatically when dropped. The destructor will
    /// panic if the deletion fails.
    ///
    /// [Deletes]: TaskRef::delete
    #[derive(PartialEq, Eq)]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct Task(TaskRef<'static>);

    impl fmt::Debug for Task {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Drop for Task {
        #[inline]
        fn drop(&mut self) {
            unsafe { self.0.delete().unwrap() };
        }
    }

    impl Task {
        /// Construct a `Task` from a raw object ID.
        #[inline]
        pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
            Self(unsafe { TaskRef::from_raw_nonnull(id) })
        }

        /// Consume and "leak" `self`, returning a reference `TaskRef<'static>`.
        #[inline]
        pub const fn leak<'a>(self) -> TaskRef<'a> {
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

        /// Borrow `Task` as [`TaskRef`].
        ///
        /// Use this to perform operations on tasks because most of the methods
        /// are implemented on `TaskRef` but not `Task`.
        #[inline]
        pub const fn as_ref(&self) -> TaskRef<'_> {
            self.0
        }
    }
}
