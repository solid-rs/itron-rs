use super::{uint_t, ER, ID, TMO};

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RMPF {
    /// 固定長メモリプールの待ち行列の先頭のタスクのID番号
    pub wtskid: ID,
    /// 固定長メモリプール領域の空きメモリ領域に割り付けることができる固定長メモリブロックの数
    pub fblkcnt: uint_t,
}

/// メモリプール管理機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn get_mpf(mpfid: ID, p_blk: *mut *mut u8) -> ER;
    pub fn pget_mpf(mpfid: ID, p_blk: *mut *mut u8) -> ER;
    pub fn tget_mpf(mpfid: ID, p_blk: *mut *mut u8, tmout: TMO) -> ER;
    pub fn rel_mpf(mpfid: ID, blk: *mut u8) -> ER;
    pub fn ini_mpf(mpfid: ID) -> ER;
    pub fn ref_mpf(mpfid: ID, pk_rmpf: *mut T_RMPF) -> ER;
}
