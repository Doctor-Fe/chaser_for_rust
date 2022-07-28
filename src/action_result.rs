//! このファイルは、サーバーからの応答を加工した構造体を定義しています。

use std::u8;

/// サーバーからの応答を保持する構造体です。<br />
///
pub struct ActionResult {
    pub data: Vec<u8>,
    pub will_continue: bool,
}

impl ActionResult {
    /// この構造体のコンストラクタです。
    pub fn new(dat: [u8; 12]) -> ActionResult {
        return ActionResult {
            data: dat[1..10].iter().map(|a| *a - 48).collect(),
            will_continue: dat[0] != 48,
        };
    }
}
