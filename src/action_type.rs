//! このファイルでは、アクションの種類の列挙型を定義しています。

/// アクションの種類の列挙型です。
pub enum ActionType {
    Look = 0x6c,
    Move = 0x6d,
    Put = 0x70,
    Search = 0x73,
}
