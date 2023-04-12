use serde::{Deserialize, Serialize};

/// あるコンポーネントのコマンド定義のデータベース
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Database {
    /// このコマンドに含まれる [Entry] のリスト
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum Entry {
    /// コマンド定義行
    Command(Command),
    /// コメント行
    Comment(Comment),
}

/// コマンド定義
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    /// コマンド名
    pub name: String,
    pub target: String,
    /// コマンドのID
    pub code: u16,
    /// コマンドのパラメータのリスト。要素数は0以上6個以下
    pub parameters: Vec<Parameter>,
    pub is_danger: bool,
    pub is_restricted: bool,
    /// コマンドの説明（衛星運用者向け）
    pub description: String,
    /// コマンドの説明（衛星開発者向け）
    pub note: String,
}

/// コマンドのパラメータ定義
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    /// パラメータのデータ型
    pub data_type: DataType,
    /// パラメータの説明
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    /// 符号あり8bit整数
    #[serde(rename = "int8_t")]
    Int8,
    /// 符号あり16bit整数
    #[serde(rename = "int16_t")]
    Int16,
    /// 符号あり32bit整数
    #[serde(rename = "int32_t")]
    Int32,
    /// 符号なし8bit整数
    #[serde(rename = "uint8_t")]
    Uint8,
    /// 符号なし16bit整数
    #[serde(rename = "uint16_t")]
    Uint16,
    /// 符号なし32bit整数
    #[serde(rename = "uint32_t")]
    Uint32,
    /// IEEE 754 単精度浮動小数
    #[serde(rename = "float")]
    Float,
    /// IEEE 754 倍精度浮動小数
    #[serde(rename = "double")]
    Double,
    /// 生データ, 可変長パラメータ
    #[serde(rename = "raw")]
    Raw,
}

/// コメント行
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    /// コメントの内容
    pub text: String,
}
