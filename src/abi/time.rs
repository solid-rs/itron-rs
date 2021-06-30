#![allow(unused_imports)]
use super::{uint_t, ATR, ER, ER_ID, EXINF, FLGPTN, HRTCNT, ID, MODE, RELTIM, STAT, SYSTIM};

/*
 *  処理単位の型定義
 */
pub type TMEHDR = Option<unsafe extern "C" fn(EXINF)>;

/*
 *  オブジェクト属性
 */
/// 周期通知を動作状態で生成
pub const TA_STA: ATR = 0x02;

/*
 *  通知処理モードの定義
 */
/// タイムイベントハンドラの呼出し
pub const TNFY_HANDLER: MODE = 0x00;
/// 変数の設定
pub const TNFY_SETVAR: MODE = 0x01;
/// 変数のインクリメント
pub const TNFY_INCVAR: MODE = 0x02;
/// タスクの起動
pub const TNFY_ACTTSK: MODE = 0x03;
/// タスクの起床
pub const TNFY_WUPTSK: MODE = 0x04;
/// セマフォの資源の返却
pub const TNFY_SIGSEM: MODE = 0x05;
/// イベントフラグのセット
pub const TNFY_SETFLG: MODE = 0x06;
/// データキューへの送信
pub const TNFY_SNDDTQ: MODE = 0x07;

/// 変数の設定
pub const TENFY_SETVAR: MODE = 0x10;
/// 変数のインクリメント
pub const TENFY_INCVAR: MODE = 0x20;
/// タスクの起動
pub const TENFY_ACTTSK: MODE = 0x30;
/// タスクの起床
pub const TENFY_WUPTSK: MODE = 0x40;
/// セマフォの返却
pub const TENFY_SIGSEM: MODE = 0x50;
/// イベントフラグのセット
pub const TENFY_SETFLG: MODE = 0x60;
/// データキューへの送信
pub const TENFY_SNDDTQ: MODE = 0x70;

/// TOPPERS/ASP3 `T_RCYC`
#[cfg(any(feature = "asp3", feature = "solid_asp3"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RCYC {
    /// 周期通知の動作状態
    pub cycstat: STAT,
    /// 次回通知時刻までの相対時間
    pub lefttim: RELTIM,
}

/// TOPPERS/FMP3 `T_RCYC`
#[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RCYC {
    /// 周期通知の動作状態
    pub cycstat: STAT,
    /// 次回通知時刻までの相対時間
    pub lefttim: RELTIM,
    /// 割付けプロセッサのID
    pub prcid: ID,
}

/// TOPPERS/ASP3 `T_RALM`
#[cfg(any(feature = "asp3", feature = "solid_asp3"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RALM {
    /// アラーム通知の動作状態
    pub almstat: STAT,
    /// 通知時刻までの相対時間
    pub lefttim: RELTIM,
}

/// TOPPERS/ASP3 `T_RALM`
#[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RALM {
    /// アラーム通知の動作状態
    pub almstat: STAT,
    /// 通知時刻までの相対時間
    pub lefttim: RELTIM,
    /// 割付けプロセッサのID
    pub prcid: ID,
}

#[cfg(all(feature = "asp3", feature = "ovrhdr"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_ROVR {
    /// オーバランハンドラの動作状態 */
    pub ovrstat: STAT,
    /// 残りプロセッサ時間 */
    pub leftotm: super::PRCTIM,
}

/// TOPPERS/ASP3 `T_CCYC`
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre")
))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct T_CCYC {
    /// 周期通知属性
    pub cycatr: ATR,
    /// 周期通知の通知方法
    pub nfyinfo: T_NFYINFO,
    /// 周期通知の通知周期
    pub cyctim: RELTIM,
    /// 周期通知の通知位相
    pub cycphs: RELTIM,
}

/// SOLID/FMP3 `T_CCYC`
#[cfg(all(feature = "solid_fmp3", feature = "dcre"))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct T_CCYC {
    /// 周期通知属性
    pub cycatr: ATR,
    /// 周期通知の通知方法
    pub nfyinfo: T_NFYINFO,
    /// 周期通知の通知周期
    pub cyctim: RELTIM,
    /// 周期通知の通知位相
    pub cycphs: RELTIM,
    #[cfg(feature = "systim_local")]
    /// 周期通知の初期割付けプロセッサ
    pub iprcid: ID,
    #[cfg(feature = "systim_local")]
    /// 周期通知の割付け可能プロセッサ
    pub affinity: uint_t,
}

/// TOPPERS/ASP3 `T_CALM`
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre")
))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct T_CALM {
    /// アラーム通知属性
    pub almatr: ATR,
    /// アラーム通知の通知方法
    pub nfyinfo: T_NFYINFO,
}

/// SOLID/FMP3 `T_CALM`
#[cfg(all(feature = "solid_fmp3", feature = "dcre"))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct T_CALM {
    /// アラーム通知属性
    pub almatr: ATR,
    /// アラーム通知の通知方法
    pub nfyinfo: T_NFYINFO,
    #[cfg(feature = "systim_local")]
    /// アラーム通知の初期割付けプロセッサ
    pub iprcid: ID,
    #[cfg(feature = "systim_local")]
    /// アラーム通知の割付け可能プロセッサ
    pub affinity: uint_t,
}

/*
 *  タイムイベントの通知方法のパケット形式の定義
 */
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct T_NFYINFO {
    /// 通知処理モード
    pub nfymode: MODE,
    /// タイムイベントの通知に関する付随情報
    pub nfy: T_NFY,
    /// エラーの通知に関する付随情報
    pub enfy: T_ENFY,
}

/// [`T_NFYINFO::nfy`]
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Clone, Copy)]
#[repr(C)]
pub union T_NFY {
    pub handler: T_NFY_HDR,
    pub setvar: T_NFY_VAR,
    pub incvar: T_NFY_IVAR,
    pub acttsk: T_NFY_TSK,
    pub wuptsk: T_NFY_TSK,
    pub sigsem: T_NFY_SEM,
    pub setflg: T_NFY_FLG,
    pub snddtq: T_NFY_DTQ,
}

/// [`T_NFYINFO::enfy`]
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Clone, Copy)]
#[repr(C)]
pub union T_ENFY {
    pub setvar: T_ENFY_VAR,
    pub incvar: T_NFY_IVAR,
    pub acttsk: T_NFY_TSK,
    pub wuptsk: T_NFY_TSK,
    pub sigsem: T_NFY_SEM,
    pub setflg: T_NFY_FLG,
    pub snddtq: T_ENFY_DTQ,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct T_NFY_HDR {
    /// タイムイベントハンドラの拡張情報
    pub exinf: EXINF,
    /// タイムイベントハンドラの先頭番地
    pub tmehdr: TMEHDR,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_NFY_VAR {
    /// 変数の番地
    pub p_var: *mut isize,
    /// 設定する値
    pub value: isize,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_NFY_IVAR {
    /// 変数の番地
    pub p_var: *mut isize,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_NFY_TSK {
    /// タスクID
    pub tskid: ID,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_NFY_SEM {
    /// セマフォID
    pub semid: ID,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_NFY_FLG {
    /// イベントフラグID
    pub flgid: ID,
    #[cfg(all(
        any(feature = "solid_asp3", feature = "solid_fmp3"),
        target_pointer_width = "64",
    ))]
    pub __pad_for_aarch64: u32,
    /// セットするビットパターン
    pub flgptn: FLGPTN,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_NFY_DTQ {
    /// データキューID
    pub dtqid: ID,
    #[cfg(all(
        any(feature = "solid_asp3", feature = "solid_fmp3"),
        target_pointer_width = "64",
    ))]
    pub __pad_for_aarch64: u32,
    /// 送信する値
    pub data: isize,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_ENFY_VAR {
    /// 変数の番地
    pub p_var: *mut isize,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_ENFY_DTQ {
    /// データキューID
    pub dtqid: ID,
}

/// 時間管理機能
#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
extern "C" {
    pub fn set_tim(systim: SYSTIM) -> ER;
    pub fn get_tim(p_systim: *mut SYSTIM) -> ER;
    pub fn adj_tim(adjtim: i32) -> ER;
    pub fn fch_hrt() -> HRTCNT;

    pub fn sta_cyc(cycid: ID) -> ER;
    pub fn stp_cyc(cycid: ID) -> ER;
    pub fn ref_cyc(cycid: ID, pk_rcyc: *mut T_RCYC) -> ER;

    pub fn sta_alm(almid: ID, almtim: RELTIM) -> ER;
    pub fn stp_alm(almid: ID) -> ER;
    pub fn ref_alm(almid: ID, pk_ralm: *mut T_RALM) -> ER;
}

/// 時間管理機能
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
extern "C" {
    pub fn acre_cyc(pk_ccyc: *const T_CCYC) -> ER_ID;
    pub fn acre_alm(pk_calm: *const T_CALM) -> ER_ID;
    pub fn del_cyc(cycid: ID) -> ER;
    pub fn del_alm(almid: ID) -> ER;
}

/// 時間管理機能
#[cfg(any(feature = "fmp3", feature = "solid_fmp3"))]
extern "C" {
    pub fn msta_cyc(cycid: ID, prcid: ID) -> ER;
    pub fn msta_alm(almid: ID, almtim: RELTIM, prcid: ID) -> ER;
}

/// 時間管理機能
#[cfg(all(feature = "asp3", feature = "ovrhdr"))]
extern "C" {
    pub fn sta_ovr(tskid: ID, ovrtim: super::PRCTIM) -> ER;
    pub fn stp_ovr(tskid: ID) -> ER;
    pub fn ref_ovr(tskid: ID, pk_rovr: *mut T_ROVR) -> ER;
}
