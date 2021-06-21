use super::{bool_t, uint_t, ATR, ER, ER_ID, ER_UINT, EXINF, ID, PRI, RELTIM, STAT, TMO};

pub type TASK = Option<unsafe extern "C" fn(EXINF)>;

/*
 *  オブジェクト属性の定義
 */
/// タスクを起動された状態で生成
pub const TA_ACT: ATR = 0x01;
/// 起動要求をキューイングしない
pub const TA_NOACTQUE: ATR = 0x02;
#[cfg(all(feature = "asp3", feature = "rstr_task"))]
/// 制約タスク
pub const TA_RSTR: ATR = 0x04;

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

/// 起床待ち
pub const TTW_SLP: STAT = 0x0001;
/// 時間経過待ち
pub const TTW_DLY: STAT = 0x0002;
/// セマフォの資源獲得待ち
pub const TTW_SEM: STAT = 0x0004;
/// イベントフラグ待ち
pub const TTW_FLG: STAT = 0x0008;
/// データキューへの送信待ち
pub const TTW_SDTQ: STAT = 0x0010;
/// データキューからの受信待ち
pub const TTW_RDTQ: STAT = 0x0020;
/// 優先度データキューへの送信待ち
pub const TTW_SPDQ: STAT = 0x0100;
/// 優先度データキューからの受信待ち
pub const TTW_RPDQ: STAT = 0x0200;
#[cfg(any(
    all(feature = "asp3", feature = "messagebuf"),
    all(feature = "solid_asp3", feature = "messagebuf")
))]
/// メッセージバッファへの送信待ち
pub const TTW_SMBF: STAT = 0x0400;
#[cfg(any(
    all(feature = "asp3", feature = "messagebuf"),
    all(feature = "solid_asp3", feature = "messagebuf")
))]
/// メッセージバッファからの受信待ち
pub const TTW_RMBF: STAT = 0x0800;
/// ミューテックスのロック待ち状態
pub const TTW_MTX: STAT = 0x0080;
/// 固定長メモリブロックの獲得待ち
pub const TTW_MPF: STAT = 0x2000;

/*
 *  その他の定数の定義
 */
/// 自タスク指定
pub const TSK_SELF: ID = 0;
/// 該当するタスクがない
pub const TSK_NONE: ID = 0;

/// 自タスクのベース優先度
pub const TPRI_SELF: PRI = 0;
/// タスクの起動時優先度
pub const TPRI_INI: PRI = 0;

/// TOPPERS/ASP3 dynamic creation extension `T_CTSK`
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre")
))]
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

/// SOLID/FMP3 extension
#[cfg(all(feature = "solid_fmp3", feature = "dcre"))]
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
    /// タスクの初期割付けプロセッサ
    pub iprcid: ID,
    /// タスクの割付け可能プロセッサ
    pub affinity: uint_t,
}

/// TOPPERS/ASP3 `T_RTSK`
#[cfg(any(feature = "asp3", feature = "solid_asp3"))]
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

/// TOPPERS/FMP3 `T_RTSK`
#[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
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
    /// 割付けプロセッサのID
    pub prcid: ID,
    /// 次の起動時の割付けプロセッサのID
    pub actprc: ID,
}

/// タスク管理機能
#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
extern "C" {
    pub fn act_tsk(tskid: ID) -> ER;
    pub fn can_act(tskid: ID) -> ER_UINT;
    pub fn get_tst(tskid: ID, p_tskstat: *mut STAT) -> ER;
    pub fn chg_pri(tskid: ID, tskpri: PRI) -> ER;
    pub fn get_pri(tskid: ID, p_tskpri: *mut PRI) -> ER;
    pub fn get_inf(p_exinf: *mut isize) -> ER;
    pub fn ref_tsk(tskid: ID, pk_rtsk: *mut T_RTSK) -> ER;
}

/// タスク管理機能
#[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
extern "C" {
    pub fn mact_tsk(tskid: ID, prcid: ID) -> ER;
    pub fn mig_tsk(tskid: ID, prcid: ID) -> ER;
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
extern "C" {
    pub fn acre_tsk(pk_ctsk: *const T_CTSK) -> ER_ID;
    pub fn del_tsk(tskid: ID) -> ER;
}

#[cfg(any(
    all(feature = "asp3", feature = "subprio"),
    feature = "fmp3",
    feature = "solid_fmp3"
))]
extern "C" {
    pub fn chg_spr(tskid: ID, subpri: uint_t) -> ER;
}

/// タスク付属同期機能
#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
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
#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
extern "C" {
    pub fn ext_tsk() -> ER;
    pub fn ras_ter(tskid: ID) -> ER;
    pub fn dis_ter() -> ER;
    pub fn ena_ter() -> ER;
    pub fn sns_ter() -> bool_t;
    pub fn ter_tsk(tskid: ID) -> ER;
}
