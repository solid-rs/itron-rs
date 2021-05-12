use super::{ER, HRTCNT, ID, RELTIM, STAT, SYSTIM};

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RCYC {
    /// 周期通知の動作状態
    pub cycstat: STAT,
    /// 次回通知時刻までの相対時間
    pub lefttim: RELTIM,
}

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RALM {
    /// アラーム通知の動作状態
    pub almstat: STAT,
    /// 通知時刻までの相対時間
    pub lefttim: RELTIM,
}

/// 時間管理機能
#[cfg(feature = "asp3")]
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
