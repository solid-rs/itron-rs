use super::{bool_t, uint_t, ATR, ER, ER_UINT, EXINF, ID, PRI, STAT, TMO};

pub type TASK = extern "C" fn(EXINF);

/// TOPPERS/ASP3 dynamic creation extension `T_CTSK`
#[cfg(all(feature = "asp3", feature = "dcre"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_CTSK {
    /// タスク属性
    pub tskatr: ATR,
    /// タスクの拡張情報
    pub exinf: EXINF,
    /// タスクのメインルーチンの先頭番地
    pub task: TASK,
    /// タスクの起動時優先度
    pub itskpri: PRI,
    /// タスクのスタック領域のサイズ
    pub stksz: usize,
    /// タスクのスタック領域の先頭番地
    pub stk: *mut u8,
}

/// TOPPERS/ASP3 `T_RTSK`
#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RTSK {
    /// タスク状態
    pub tskstat: STAT,
    /// タスクの現在優先度
    pub tskpri: PRI,
    /// タスクのベース優先度
    pub tskbpri: PRI,
    /// 待ち要因
    pub tskwait: STAT,
    /// 待ち対象のオブジェクトのID
    pub wobjid: ID,
    /// タイムアウトするまでの時間
    pub lefttmo: TMO,
    /// 起動要求キューイング数
    pub actcnt: uint_t,
    /// 起床要求キューイング数
    pub wupcnt: uint_t,
    /// タスク終了要求状態
    pub raster: bool_t,
    /// タスク終了禁止状態
    pub dister: bool_t,
}

/// タスク管理機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn act_tsk(tskid: ID) -> ER;
    pub fn iact_tsk(tskid: ID) -> ER;
    pub fn can_act(tskid: ID) -> ER_UINT;
    pub fn ext_tsk() -> ER;
    pub fn ter_tsk(tskid: ID) -> ER;
    pub fn chg_pri(tskid: ID, tskpri: PRI) -> ER;
    pub fn get_pri(tskid: ID, p_tskpri: *mut PRI) -> ER;
    pub fn get_inf(p_exinf: *mut isize) -> ER;
    pub fn ref_tsk(tskid: ID, pk_rtsk: *mut T_RTSK) -> ER;
}

#[cfg(all(feature = "asp3", feature = "dcre"))]
extern "C" {
    pub fn acre_tsk(pk_ctsk: *const T_CTSK) -> ER;
    pub fn del_tsk(tskid: ID) -> ER;
}
