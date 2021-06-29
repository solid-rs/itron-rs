use super::{uint_t, ATR, ER, ER_ID, ID, MPF_T, TMO};

#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RMPF {
    /// 固定長メモリプールの待ち行列の先頭のタスクのID番号
    pub wtskid: ID,
    /// 固定長メモリプール領域の空きメモリ領域に割り付けることができる固定長メモリブロックの数
    pub fblkcnt: uint_t,
}

#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_CMPF {
    /// 固定長メモリプール属性
    pub mpfatr: ATR,
    /// 獲得できる固定長メモリブロックの数
    pub blkcnt: uint_t,
    /// 固定長メモリブロックのサイズ
    pub blksz: uint_t,
    /// 固定長メモリプール領域の先頭番地
    pub mpf: *mut MPF_T,
    /// 固定長メモリプール管理領域の先頭番地
    pub mpfmb: *mut u8,
}

/// メモリプール管理機能
#[cfg(any(
    feature = "asp3",
    feature = "fmp3",
    feature = "solid_asp3",
    feature = "solid_fmp3"
))]
extern "C" {
    pub fn get_mpf(mpfid: ID, p_blk: *mut *mut u8) -> ER;
    pub fn pget_mpf(mpfid: ID, p_blk: *mut *mut u8) -> ER;
    pub fn tget_mpf(mpfid: ID, p_blk: *mut *mut u8, tmout: TMO) -> ER;
    pub fn rel_mpf(mpfid: ID, blk: *mut u8) -> ER;
    pub fn ini_mpf(mpfid: ID) -> ER;
    pub fn ref_mpf(mpfid: ID, pk_rmpf: *mut T_RMPF) -> ER;
}

/// メモリプール管理機能
#[cfg(any(
    all(feature = "asp3", feature = "dcre"),
    all(feature = "solid_asp3", feature = "dcre"),
    all(feature = "solid_fmp3", feature = "dcre")
))]
extern "C" {
    pub fn acre_mpf(pk_cmpf: *const T_CMPF) -> ER_ID;
    pub fn del_mpf(mpfid: ID) -> ER;
}
