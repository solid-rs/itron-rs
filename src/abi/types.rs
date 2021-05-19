//! Provides basic data type definitions and constants.
//!
//! Types that can be unambiguously mapped to Rust types, such as `int32_t`
//! but not `int_t`, should not have type aliases to minimize the maintenance
//! efforts.

/// 自然なサイズの符号付き整数
pub type int_t = i32;

/// 自然なサイズの符号無し整数
pub type uint_t = u32;

pub type bool_t = int_t;

pub const TRUE: bool_t = 1;
pub const FALSE: bool_t = 0;

/// 機能コード
pub type FN = int_t;

/// エラーコード
pub type ER = int_t;

// Non-zero version of [`ER`]
pub type NonZeroER = core::num::NonZeroI32;

/// オブジェクトのID番号
pub type ID = int_t;

/// Non-null version of [`ID`]
pub type NonNullID = core::num::NonZeroI32;

/// オブジェクトの属性
pub type ATR = uint_t;

/// オブジェクトの状態
pub type STAT = uint_t;

/// サービスコールの動作モード
pub type MODE = uint_t;

/// 優先度
pub type PRI = int_t;

/// タイムアウト指定
pub type TMO = u32;
/// 拡張情報
pub type EXINF = core::mem::MaybeUninit<isize>;
/// 相対時間
pub type RELTIM = u32;

// Assuming `defined(UINT64_MAX)`
/// システム時刻
pub type SYSTIM = u64;

// Assuming `USE_64BIT_HRTCNT`
/// 高分解能タイマのカウント値
pub type HRTCNT = u64;

/// プログラムの起動番地
pub type FP = unsafe fn();

/// エラーコードまたは真偽値
pub type ER_BOOL = int_t;
/// エラーコードまたはID番号
pub type ER_ID = int_t;
/// エラーコードまたは符号無し整数
pub type ER_UINT = int_t;

/// 管理領域を確保するためのデータ型
pub type MB_T = usize;

/// アクセス許可パターン
pub type ACPTN = u32;

/// アクセス許可ベクタ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct acvct {
    /// 通常操作1のアクセス許可パターン
    pub acptn1: ACPTN,
    /// 通常操作2のアクセス許可パターン
    pub acptn2: ACPTN,
    /// 管理操作のアクセス許可パターン
    pub acptn3: ACPTN,
    /// 参照操作のアクセス許可パターン
    pub acptn4: ACPTN,
}

/// イベントフラグのビットパターン
pub type FLGPTN = uint_t;

/// 割込み番号
pub type INTNO = uint_t;

/// 割込みハンドラ番号
pub type INHNO = uint_t;

/// CPU例外ハンドラ番号
pub type EXCNO = uint_t;

/*
 *  オブジェクト属性
 */
/// オブジェクト属性を指定しない */
pub const TA_NULL: ATR = 0;

/*
 *  タイムアウト指定
 */
/// ポーリング
pub const TMO_POL: TMO = 0;
/// 永久待ち
pub const TMO_FEVR: TMO = TMO::MAX;
/// ノンブロッキング
pub const TMO_NBLK: TMO = TMO::MAX - 1;

/// 相対時間（RELTIM）に指定できる最大値［NGKI0551］
///
/// 66分40秒まで指定可
pub const TMAX_RELTIM: TMO = 4_000_000_000;

/*
 *  アクセス許可パターン
 */
/// カーネルドメインだけにアクセスを許可
pub const TACP_KERNEL: ACPTN = 0;
/// すべてのドメインからアクセスを許可
pub const TACP_SHARED: ACPTN = !0;
