//! Tasks
use core::{fmt, marker::PhantomData, mem::MaybeUninit};

use crate::{
    abi,
    error::{Error, ErrorCode, ErrorKind, Kind},
    time::{Duration, Timeout},
};

// TODO: sta_ovr
// TODO: stp_ovr
// TODO: ref_ovr
// TODO: chg_spr
// TODO: mact_tsk
// TODO: chg_spr
// TODO: TA_ACT
// TODO: TA_NOACTQUE
// TODO: TA_RTSK

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
    /// Error type for [`TaskRef::activate_on`].
    #[cfg(any(feature = "none", feature = "fmp3", feature = "solid_fmp3"))]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(any(feature = "none", feature = "fmp3", feature = "solid_fmp3"))))]
    pub enum ActivateOnError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(any())]
        AccessDenied,
        #[cfg(not(feature = "none"))]
        QueueOverflow,
        /// The class the task belongs to does not permit assigning tasks to the
        /// specified processor.
        #[cfg(not(feature = "none"))]
        BadParam,
    }
}

#[cfg(any(feature = "none", feature = "fmp3", feature = "solid_fmp3"))]
impl ErrorKind for ActivateOnError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_QOVR => Some(Self::QueueOverflow(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR => Some(Self::BadParam(Kind::from_error_code(code))),
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

define_error_kind! {
    /// Error type for [`TaskRef::state`].
    pub enum StateError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for StateError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // `E_MACX` is considered critical, hence excluded
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
    /// Error type for [`TaskRef::info`].
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
            // `E_MACX` is considered critical, hence excluded
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
    /// Error type for [`TaskRef::wake`].
    pub enum WakeError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(any())]
        AccessDenied,
        /// Bad state.
        ///
        ///  - The task is dormant.
        ///
        #[cfg(not(feature = "none"))]
        BadState,
        #[cfg(not(feature = "none"))]
        QueueOverflow,
    }
}

impl ErrorKind for WakeError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadState(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_QOVR => Some(Self::QueueOverflow(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::cancel_wake_all`].
    pub enum CancelWakeAllError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(any())]
        AccessDenied,
        /// Bad state.
        ///
        ///  - The task is dormant.
        ///
        #[cfg(not(feature = "none"))]
        BadState,
    }
}

impl ErrorKind for CancelWakeAllError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadState(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::release_wait`].
    pub enum ReleaseWaitError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(any())]
        AccessDenied,
        /// Bad state.
        ///
        ///  - The task is not waiting.
        ///
        #[cfg(not(feature = "none"))]
        BadState,
    }
}

impl ErrorKind for ReleaseWaitError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadState(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::suspend`].
    pub enum SuspendError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(any())]
        AccessDenied,
        /// Bad state.
        ///
        ///  - The task is dormant.
        ///  - The task has a pending termination request.
        ///
        #[cfg(not(feature = "none"))]
        BadState,
        /// The task is already suspended.
        #[cfg(not(feature = "none"))]
        QueueOverflow,
    }
}

impl ErrorKind for SuspendError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ | abi::E_RASTER => Some(Self::BadState(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_QOVR => Some(Self::QueueOverflow(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::resume`].
    pub enum ResumeError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(any())]
        AccessDenied,
        /// Bad state.
        ///
        ///  - The task is not suspended.
        ///
        #[cfg(not(feature = "none"))]
        BadState,
    }
}

impl ErrorKind for ResumeError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_ID | abi::E_NOEXS => Some(Self::BadId(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_OBJ => Some(Self::BadState(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::terminate`].
    pub enum TerminateError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        /// Bad state.
        ///
        ///  - The task is dormant.
        ///  - The task is assigned to a processor that is different from the
        ///    current one (`E_OBJ`, NGKI3481).
        ///
        #[cfg(not(feature = "none"))]
        BadState,
        /// Bad parameter.
        ///
        ///  - The current task cannot be terminated,
        ///
        #[cfg(not(feature = "none"))]
        BadParam,
    }
}

impl ErrorKind for TerminateError {
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
            #[cfg(not(feature = "none"))]
            abi::E_ILUSE => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`TaskRef::raise_termination`].
    pub enum RaiseTerminationError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(not(feature = "none"))]
        BadId,
        #[cfg(any())]
        AccessDenied,
        /// Bad state.
        ///
        ///  - The task is dormant.
        ///  - The task is assigned to a processor that is different from the
        ///    current one (`E_OBJ`, NGKI3481).
        ///
        #[cfg(not(feature = "none"))]
        BadState,
        /// Bad state.
        ///
        ///  - The current task cannot be terminated,
        ///
        #[cfg(not(feature = "none"))]
        BadParam,
    }
}

impl ErrorKind for RaiseTerminationError {
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
            #[cfg(not(feature = "none"))]
            abi::E_ILUSE => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`sleep`].
    pub enum SleepError {
        #[cfg(not(feature = "none"))]
        BadContext,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(not(feature = "none"))]
        Released,
        #[cfg(not(feature = "none"))]
        TerminateRequest,
    }
}

impl ErrorKind for SleepError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RLWAI => Some(Self::Released(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RASTER => Some(Self::TerminateRequest(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`sleep_timeout`].
    pub enum SleepTimeoutError {
        #[cfg(not(feature = "none"))]
        BadContext,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(not(feature = "none"))]
        Timeout,
        #[cfg(not(feature = "none"))]
        Released,
        #[cfg(not(feature = "none"))]
        TerminateRequest,
    }
}

impl ErrorKind for SleepTimeoutError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // E_PAR is considered critial, hence excluded
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_TMOUT => Some(Self::Timeout(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RLWAI => Some(Self::Released(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RASTER => Some(Self::TerminateRequest(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`delay`].
    pub enum DelayError {
        #[cfg(not(feature = "none"))]
        BadContext,
        /// The task is a restricted task.
        #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
        NotSupported,
        #[cfg(not(feature = "none"))]
        Released,
        #[cfg(not(feature = "none"))]
        TerminateRequest,
    }
}

impl ErrorKind for DelayError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // E_PAR is considered critial, hence excluded
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(all(not(feature = "none"), feature = "rstr_task"))]
            abi::E_NOSPT => Some(Self::NotSupported(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RLWAI => Some(Self::Released(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_RASTER => Some(Self::TerminateRequest(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`exit`].
    pub enum ExitError {
        #[cfg(not(feature = "none"))]
        BadContext,
    }
}

impl ErrorKind for ExitError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            // E_SYS is considered critial, hence excluded
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`disable_termination`].
    pub enum DisableTerminationError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(any())]
        AccessDenied,
    }
}

impl ErrorKind for DisableTerminationError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

/// Error type for [`enable_termination`].
pub type EnableTerminationError = DisableTerminationError;

/// The error type returned by [`current`] when the CPU lock state is active,
/// or the current thread is not in a task context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BadContextError(());

define_error_kind! {
    /// Error type for [`current_id`].
    pub enum CurrentIdError {
        /// The CPU lock state is active.
        #[cfg(not(feature = "none"))]
        BadContext,
    }
}

impl ErrorKind for CurrentIdError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        match code.get() {
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

define_error_kind! {
    /// Error type for [`Task::build`].
    #[cfg(feature = "dcre")]
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub enum BuildError {
        #[cfg(not(feature = "none"))]
        BadContext,
        #[cfg(any())]
        AccessDenied,
        /// Ran out of memory or task IDs.
        #[cfg(not(feature = "none"))]
        OutOfMemory,
        /// Bad parameter.
        ///
        ///  - The specified stack region overlaps with an existing memory
        ///    object (NGKI1060, `E_OBJ`).
        ///
        ///  - The specified system stack region is not included a kernel-only
        ///    memory object (NGKI1057, `E_OBJ`).
        ///
        ///  - The specified stack size is too small (NGKI1042, `E_PAR`).
        ///
        ///  - The specified system stack size is too small (NGKI1044, `E_PAR`).
        ///
        ///  - NGKI5108, `E_PAR`.
        ///
        ///  - The kernel configuration requires manual stack specification, but
        ///    the caller did not specify one (NGKI3907, `E_PAR`).
        ///
        ///  - The specified stack does not meet target-specific requirements
        ///    (NGKI1056, `E_PAR`).
        ///
        ///  - The specified system stack does not meet target-specific
        ///    requirements (NGKI1065, `E_PAR`).
        ///
        ///  - The caller requested to create a system task, and `sstk` is
        ///    non-null (NGKI1068, `E_PAR`).
        ///
        ///  - The caller requested to create a system task, `sstksz != 0`,
        ///    and `stk` is non-null (NGKI1071, `E_PAR`).
        ///
        #[cfg(not(feature = "none"))]
        BadParam,
    }
}

#[cfg(feature = "dcre")]
impl ErrorKind for BuildError {
    fn from_error_code(code: ErrorCode) -> Option<Self> {
        // `Builder::build` uses `get_pid` to get the current processor and
        // coalesces its error code into this error kind type. Thus, this error
        // kind type must be able to handle errors from both `acre_tsk` and
        // `get_pid`!
        match code.get() {
            // `E_MACV` is considered critical, hence excluded
            #[cfg(not(feature = "none"))]
            abi::E_CTX => Some(Self::BadContext(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OACV => Some(Self::AccessDenied(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_NOID | abi::E_NOMEM => Some(Self::OutOfMemory(Kind::from_error_code(code))),
            #[cfg(not(feature = "none"))]
            abi::E_PAR => Some(Self::BadParam(Kind::from_error_code(code))),
            #[cfg(any())]
            abi::E_OBJ => Some(Self::BadParam(Kind::from_error_code(code))),
            _ => None,
        }
    }
}

/// Task priority value.
pub type Priority = abi::PRI;

/// Task state returned by [`TaskRef::state`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum State {
    Running = abi::TTS_RUN as u8,
    Ready = abi::TTS_RDY as u8,
    Waiting = abi::TTS_WAI as u8,
    Suspended = abi::TTS_SUS as u8,
    WaitingSuspended = abi::TTS_WAS as u8,
    Dormant = abi::TTS_DMT as u8,
}

impl State {
    #[inline]
    unsafe fn from_abi_unchecked(x: abi::STAT) -> Self {
        unsafe { core::mem::transmute(x as u8) }
    }
}

/// Task information returned by [`TaskRef::info`].
#[derive(Debug, Clone, Copy)]
pub struct Info {
    #[cfg(not(feature = "none"))]
    raw: abi::T_RTSK,
}

impl Info {
    /// Get the task's state.
    #[inline]
    pub fn state(&self) -> State {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe { State::from_abi_unchecked(self.raw.tskstat) },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// Get the task's current priority.
    #[inline]
    pub fn current_priority(&self) -> Priority {
        match () {
            #[cfg(not(feature = "none"))]
            () => self.raw.tskpri,
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// Get the task's base priority.
    #[inline]
    pub fn base_priority(&self) -> Priority {
        match () {
            #[cfg(not(feature = "none"))]
            () => self.raw.tskbpri,
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    // TODO: tskwait
    // TODO: wobjid
    // TODO: lefttmo
    // TODO: actcnt
    // TODO: wupcnt
    // TODO: raster
    // TODO: dister
    // TODO: prcid
    // TODO: actprc
}

/// `slp_tsk`: Put the current task to sleep.
///
/// The [`TaskRef::wake`] method and this function are semantically analogous to
/// `std::thread::Thread::unpark` and `std::thread::park`, respectively.
#[inline]
#[doc(alias = "slp_tsk")]
#[doc(alias = "park")]
pub fn sleep() -> Result<(), Error<SleepError>> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe {
            Error::err_if_negative(abi::slp_tsk())?;
            Ok(())
        },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `dly_tsk`: Put the current task to sleep with timeout.
#[inline]
#[doc(alias = "tslp_tsk")]
#[doc(alias = "park_timeout")]
pub fn sleep_timeout(tmo: Timeout) -> Result<(), Error<SleepTimeoutError>> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe {
            Error::err_if_negative(abi::tslp_tsk(tmo.as_raw()))?;
            Ok(())
        },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `dly_tsk`: Delay the current task.
#[inline]
#[doc(alias = "dly_tsk")]
pub fn delay(dur: Duration) -> Result<(), Error<DelayError>> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe {
            Error::err_if_negative(abi::dly_tsk(dur.as_raw()))?;
            Ok(())
        },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `ext_tsk`: Terminate the current task.
///
/// This function will not return if it succeeds.
///
/// # Safety
///
/// If the task's stack is reused later, stored local variables are
/// destroyed without running their destructors, violating the [pinning]
/// requirements.
///
/// [pinning]: core::pin
#[inline]
#[doc(alias = "ext_tsk")]
pub unsafe fn exit() -> Error<ExitError> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe { Error::new_unchecked(ErrorCode::new_unchecked(abi::ext_tsk())) },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `dis_ter`: Disable the termination of the current task by
/// [a termination request].
///
/// [a termination request]: TaskRef::raise_termination
#[inline]
#[doc(alias = "dis_ter")]
pub fn disable_termination() -> Result<(), Error<DisableTerminationError>> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe {
            Error::err_if_negative(abi::dis_ter())?;
            Ok(())
        },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `ena_ter`: Re-enable the termination of the current task by
/// [a termination request].
///
/// [a termination request]: TaskRef::raise_termination
#[inline]
#[doc(alias = "ena_ter")]
pub fn enable_termination() -> Result<(), Error<EnableTerminationError>> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe {
            Error::err_if_negative(abi::ena_ter())?;
            Ok(())
        },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `sns_ter`: Determine if the termination by [a termination request] is
/// disabled for the current task.
///
/// [a termination request]: TaskRef::raise_termination
#[inline]
#[doc(alias = "sns_ter")]
pub fn is_termination_disabled() -> bool {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe { abi::sns_ter() != 0 },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

/// `get_tid`: Get the currently running task's ID.
#[inline]
#[doc(alias = "get_tid")]
pub fn current_id() -> Result<Option<abi::NonNullID>, Error<CurrentIdError>> {
    match () {
        #[cfg(not(feature = "none"))]
        () => unsafe {
            let mut out = MaybeUninit::uninit();
            Error::err_if_negative(abi::get_tid(out.as_mut_ptr()))?;
            Ok(abi::NonNullID::new(out.assume_init()))
        },
        #[cfg(feature = "none")]
        () => unimplemented!(),
    }
}

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

/// # Object ID conversion
impl TaskRef<'_> {
    /// Construct a `TaskRef` from a raw object ID.
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

    /// Get the raw object ID as [`abi::NonNullID`].
    #[inline]
    pub const fn as_raw_nonnull(self) -> abi::NonNullID {
        self.id
    }
}

/// # Management
impl TaskRef<'_> {
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

    /// `mact_tsk`: Pend an activation request for the task, assigning it to
    /// the specified processor.
    #[inline]
    #[doc(alias = "mact_tsk")]
    #[cfg(any(feature = "none", feature = "fmp3", feature = "solid_fmp3"))]
    #[cfg_attr(
        feature = "doc_cfg",
        doc(cfg(any(feature = "none", feature = "fmp3", feature = "solid_fmp3")))
    )]
    pub fn activate_on(
        self,
        processor: crate::processor::Processor,
    ) -> Result<(), Error<ActivateOnError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::mact_tsk(self.as_raw(), processor.as_raw()))?;
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
    ///
    /// # Safety
    ///
    /// See [Object ID Wrappers](crate#object-id-wrappers).
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

    /// `get_tst`: Get the task's state.
    #[inline]
    #[doc(alias = "get_tst")]
    pub fn state(self) -> Result<State, Error<StateError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::get_tst(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(State::from_abi_unchecked(pri.assume_init()))
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ref_tsk`: Get the task's general information.
    #[inline]
    #[doc(alias = "ref_tsk")]
    pub fn info(self) -> Result<Info, Error<InfoError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let mut pri = MaybeUninit::uninit();
                Error::err_if_negative(abi::ref_tsk(self.as_raw(), pri.as_mut_ptr()))?;
                Ok(Info {
                    raw: pri.assume_init(),
                })
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// # Synchronization
impl TaskRef<'_> {
    /// `wup_tsk`: Pend a wake up request for the task.
    ///
    /// This method and the [`sleep`] function are semantically analogous to
    /// `std::thread::Thread::unpark` and `std::thread::park`, respectively.
    /// However, unlike `unpark`, **this method will return
    /// `Err(WakeError::QueueOverflow)` if the token is already present.**
    #[inline]
    #[doc(alias = "wup_tsk")]
    #[doc(alias = "unpark")]
    pub fn wake(self) -> Result<(), Error<WakeError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::wup_tsk(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `can_wup`: Cancel any wake up requests for the task.
    /// Returns the number of cancelled requests.
    #[inline]
    #[doc(alias = "can_wup")]
    pub fn cancel_wake_all(self) -> Result<usize, Error<CancelWakeAllError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                let count = Error::err_if_negative(abi::can_wup(self.as_raw()))?;
                Ok(count as usize)
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `rel_wai`: Forcibly release the task from waiting.
    #[inline]
    #[doc(alias = "rel_wai")]
    pub fn release_wait(self) -> Result<(), Error<ReleaseWaitError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::rel_wai(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `sus_tsk`: Suspend the task.
    #[inline]
    #[doc(alias = "sus_tsk")]
    pub fn suspend(self) -> Result<(), Error<SuspendError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::sus_tsk(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `rsm_tsk`: Resume the task.
    #[inline]
    #[doc(alias = "rsm_tsk")]
    pub fn resume(self) -> Result<(), Error<ResumeError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::rsm_tsk(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// # Termination
impl TaskRef<'_> {
    /// `ter_tsk`: Terminate the task.
    ///
    /// # Safety
    ///
    /// If the task's stack is reused later, stored local variables are
    /// destroyed without running their destructors, violating the [pinning]
    /// requirements.
    ///
    /// [pinning]: core::pin
    #[inline]
    #[doc(alias = "ter_tsk")]
    pub unsafe fn terminate(self) -> Result<(), Error<TerminateError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::ter_tsk(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }

    /// `ras_ter`: Pend a termination request.
    ///
    /// # Safety
    ///
    /// If the task's stack is reused later, stored local variables are
    /// destroyed without running their destructors, violating the [pinning]
    /// requirements.
    ///
    /// [pinning]: core::pin
    #[inline]
    #[doc(alias = "ras_ter")]
    pub unsafe fn raise_termination(self) -> Result<(), Error<RaiseTerminationError>> {
        match () {
            #[cfg(not(feature = "none"))]
            () => unsafe {
                Error::err_if_negative(abi::ras_ter(self.as_raw()))?;
                Ok(())
            },
            #[cfg(feature = "none")]
            () => unimplemented!(),
        }
    }
}

/// Get a reference to the current task.
///
/// This function fails if it's called from an interrupt context or the CPU
/// lock state is active.
pub fn current() -> Result<Current, BadContextError> {
    if super::kernel::is_task_context() {
        match current_id() {
            Ok(id) => Ok(Current {
                // Safety: It's allowed to get the current task's `TaskRef`.
                //         The retrieved `TaskRef` will not outlive the task
                //         because `Current` is `!Send`.
                inner: unsafe { TaskRef::from_raw_nonnull(id.unwrap()) },
                _no_send: PhantomData,
            }),
            Err(e) => match e.kind() {
                CurrentIdError::BadContext(_) => Err(BadContextError(())),
            },
        }
    } else {
        Err(BadContextError(()))
    }
}

/// Represents a reference to the current task. Returned by [`current`].
///
/// This type is `!Send`, so it cannot be sent to other threads. This ensures
/// any `TaskRef`s created from this type do not outlive the referenced task.
#[derive(Debug, Clone, Copy)]
pub struct Current {
    inner: TaskRef<'static>,
    _no_send: PhantomData<*mut ()>,
}

impl Current {
    /// Get the raw object ID.
    #[inline]
    pub const fn as_raw(&self) -> abi::ID {
        self.inner.as_raw()
    }

    /// Get the raw object ID as [`abi::NonNullID`].
    #[inline]
    pub const fn as_raw_nonnull(&self) -> abi::NonNullID {
        self.inner.as_raw_nonnull()
    }

    /// Borrow `Current` as [`TaskRef`].
    ///
    /// Use this to perform operations on tasks because most of the methods
    /// are implemented on `TaskRef` but not `Current`.
    #[inline]
    pub const fn as_ref(&self) -> TaskRef<'_> {
        self.inner
    }
}

#[cfg(feature = "dcre")]
pub use self::owned::*;

#[cfg(feature = "dcre")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
mod owned {
    use super::*;

    #[cfg(any(feature = "none", feature = "solid_fmp3"))]
    pub use self::processor_set::*;

    #[cfg(any(feature = "none", feature = "solid_fmp3"))]
    mod processor_set {
        use crate::{abi, processor::Processor};
        use core::convert::TryFrom;

        /// The trait implemented by types that can be passed to
        /// [`crate::task::Builder::processor_affinity`]. This trait is [sealed].
        ///
        /// [sealed]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
        #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
        pub trait IntoProcessorSet: private::Sealed + Sized {
            #[doc(hidden)]
            fn into_uint_t(self) -> abi::uint_t;
        }

        /// Implements [the sealed trait pattern (C-SEALED)].
        ///
        /// [the sealed trait pattern (C-SEALED)]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
        mod private {
            use super::*;

            pub trait Sealed {}

            impl<T: IntoIterator<Item = Processor>> Sealed for T {}
            impl Sealed for Processor {}
            impl Sealed for AllProcessors {}
        }

        impl<T: IntoIterator<Item = Processor>> IntoProcessorSet for T {
            #[doc(hidden)]
            #[inline]
            fn into_uint_t(self) -> abi::uint_t {
                self.into_iter()
                    .fold(0, |st, processor| st | processor.into_uint_t())
            }
        }

        impl IntoProcessorSet for Processor {
            #[doc(hidden)]
            #[inline]
            fn into_uint_t(self) -> abi::uint_t {
                u32::try_from(self.as_raw() - 1)
                    .ok()
                    .and_then(|i| (1 as abi::uint_t).checked_shl(i))
                    .expect("invalid processor ID")
            }
        }

        /// An instance of [`IntoProcessorSet`] specifying all processors.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct AllProcessors;

        impl IntoProcessorSet for AllProcessors {
            #[doc(hidden)]
            #[inline]
            fn into_uint_t(self) -> abi::uint_t {
                abi::uint_t::MAX
            }
        }
    }

    /// The builder type for [tasks](Task). Created by [`Task::build`].
    ///
    /// Its generic parameters are an implementation detail.
    #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "dcre")))]
    pub struct Builder<Start, Stack, InitialPriority> {
        start: Start,
        stack: Stack,
        initial_priority: InitialPriority,
        assign_to_current_procesor: bool,
        #[cfg(not(feature = "none"))]
        raw: abi::T_CTSK,
    }

    /// Builder field hole types
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    pub mod builder_hole {
        pub struct __start_is_not_specified__;
        pub struct __stack_is_not_specified__;
        pub struct __initial_priority_is_not_specified__;
    }

    impl Task {
        /// `acre_tsk`: Create a builder for `Task`.
        ///
        /// # Example
        ///
        /// ```rust,no_run
        /// use itron::task::Task;
        /// let captured_variable = 42u16;
        /// let task = Task::build()
        ///     .start(move || { let _ = captured_variable; })
        ///     .stack(4096)
        ///     .initial_priority(4)
        ///     .finish()
        ///     .expect("failed to create a task");
        ///
        /// task.as_ref().activate().expect("failed to activate the created task");
        ///
        /// // The created task might be still active, so if we just let `task`
        /// // go out of scope, its destructor will panic. `Task::leak` consumes
        /// // `Task` and prevents the destructor from running.
        /// task.leak();
        /// ```
        #[inline]
        #[doc(alias = "acre_tsk")]
        pub fn build() -> Builder<
            builder_hole::__start_is_not_specified__,
            builder_hole::__stack_is_not_specified__,
            builder_hole::__initial_priority_is_not_specified__,
        > {
            Builder {
                start: builder_hole::__start_is_not_specified__,
                stack: builder_hole::__stack_is_not_specified__,
                initial_priority: builder_hole::__initial_priority_is_not_specified__,
                assign_to_current_procesor: true,
                #[cfg(any(feature = "asp3", feature = "solid_asp3"))]
                raw: abi::T_CTSK {
                    tskatr: abi::TA_NULL,
                    exinf: abi::EXINF::uninit(),
                    task: None,
                    itskpri: 0,
                    stksz: 0,
                    stk: core::ptr::null_mut(),
                },
                #[cfg(feature = "solid_fmp3")]
                raw: abi::T_CTSK {
                    tskatr: abi::TA_NULL,
                    exinf: abi::EXINF::uninit(),
                    task: None,
                    itskpri: 0,
                    stksz: 0,
                    stk: core::ptr::null_mut(),
                    affinity: abi::uint_t::MAX,
                    iprcid: 0,
                },
            }
        }
    }

    impl<Start, Stack, InitialPriority> Builder<Start, Stack, InitialPriority> {
        /// (**Mandatory**) Specify the entry point.
        #[inline]
        pub fn start(
            self,
            value: impl crate::closure::IntoClosure,
        ) -> Builder<(), Stack, InitialPriority> {
            let (task, exinf) = value.into_closure();
            Builder {
                // FIXME: Use the struct update syntax when rust-lang/rfcs#2528
                //        is implemented
                start: (),
                stack: self.stack,
                initial_priority: self.initial_priority,
                assign_to_current_procesor: self.assign_to_current_procesor,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CTSK {
                    task: Some(task),
                    exinf,
                    ..self.raw
                },
            }
        }

        /// (**Mandatory**) Specify to use an automatically allocated stack
        /// region of the specified size.
        #[inline]
        pub fn stack(self, size: usize) -> Builder<Start, (), InitialPriority> {
            Builder {
                start: self.start,
                stack: (),
                initial_priority: self.initial_priority,
                assign_to_current_procesor: self.assign_to_current_procesor,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CTSK {
                    stksz: size,

                    stk: core::ptr::null_mut(),
                    ..self.raw
                },
            }
        }

        /// (**Mandatory**) Specify the initial priority.
        #[inline]
        pub fn initial_priority(self, value: Priority) -> Builder<Start, Stack, ()> {
            Builder {
                start: self.start,
                stack: self.stack,
                initial_priority: (),
                assign_to_current_procesor: self.assign_to_current_procesor,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CTSK {
                    itskpri: value,
                    ..self.raw
                },
            }
        }

        /// Specify the task's initial assigned processor. Defaults to the
        /// current processor when unspecified.
        #[inline]
        #[cfg(any(feature = "none", feature = "solid_fmp3"))]
        #[cfg_attr(
            feature = "doc_cfg",
            doc(cfg(any(feature = "none", feature = "solid_fmp3")))
        )]
        pub fn initial_processor(self, value: crate::processor::Processor) -> Self {
            Builder {
                assign_to_current_procesor: false,
                #[cfg(not(feature = "none"))]
                raw: abi::T_CTSK {
                    iprcid: value.as_raw(),
                    ..self.raw
                },
                ..self
            }
        }

        /// Specify the task's assignable processsor set. Defaults to all
        /// processors when unspecified.
        ///
        /// This function might panic if an invalid processor ID is specified.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// #![feature(const_option)]
        /// use itron::{task::Task, processor::Processor};
        /// const P1: Processor = Processor::from_raw(1).unwrap();
        /// const P3: Processor = Processor::from_raw(3).unwrap();
        /// const P4: Processor = Processor::from_raw(4).unwrap();
        /// let task = Task::build()
        ///     .start(move || {})
        ///     .stack(4096)
        ///     .initial_priority(4)
        ///     .initial_processor(P3)
        ///     .processor_affinity([P1, P3, P4])
        ///     .finish()
        ///     .expect("failed to create a task");
        /// ```
        #[inline]
        #[cfg(any(feature = "none", feature = "solid_fmp3"))]
        #[cfg_attr(
            feature = "doc_cfg",
            doc(cfg(any(feature = "none", feature = "solid_fmp3")))
        )]
        pub fn processor_affinity(self, value: impl IntoProcessorSet) -> Self {
            Builder {
                #[cfg(not(feature = "none"))]
                raw: abi::T_CTSK {
                    affinity: value.into_uint_t(),
                    ..self.raw
                },
                ..self
            }
        }
    }

    impl Builder<(), (), ()> {
        /// Create a task using the specified parameters.
        pub fn finish(mut self) -> Result<Task, Error<BuildError>> {
            #[cfg(feature = "solid_fmp3")]
            if self.assign_to_current_procesor {
                unsafe { Error::err_if_negative(abi::get_pid(&mut self.raw.iprcid))? };
            }

            match () {
                #[cfg(not(feature = "none"))]
                () => unsafe {
                    let id = Error::err_if_negative(abi::acre_tsk(&self.raw))?;
                    // Safety: We own the task we create
                    Ok(Task::from_raw_nonnull(abi::NonNullID::new_unchecked(id)))
                },
                #[cfg(feature = "none")]
                () => unimplemented!(),
            }
        }
    }

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
        ///
        /// # Safety
        ///
        /// See [Object ID Wrappers](crate#object-id-wrappers).
        #[inline]
        pub const unsafe fn from_raw_nonnull(id: abi::NonNullID) -> Self {
            Self(unsafe { TaskRef::from_raw_nonnull(id) })
        }

        /// Consume and "leak" `self`, returning a reference `TaskRef<'a>`.
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

        /// Get the raw object ID as [`abi::NonNullID`].
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
