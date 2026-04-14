//! Output types and rendering for structured CLI content.
//!
//! Supports multiple formats: Plain, Text, Markdown, JSON, and JSONL.
//! Build `Output` using fluent builder methods, then render with `Ui::render()`.

pub mod content;
pub mod render;
pub mod table;

pub use content::{Block, DefinitionEntry, KeyValueEntry, Output, StatusKind, Table, TableLayout};

#[cfg(test)]
mod tests;
