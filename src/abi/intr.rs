use super::{bool_t, ER, ER_BOOL, INTNO, PRI};

/*
 *  処理単位の型定義
 */
pub type ISR = Option<unsafe extern "C" fn(super::EXINF)>;

/*
 *  その他の定数の定義
 */
/// 割込み優先度マスク全解除
pub const TIPM_ENAALL: PRI = 0;

/// TOPPERS/ASP3 and SOLID/FMP3 `T_CISR`
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct T_CISR {
    /// 割込みサービスルーチン属性
    pub isratr: super::ATR,
    /// 割込みサービスルーチンの拡張情報
    pub exinf: super::EXINF,
    /// 割込みサービスルーチンを登録する割込み番号
    pub intno: INTNO,
    /// 割込みサービスルーチンの先頭番地
    pub isr: ISR,
    /// 割込みサービスルーチン優先度
    pub isrpri: PRI,
}

/// 割込み管理機能
#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
extern "C" {
    pub fn dis_int(intno: INTNO) -> ER;
    pub fn ena_int(intno: INTNO) -> ER;
    pub fn clr_int(intno: INTNO) -> ER;
    pub fn ras_int(intno: INTNO) -> ER;
    pub fn prb_int(intno: INTNO) -> ER_BOOL;
    pub fn chg_ipm(intpri: PRI) -> ER;
    pub fn get_ipm(p_intpri: *mut PRI) -> ER;
}

/// 割込み管理機能
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
extern "C" {
    pub fn acre_isr(pk_cisr: *const T_CISR) -> super::ER_ID;
    pub fn del_isr(isrid: super::ID) -> ER;
}

/// CPU例外管理機能
#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
extern "C" {
    pub fn xsns_dpn(p_excinf: *mut u8) -> bool_t;
}
