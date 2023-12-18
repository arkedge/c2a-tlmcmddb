use serde::{Deserialize, Serialize};

/// あるコンポーネントのテレメトリ定義のデータベース
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Database {
    /// データベースに含まれるテレメトリ定義のリスト
    pub telemetries: Vec<Telemetry>,
}

/// テレメトリの定義
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Telemetry {
    /// このテレメトリ定義の名前
    pub name: String,
    /// このテレメトリ定義のメタデータ
    pub metadata: Metadata,
    /// このテレメトリの構造定義
    /// blobが追加される前との互換性のため、entriesをaliasとする
    #[serde(alias = "entries")]
    pub content: Content,
}

/// テレメトリ定義のメタデータ
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    pub target: String,
    /// テレメトリ定義のID。SH.TLM_IDと一致する
    pub packet_id: u8,
    pub is_enabled: bool,
    pub is_restricted: bool,
    pub local_variables: String,
}

/// バイト列を解釈しなblob tlmと、entryのリストとして解釈されるstruct tlmがある
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// blob が追加される前との互換性のため、untaggedとする
#[serde(untagged)]
pub enum Content {
    /// このテレメトリはblobであり、構造をもたない
    Blob,
    /// このテレメトリ定義に含まれる [Entry] のリスト
    Struct(Vec<Entry>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum Entry {
    /// ビットフィールドの集合
    FieldGroup(FieldGroup),
    /// コメント行
    Comment(Comment),
}

/// ビットフィールドの集合
///
/// TLM DB CSVにおいて縦方向のセル結合で表現されているもの。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldGroup {
    /// 搭載ソフトウェアのコード生成に必要な情報
    pub onboard_software_info: OnboardSoftwareInfo,
    /// この [`FieldGroup`] に含まれる [SubEntry] のリスト
    pub sub_entries: Vec<SubEntry>,
}

/// [FieldGroup] 内のエントリ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type")]
pub enum SubEntry {
    Field(Field),
    Comment(Comment),
}

/// オクテットアラインされていないフィールド（ビットフィールド）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    /// テレメトリのオクテット列からこのフィールドの値を抜き出す際に必要な情報
    pub extraction_info: FieldExtractionInfo,
    pub conversion_info: ConversionInfo,
    /// このフィールドの説明（衛星運用者向け）
    pub description: String,
    /// このフィールドの説明（衛星開発者向け）
    pub note: String,
}

/// [Field] の値をテレメトリのオクテット列から抜き出す際に必要な情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldExtractionInfo {
    /// 未使用。通常は `"PACKET"` で固定。SIB2 由来
    pub extraction_type: String,
    /// このフィールドの値のMSBが、テレメトリのオクテット列内の `octet_position` オクテット目にあることを示す
    pub octet_position: usize,
    /// このフィールドの値のMSBが、`octet_position` オクテット目の `bit_position` ビット目（MSBを0とする）にあることを示す
    pub bit_position: usize,
    /// このフィールドの値のビット幅を示す
    pub bit_length: usize,
}

/// 搭載ソフトウェアのコード生成に必要な情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardSoftwareInfo {
    /// 搭載ソフトウェアにおいて [FieldGroup] の値を表現するために用いるデータ型
    ///
    /// [FieldGroup] に複数の [Field] が含まれる場合は符号なし整数型([`Uint8`](VariableType::Uint8), [`Uint16`](VariableType::Uint16), [`Uint32`](VariableType::Uint32))でなければならない。
    pub variable_type: VariableType,
    /// 搭載ソフトウェアにおいて、このフィールドの値を組み立てるための式
    pub expression: String,
}

/// 工学値変換の規則
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConversionInfo {
    /// 変換なし（工学値は生値と同一）
    None,
    /// 変換なし。地上局ソフトウェアで工学値を表示する際は16進数で表示する
    Hex,
    /// ステータス変換。整数値に対応する文字列を定義し、その文字列を工学値とする
    Status(conversion::Status),
    /// 多項式変換。ここで定義した係数からなる多項式において、生値を不定元とした値を工学値とする
    Polynomial(conversion::Polynomial),
}

pub mod conversion {
    use serde::{Deserialize, Serialize};

    /// ステータス変換の規則の定義
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Status {
        /// 整数値と文字列の対応のリスト
        pub variants: Vec<Variant>,
        /// `variants` で定義されていない整数値に対応する文字列
        pub default_value: Option<String>,
    }

    /// ステータス変換に用いる整数値と文字列の対応
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Variant {
        /// 変換前の整数値
        pub key: i64,
        /// 変換後の文字列
        pub value: String,
    }

    /// 多項式変換に用いる係数
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Polynomial {
        pub a0: f64,
        pub a1: f64,
        pub a2: f64,
        pub a3: f64,
        pub a4: f64,
        pub a5: f64,
    }
}

/// 搭載ソフトウェアにおいてフィールドの値を表現するために用いるデータ型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariableType {
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
}

impl VariableType {
    /// オクテット幅
    pub fn octet_width(&self) -> usize {
        match self {
            VariableType::Int8 => 1,
            VariableType::Int16 => 2,
            VariableType::Int32 => 4,
            VariableType::Uint8 => 1,
            VariableType::Uint16 => 2,
            VariableType::Uint32 => 4,
            VariableType::Float => 4,
            VariableType::Double => 8,
        }
    }

    /// ビット幅
    pub fn bit_width(&self) -> usize {
        self.octet_width() * 8
    }

    /// 符号なし整数型であるかどうか
    pub fn is_unsigned_integer(&self) -> bool {
        matches!(
            self,
            VariableType::Uint8 | VariableType::Uint16 | VariableType::Uint32
        )
    }
}

/// コメント行
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    /// コメントの内容
    pub text: String,
}
