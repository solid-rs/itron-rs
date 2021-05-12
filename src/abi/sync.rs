use super::{uint_t, ER, FLGPTN, ID, MODE, PRI, RELTIM, TMO};

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
