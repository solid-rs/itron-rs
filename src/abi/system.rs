use super::{bool_t, uint_t, ER, ID, PRI};

/// システム状態管理機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn rot_rdq(tskpri: PRI) -> ER;
    pub fn get_tid(p_tskid: *mut ID) -> ER;
    pub fn get_lod(tskpri: PRI, p_load: *mut uint_t) -> ER;
    pub fn get_nth(tskpri: PRI, nth: uint_t, p_tskid: *mut ID) -> ER;
    pub fn loc_cpu() -> ER;
    pub fn unl_cpu() -> ER;
    pub fn dis_dsp() -> ER;
    pub fn ena_dsp() -> ER;
    pub fn sns_ctx() -> bool_t;
    pub fn sns_loc() -> bool_t;
    pub fn sns_dsp() -> bool_t;
    pub fn sns_dpn() -> bool_t;
    pub fn sns_ker() -> bool_t;
    pub fn ext_ker() -> ER;
}