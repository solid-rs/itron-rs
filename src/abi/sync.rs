use super::{uint_t, ATR, ER, ER_ID, FLGPTN, ID, MODE, PRI, RELTIM, TMO};

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RSEM {
    /// セマフォの待ち行列の先頭のタスクのID番号
    pub wtskid: ID,
    /// セマフォの現在の資源数
    pub semcnt: uint_t,
}

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RFLG {
    /// イベントフラグの待ち行列の先頭のタスクのID番号
    pub wtskid: ID,
    /// イベントフラグの現在のビットパターン
    pub flgptn: FLGPTN,
}

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RDTQ {
    /// データキューの送信待ち行列の先頭のタスクのID番号
    pub stskid: ID,
    /// データキューの受信待ち行列の先頭のタスクのID番号
    pub rtskid: ID,
    /// データキュー管理領域に格納されているデータの数
    pub sdtqcnt: uint_t,
}

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RPDQ {
    /// 優先度データキューの送信待ち行列の先頭のタスクのID番号
    pub stskid: ID,
    /// 優先度データキューの受信待ち行列の先頭のタスクのID番号
    pub rtskid: ID,
    /// 優先度データキュー管理領域に格納されているデータの数
    pub spdqcnt: uint_t,
}

#[cfg(feature = "asp3")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_RMTX {
    /// ミューテックスをロックしているタスクのID番号
    pub htskid: ID,
    /// ミューテックスの待ち行列の先頭のタスクのID番号
    pub wtskid: ID,
}

#[cfg(all(feature = "asp3", feature = "dcre"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_CSEM {
    /// セマフォ属性
    pub sematr: ATR,
    /// セマフォの初期資源数
    pub isemcnt: uint_t,
    /// セマフォの最大資源数
    pub maxsem: uint_t,
}

#[cfg(all(feature = "asp3", feature = "dcre"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_CFLG {
    /// イベントフラグ属性
    pub flgatr: ATR,
    /// イベントフラグの初期ビットパターン
    pub iflgptn: FLGPTN,
}

#[cfg(all(feature = "asp3", feature = "dcre"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_CDTQ {
    /// データキュー属性
    pub dtqatr: ATR,
    /// データキュー管理領域に格納できるデータ数
    pub dtqcnt: uint_t,
    /// データキュー管理領域の先頭番地
    pub dtqmb: *mut u8,
}

#[cfg(all(feature = "asp3", feature = "dcre"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_CPDQ {
    /// 優先度データキュー属性
    pub pdqatr: ATR,
    /// 優先度データキュー管理領域に格納できるデータ数
    pub pdqcnt: uint_t,
    /// 優先度データキューに送信できるデータ優先度の最大値
    pub maxdpri: PRI,
    /// 優先度データキュー管理領域の先頭番地
    pub pdqmb: *mut u8,
}

#[cfg(all(feature = "asp3", feature = "dcre"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct T_CMTX {
    /// ミューテックス属性
    pub mtxatr: ATR,
    /// ミューテックスの上限優先度
    pub ceilpri: PRI,
}

/// 同期・通信機能
#[cfg(feature = "asp3")]
extern "C" {
    pub fn sig_sem(semid: ID) -> ER;
    pub fn wai_sem(semid: ID) -> ER;
    pub fn pol_sem(semid: ID) -> ER;
    pub fn twai_sem(semid: ID, tmout: TMO) -> ER;
    pub fn ini_sem(semid: ID) -> ER;
    pub fn ref_sem(semid: ID, pk_rsem: *mut T_RSEM) -> ER;

    pub fn set_flg(flgid: ID, setptn: FLGPTN) -> ER;
    pub fn clr_flg(flgid: ID, clrptn: FLGPTN) -> ER;
    pub fn wai_flg(flgid: ID, waiptn: FLGPTN, wfmode: MODE, p_flgptn: *mut FLGPTN) -> ER;
    pub fn pol_flg(flgid: ID, waiptn: FLGPTN, wfmode: MODE, p_flgptn: *mut FLGPTN) -> ER;
    pub fn twai_flg(
        flgid: ID,
        waiptn: FLGPTN,
        wfmode: MODE,
        p_flgptn: *mut FLGPTN,
        tmout: TMO,
    ) -> ER;
    pub fn ini_flg(flgid: ID) -> ER;
    pub fn ref_flg(flgid: ID, pk_rflg: *mut T_RFLG) -> ER;

    pub fn snd_dtq(dtqid: ID, data: isize) -> ER;
    pub fn psnd_dtq(dtqid: ID, data: isize) -> ER;
    pub fn tsnd_dtq(dtqid: ID, data: isize, tmout: TMO) -> ER;
    pub fn fsnd_dtq(dtqid: ID, data: isize) -> ER;
    pub fn rcv_dtq(dtqid: ID, p_data: *mut isize) -> ER;
    pub fn prcv_dtq(dtqid: ID, p_data: *mut isize) -> ER;
    pub fn trcv_dtq(dtqid: ID, p_data: *mut isize, tmout: TMO) -> ER;
    pub fn ini_dtq(dtqid: ID) -> ER;
    pub fn ref_dtq(dtqid: ID, pk_rdtq: *mut T_RDTQ) -> ER;

    pub fn snd_pdq(pdqid: ID, data: isize, datapri: PRI) -> ER;
    pub fn psnd_pdq(pdqid: ID, data: isize, datapri: PRI) -> ER;
    pub fn tsnd_pdq(pdqid: ID, data: isize, datapri: PRI, tmout: TMO) -> ER;
    pub fn rcv_pdq(pdqid: ID, p_data: *mut isize, p_datapri: *mut PRI) -> ER;
    pub fn prcv_pdq(pdqid: ID, p_data: *mut isize, p_datapri: *mut PRI) -> ER;
    pub fn trcv_pdq(pdqid: ID, p_data: *mut isize, p_datapri: *mut PRI, tmout: TMO) -> ER;
    pub fn ini_pdq(pdqid: ID) -> ER;
    pub fn ref_pdq(pdqid: ID, pk_rpdq: *mut T_RPDQ) -> ER;

    pub fn loc_mtx(mtxid: ID) -> ER;
    pub fn ploc_mtx(mtxid: ID) -> ER;
    pub fn tloc_mtx(mtxid: ID, tmout: TMO) -> ER;
    pub fn unl_mtx(mtxid: ID) -> ER;
    pub fn ini_mtx(mtxid: ID) -> ER;
    pub fn ref_mtx(mtxid: ID, pk_rmtx: *mut T_RMTX) -> ER;
}

/// 同期・通信機能
#[cfg(all(feature = "asp3", feature = "dcre"))]
extern "C" {
    pub fn acre_sem(pk_csem: *const T_CSEM) -> ER_ID;
    pub fn acre_flg(pk_cflg: *const T_CFLG) -> ER_ID;
    pub fn acre_dtq(pk_cdtq: *const T_CDTQ) -> ER_ID;
    pub fn acre_pdq(pk_cpdq: *const T_CPDQ) -> ER_ID;
    pub fn acre_mtx(pk_cmtx: *const T_CMTX) -> ER_ID;
    pub fn del_sem(semid: ID) -> ER;
    pub fn del_flg(flgid: ID) -> ER;
    pub fn del_dtq(dtqid: ID) -> ER;
    pub fn del_pdq(pdqid: ID) -> ER;
    pub fn del_mtx(mtxid: ID) -> ER;
}
