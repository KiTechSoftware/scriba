//! Output types and rendering for structured CLI content.
//!
//! Supports multiple formats: Plain, Text, Markdown, JSON, and JSONL.
//! Build `Output` using fluent builder methods, then render with `Ui::render()`.

pub mod content;
pub mod diff;
pub mod render;
pub mod style;
pub mod table;

pub use content::{Block, DefinitionEntry, KeyValueEntry, Output, StatusKind, Table, TableLayout};
pub use diff::{format_diff_for_scriba, parse_diff, render_colored_diff, DiffLine, DiffLineKind};
pub use style::{Styled, TextStyle};

#[cfg(test)]
mod tests;
