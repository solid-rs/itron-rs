use super::ER;

/// システムエラー
pub const E_SYS: ER = -5;

/// 未サポート機能
pub const E_NOSPT: ER = -9;

/// 予約機能コード
pub const E_RSFN: ER = -10;

/// 予約属性
pub const E_RSATR: ER = -11;

/// パラメータエラー
pub const E_PAR: ER = -17;

/// 不正ID番号
pub const E_ID: ER = -18;

/// コンテキストエラー
pub const E_CTX: ER = -25;

/// メモリアクセス違反
pub const E_MACV: ER = -26;

/// オブジェクトアクセス違反
pub const E_OACV: ER = -27;

/// サービスコール不正使用
pub const E_ILUSE: ER = -28;

/// メモリ不足
pub const E_NOMEM: ER = -33;

/// ID番号不足
pub const E_NOID: ER = -34;

/// 資源不足
pub const E_NORES: ER = -35;

/// オブジェクト状態エラー
pub const E_OBJ: ER = -41;

/// オブジェクト未生成
pub const E_NOEXS: ER = -42;

/// キューイングオーバーフロー
pub const E_QOVR: ER = -43;

/// 待ち状態の強制解除
pub const E_RLWAI: ER = -49;

/// ポーリング失敗またはタイムアウト
pub const E_TMOUT: ER = -50;

/// 待ちオブジェクトの削除
pub const E_DLT: ER = -51;

/// 待ちオブジェクトの状態変化
pub const E_CLS: ER = -52;

/// タスクの終了要求
pub const E_RASTER: ER = -53;

/// ノンブロッキング受付け
pub const E_WBLK: ER = -57;

/// バッファオーバーフロー
pub const E_BOVR: ER = -58;

/// 通信エラー
pub const E_COMM: ER = -65;
