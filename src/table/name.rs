
#[derive(Debug)]
pub struct NamingTable {
    /// Format selector (=1 if some language-tag, =0 if none).
    pub format: u16,
    /// Number of name records.
    pub count: u16,
    /// Offset to start of string storage (from start of table).
    pub string_offset: u16,
    /// The name records where count is the number of records.
    pub name_records: Vec<NameRecord>,
    /// Number of language-tag records.
    pub lang_tag_count: u16,
    /// The language-tag records where lang_tag_count is the number of records.
    pub lang_tag_records: Vec<LangTagRecord>,
}

#[derive(Debug)]
pub struct LangTagRecord {
    /// Language-tag string length (in bytes)
    pub length: u16,
    /// Language-tag string offset from start of storage area (in bytes).
    pub offset: u16,
}

#[derive(Debug)]
pub struct NameRecord {
    /// Platform ID.
    pub platform_id: u16,
    /// Platform-specific encoding ID.
    pub encoding_id: u16,
    /// Language ID.
    pub language_id: u16,
    /// Name ID.
    pub name_id: u16,
    /// String length (in bytes).
    pub length: u16,
    /// String offset from start of storage area (in bytes).
    pub offset: u16,
}
