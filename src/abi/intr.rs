use super::{bool_t, ER, ER_BOOL, INTNO, PRI};

/// 割込み管理機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn dis_int(intno: INTNO) -> ER;
    pub fn ena_int(intno: INTNO) -> ER;
    pub fn clr_int(intno: INTNO) -> ER;
    pub fn ras_int(intno: INTNO) -> ER;
    pub fn prb_int(intno: INTNO) -> ER_BOOL;
    pub fn chg_ipm(intpri: PRI) -> ER;
    pub fn get_ipm(p_intpri: *mut PRI) -> ER;
}

/// CPU例外管理機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn xsns_dpn(p_excinf: *mut u8) -> bool_t;
}
