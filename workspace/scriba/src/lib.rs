pub mod config;
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
pub use error::{Result, ScribaError as Error};
pub use output::{Block, DefinitionEntry, KeyValueEntry, Output, StatusKind, Table};
pub use ui::Ui;

#[cfg(feature = "logger")]
pub use logger::Logger;

#[cfg(feature = "prompt")]
pub use prompt::{MultiSelectOption, MultiSelectRequest, SelectOption, SelectRequest};
