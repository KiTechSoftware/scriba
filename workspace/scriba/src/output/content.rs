use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Structured output container with blocks and metadata.
///
/// Build up content using builder methods (`.heading()`, `.paragraph()`, etc.), then
/// render with `Ui::render()` or `Ui::print()` in your preferred format.
///
/// # Examples
///
/// ```
/// use scriba::{Output, StatusKind};
///
/// let output = Output::new()
///     .title("Report")
///     .heading(1, "Summary")
///     .paragraph("All systems operational")
///     .status(StatusKind::Ok, "Complete");
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<Block>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub data: BTreeMap<String, Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain: Option<Value>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jsonl_records: Vec<Value>,
}

impl Output {
    /// Create a new empty output.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title (rendered as # in Markdown, underlined in Text).
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Set the subtitle (rendered as italics in Markdown).
    pub fn subtitle(mut self, value: impl Into<String>) -> Self {
        self.subtitle = Some(value.into());
        self
    }

    /// Add structured data (key-value pair).
    ///
    /// Data is serialized and included in JSON/JSONL formats.
    pub fn data(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let value = serde_json::to_value(value).unwrap_or(Value::Null);
        self.data.insert(key.into(), value);
        self
    }

    /// Set a plain scalar value for rendering.
    ///
    /// Used with `Format::Plain` for simple output (string, number, boolean).
    pub fn plain(mut self, value: impl Serialize) -> Self {
        self.plain = Some(serde_json::to_value(value).unwrap_or(Value::Null));
        self
    }

    /// Add a JSONL record.
    ///
    /// Multiple records are rendered as newline-delimited JSON when using `Format::Jsonl`.
    pub fn jsonl_record(mut self, value: impl Serialize) -> Self {
        self.jsonl_records
            .push(serde_json::to_value(value).unwrap_or(Value::Null));
        self
    }

    /// Add a heading block.
    ///
    /// - `level`: 1-6 (levels 1-2 typically shown in most formats)
    /// - `text`: Heading content
    pub fn heading(mut self, level: u8, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Heading {
            level,
            text: text.into(),
        });
        self
    }

    /// Add a paragraph block.
    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Paragraph { text: text.into() });
        self
    }

    /// Add a single line of text.
    pub fn line(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Line { text: text.into() });
        self
    }

    /// Add a visual separator.
    pub fn separator(mut self) -> Self {
        self.blocks.push(Block::Separator);
        self
    }

    /// Add a list (ordered or unordered).
    ///
    /// - `ordered`: `true` for numbered list, `false` for bullet points
    pub fn list(mut self, ordered: bool, items: Vec<String>) -> Self {
        self.blocks.push(Block::List { ordered, items });
        self
    }

    /// Add a code block.
    ///
    /// - `language`: Optional language hint for syntax highlighting (e.g., "rust", "bash")
    /// - `code`: Source code content
    pub fn code(mut self, language: Option<String>, code: impl Into<String>) -> Self {
        self.blocks.push(Block::Code {
            language,
            code: code.into(),
        });
        self
    }

    /// Add a data table.
    pub fn table(mut self, title: Option<String>, table: Table) -> Self {
        self.blocks.push(Block::Table { title, table });
        self
    }

    /// Add a JSON data block.
    pub fn json(mut self, value: impl Serialize) -> Self {
        self.blocks.push(Block::Json {
            value: serde_json::to_value(value).unwrap_or(Value::Null),
        });
        self
    }

    /// Add or append a key-value pair.
    ///
    /// Consecutive calls group into a single block.
    pub fn key_value(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        let entry = KeyValueEntry {
            key: key.into(),
            value: value.to_string(),
        };

        match self.blocks.last_mut() {
            Some(Block::KeyValue { entries }) => entries.push(entry),
            _ => self.blocks.push(Block::KeyValue {
                entries: vec![entry],
            }),
        }

        self
    }

    /// Add or append a definition (term/description pair).
    ///
    /// Consecutive calls group into a single definition list.
    pub fn definition(mut self, term: impl Into<String>, description: impl Into<String>) -> Self {
        let entry = DefinitionEntry {
            term: term.into(),
            description: description.into(),
        };

        match self.blocks.last_mut() {
            Some(Block::DefinitionList { entries }) => entries.push(entry),
            _ => self.blocks.push(Block::DefinitionList {
                entries: vec![entry],
            }),
        }

        self
    }

    /// Add a status block (for success, warning, error messages).
    pub fn status(mut self, kind: StatusKind, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Status {
            kind,
            text: text.into(),
        });
        self
    }

    /// Add a section with a heading and code block.
    ///
    /// Convenience method that adds both a level-2 heading and code block.
    pub fn section(
        mut self,
        title: impl Into<String>,
        content: impl Into<String>,
        language: impl Into<Option<String>>,
    ) -> Self {
        self.blocks.push(Block::Heading {
            level: 2,
            text: title.into(),
        });

        self.blocks.push(Block::Code {
            language: language.into(),
            code: content.into(),
        });

        self
    }

    /// Create output from a serializable value.
    ///
    /// Converts objects to key-value data; other values are stored as data["value"].
    pub fn from_serializable(value: impl Serialize) -> Self {
        let json = serde_json::to_value(value).unwrap_or(Value::Null);

        match json {
            Value::Object(map) => Self {
                title: None,
                subtitle: None,
                blocks: Vec::new(),
                data: map.into_iter().collect(),
                plain: None,
                jsonl_records: Vec::new(),
            },
            other => Self::new().data("value", other),
        }
    }
}

/// A content block within an output.
///
/// Blocks are rendered according to the configured format (Markdown, JSON, etc.).
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    /// Heading at the specified level (1-6).
    Heading { level: u8, text: String },
    /// A paragraph of text.
    Paragraph { text: String },
    /// A single line of text.
    Line { text: String },
    /// A visual separator/divider.
    Separator,
    /// An ordered or unordered list.
    List { ordered: bool, items: Vec<String> },
    /// A code block with optional language hint.
    Code {
        language: Option<String>,
        code: String,
    },
    /// A structured data table.
    Table { title: Option<String>, table: Table },
    /// Raw JSON data.
    Json { value: Value },
    /// Key-value pairs.
    KeyValue { entries: Vec<KeyValueEntry> },
    /// Term definitions.
    DefinitionList { entries: Vec<DefinitionEntry> },
    /// Status indicator (ok, warning, error).
    Status { kind: StatusKind, text: String },
}

/// A structured data table with headers and rows.
///
/// # Examples
///
/// ```
/// use scriba::Table;
///
/// let table = Table::new(
///     vec!["Name".into(), "Value".into()],
///     vec![
///         vec!["Option A".into(), "1".into()],
///         vec!["Option B".into(), "2".into()],
///     ],
/// ).with_index();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Table {
    /// Column headers.
    pub headers: Vec<String>,
    /// Table rows.
    pub rows: Vec<Vec<String>>,
    #[serde(default)]
    /// Whether to show row numbers.
    pub show_index: bool,
    #[serde(default = "default_index_header")]
    /// Header label for index column (e.g., "#").
    pub index_header: String,
}

fn default_index_header() -> String {
    "#".to_string()
}

impl Table {
    /// Create a new table with headers and rows.
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            headers,
            rows,
            show_index: false,
            index_header: default_index_header(),
        }
    }

    /// Enable row numbering.
    pub fn with_index(mut self) -> Self {
        self.show_index = true;
        self
    }

    /// Set custom header for row number column.
    pub fn with_index_header(mut self, value: impl Into<String>) -> Self {
        self.show_index = true;
        self.index_header = value.into();
        self
    }

    /// Create table from borrowed string slices.
    pub fn from_slices(headers: &[&str], rows: &[Vec<String>]) -> Self {
        Self {
            headers: headers.iter().map(|s| (*s).to_string()).collect(),
            rows: rows.to_vec(),
            show_index: false,
            index_header: default_index_header(),
        }
    }

    /// Get a version of this table with row indices materialized.
    pub fn materialized(&self) -> Self {
        if !self.show_index {
            return self.clone();
        }

        let mut headers = Vec::with_capacity(self.headers.len() + 1);
        headers.push(self.index_header.clone());
        headers.extend(self.headers.clone());

        let rows = self
            .rows
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let mut new_row = Vec::with_capacity(row.len() + 1);
                new_row.push((idx + 1).to_string());
                new_row.extend(row.clone());
                new_row
            })
            .collect();

        Self {
            headers,
            rows,
            show_index: false,
            index_header: self.index_header.clone(),
        }
    }

    /// Convert table to JSON value.
    pub fn to_json_value(&self) -> Value {
        serde_json::to_value(self.materialized()).unwrap_or(Value::Null)
    }
}

/// A key-value entry for display.
#[derive(Debug, Clone, Serialize)]
pub struct KeyValueEntry {
    /// The key.
    pub key: String,
    /// The value.
    pub value: String,
}

impl KeyValueEntry {
    /// Create a new key-value entry.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// A term-definition pair for display in a definition list.
///
/// # Example
///
/// ```
/// use scriba::Output;
///
/// let output = Output::new()
///     .definition("JPEG", "A lossy image compression format");
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DefinitionEntry {
    /// The term being defined.
    pub term: String,
    /// The description or definition.
    pub description: String,
}

/// Status indicator kind for status blocks.
///
/// Used to indicate success, warnings, errors, or informational messages.
///
/// # Example
///
/// ```
/// use scriba::{Output, StatusKind};
///
/// let output = Output::new()
///     .status(StatusKind::Ok, "Deployment complete")
///     .status(StatusKind::Warning, "High resource usage");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusKind {
    /// Informational status.
    Info,
    /// Success status (typically green).
    Ok,
    /// Warning status (typically yellow/orange).
    Warning,
    /// Error status (typically red).
    Error,
    /// Deprecated: use `Ok` instead.
    #[deprecated(since = "0.2.0", note = "use `StatusKind::Ok` instead")]
    Success,
}
