use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn subtitle(mut self, value: impl Into<String>) -> Self {
        self.subtitle = Some(value.into());
        self
    }

    pub fn data(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let value = serde_json::to_value(value).unwrap_or(Value::Null);
        self.data.insert(key.into(), value);
        self
    }

    pub fn plain(mut self, value: impl Serialize) -> Self {
        self.plain = Some(serde_json::to_value(value).unwrap_or(Value::Null));
        self
    }

    pub fn jsonl_record(mut self, value: impl Serialize) -> Self {
        self.jsonl_records
            .push(serde_json::to_value(value).unwrap_or(Value::Null));
        self
    }

    pub fn heading(mut self, level: u8, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Heading {
            level,
            text: text.into(),
        });
        self
    }

    pub fn paragraph(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Paragraph { text: text.into() });
        self
    }

    pub fn line(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Line { text: text.into() });
        self
    }

    pub fn separator(mut self) -> Self {
        self.blocks.push(Block::Separator);
        self
    }

    pub fn list(mut self, ordered: bool, items: Vec<String>) -> Self {
        self.blocks.push(Block::List { ordered, items });
        self
    }

    pub fn code(mut self, language: Option<String>, code: impl Into<String>) -> Self {
        self.blocks.push(Block::Code {
            language,
            code: code.into(),
        });
        self
    }

    pub fn table(mut self, title: Option<String>, table: Table) -> Self {
        self.blocks.push(Block::Table { title, table });
        self
    }

    pub fn json(mut self, value: impl Serialize) -> Self {
        self.blocks.push(Block::Json {
            value: serde_json::to_value(value).unwrap_or(Value::Null),
        });
        self
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph { text: String },
    Line { text: String },
    Separator,
    List { ordered: bool, items: Vec<String> },
    Code { language: Option<String>, code: String },
    Table { title: Option<String>, table: Table },
    Json { value: Value },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    #[serde(default)]
    pub show_index: bool,
    #[serde(default = "default_index_header")]
    pub index_header: String,
}

fn default_index_header() -> String {
    "#".to_string()
}

impl Table {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            headers,
            rows,
            show_index: false,
            index_header: default_index_header(),
        }
    }

    pub fn with_index(mut self) -> Self {
        self.show_index = true;
        self
    }

    pub fn with_index_header(mut self, value: impl Into<String>) -> Self {
        self.show_index = true;
        self.index_header = value.into();
        self
    }

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
}