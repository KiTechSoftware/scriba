//! Interactive terminal prompts and form inputs.
//!
//! Requires the `prompt` feature. Access via `Ui` methods:
//! - `Ui::text()` — Text input
//! - `Ui::confirm()` — Yes/no confirmation
//! - `Ui::select()` — Choose one option
//! - `Ui::multiselect()` — Choose multiple options

mod inquire;

pub use inquire::{
    confirm, multiselect, select, text, MultiSelectOption, MultiSelectRequest, SelectOption,
    SelectRequest,
};
