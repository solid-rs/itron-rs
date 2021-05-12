use super::{bool_t, uint_t, ATR, ER, ER_ID, ER_UINT, EXINF, ID, PRI, RELTIM, STAT, TMO};

pub type TASK = Option<unsafe extern "C" fn(EXINF)>;

/*
 *  オブジェクト属性の定義
 */
/// タスクを起動された状態で生成
pub const TA_ACT: ATR = 0x01;
/// 起動要求をキューイングしない
pub const TA_NOACTQUE: ATR = 0x02;

/*
 *  オブジェクトの状態の定義
 */
/// 実行状態
pub const TTS_RUN: STAT = 0x01;
/// 実行可能状態
pub const TTS_RDY: STAT = 0x02;
/// 待ち状態
pub const TTS_WAI: STAT = 0x04;
/// 強制待ち状態
pub const TTS_SUS: STAT = 0x08;
/// 二重待ち状態
pub const TTS_WAS: STAT = 0x0c;
/// 休止状態
pub const TTS_DMT: STAT = 0x10;

/// TOPPERS/ASP3 dynamic creation extension `T_CTSK`
#[cfg(all(feature = "asp3", feature = "dcre"))]
#[derive(Debug, Clone, Copy)]
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
    pub fn can_act(tskid: ID) -> ER_UINT;
    pub fn get_tst(tskid: ID, p_tskstat: *mut STAT) -> ER;
    pub fn chg_pri(tskid: ID, tskpri: PRI) -> ER;
    pub fn get_pri(tskid: ID, p_tskpri: *mut PRI) -> ER;
    pub fn get_inf(p_exinf: *mut isize) -> ER;
    pub fn ref_tsk(tskid: ID, pk_rtsk: *mut T_RTSK) -> ER;
}

#[cfg(all(feature = "asp3", feature = "dcre"))]
extern "C" {
    pub fn acre_tsk(pk_ctsk: *const T_CTSK) -> ER_ID;
    pub fn del_tsk(tskid: ID) -> ER;
}

/// タスク付属同期機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn slp_tsk() -> ER;
    pub fn tslp_tsk(tmout: TMO) -> ER;
    pub fn wup_tsk(tskid: ID) -> ER;
    pub fn can_wup(tskid: ID) -> ER_UINT;
    pub fn rel_wai(tskid: ID) -> ER;
    pub fn sus_tsk(tskid: ID) -> ER;
    pub fn rsm_tsk(tskid: ID) -> ER;
    pub fn dly_tsk(dlytim: RELTIM) -> ER;
}

/// タスク終了機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn ext_tsk() -> ER;
    pub fn ras_ter(tskid: ID) -> ER;
    pub fn dis_ter() -> ER;
    pub fn ena_ter() -> ER;
    pub fn sns_ter() -> bool_t;
    pub fn ter_tsk(tskid: ID) -> ER;
}
