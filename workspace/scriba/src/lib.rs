//! `scriba` — Typed CLI output, prompts, and terminal rendering for Rust.
//!
//! Build clean, structured command-line interfaces with composable output primitives,
//! interactive prompts, styled logging, and optional ASCII banners.
//!
//! # Features
//!
//! - 📄 **Multi-format rendering**: Plain, Text, Markdown, JSON, JSONL
//! - 🧱 **Typed output blocks**: Composable primitives for CLI content
//! - 💬 **Interactive prompts** (feature: `prompt`): Text, confirm, select, multiselect
//! - 🎨 **Styled logging** (feature: `logger`): Verbosity-aware stderr output
//! - 🔤 **ASCII art / figlet** (feature: `figlet`): Banner rendering
//! - ⚙️ **Feature-gated integrations**: Keep dependencies minimal
//!
//! # Quick Start
//!
//! ```ignore
//! use scriba::{Format, Output, Ui};
//!
//! fn main() -> scriba::Result<()> {
//!     let ui = Ui::new().with_format(Format::Markdown);
//!     let output = Output::new()
//!         .heading(1, "Hello")
//!         .paragraph("Clean CLI rendering");
//!     ui.print(&output)?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod envelope;
pub mod error;
pub mod output;
pub mod ui;

#[cfg(feature = "prompt")]
pub mod prompt;

#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "figlet")]
pub mod figlet;

pub use config::{ColorMode, Config, Format, Level};
pub use envelope::{EnvelopeConfig, EnvelopeFields, EnvelopeLayout, EnvelopeMode, Meta};
pub use error::{Result, ScribaError as Error};
pub use output::{Block, DefinitionEntry, KeyValueEntry, Output, StatusKind, Table};
pub use ui::Ui;

#[cfg(feature = "logger")]
pub use logger::Logger;

#[cfg(feature = "prompt")]
pub use prompt::{MultiSelectOption, MultiSelectRequest, SelectOption, SelectRequest};
