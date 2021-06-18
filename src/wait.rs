//! Shared definitions for waitable objects
pub use crate::abi;

/// Specifies the sorting order of a wait queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueOrder {
    /// The wait queue is processed in a FIFO order.
    Fifo,
    /// The wait queue is processed in a task priority order.
    TaskPriority,
}

impl QueueOrder {
    /// Convert `self` to a value of type [`abi::ATR`].
    #[inline]
    pub fn as_raw_atr(self) -> abi::ATR {
        match self {
            #[cfg(not(feature = "none"))]
            Self::Fifo => abi::TA_NULL,
            #[cfg(not(feature = "none"))]
            Self::TaskPriority => abi::TA_TPRI,
            #[cfg(feature = "none")]
            _ => unreachable!(),
        }
    }
}
