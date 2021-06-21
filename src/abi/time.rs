use super::{ER, HRTCNT, ID, RELTIM, STAT, SYSTIM};

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

// TODO: `acre_cyc`
// TODO: `acre_alm`

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
