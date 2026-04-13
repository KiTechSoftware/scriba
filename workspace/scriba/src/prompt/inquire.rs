use ::inquire::{
    ui::{Color, RenderConfig, StyleSheet, Styled},
    Confirm, MultiSelect, Select, Text,
};

use crate::{Config, Error, Result};

/// A selectable option with id and label.
///
/// Used in `SelectRequest` for single-choice prompts.
///
/// # Example
///
/// ```
/// use scriba::SelectOption;
///
/// let option = SelectOption::new("dev", "Development")
///     .description("Local development environment");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    /// Unique identifier for this option.
    pub id: String,
    /// Display label for this option.
    pub label: String,
    /// Optional description shown below label.
    pub description: Option<String>,
}

impl SelectOption {
    /// Create a new selectable option.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
        }
    }

    /// Add a description to this option.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    fn display(&self) -> String {
        match &self.description {
            Some(description) => format!("{} — {}", self.label, description),
            None => self.label.clone(),
        }
    }
}

/// A selectable option for multi-select prompts.
///
/// Used in `MultiSelectRequest`. Can be pre-selected.
///
/// # Example
///
/// ```
/// use scriba::MultiSelectOption;
///
/// let option = MultiSelectOption::new("logger", "Logger")
///     .description("Styled logging")
///     .selected(true);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSelectOption {
    /// Unique identifier for this option.
    pub id: String,
    /// Display label for this option.
    pub label: String,
    /// Optional description shown below label.
    pub description: Option<String>,
    /// Whether this option is pre-selected.
    pub selected: bool,
}

impl MultiSelectOption {
    /// Create a new multi-select option.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            selected: false,
        }
    }

    /// Add a description to this option.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Set whether this option is pre-selected.
    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }

    fn display(&self) -> String {
        match &self.description {
            Some(description) => format!("{} — {}", self.label, description),
            None => self.label.clone(),
        }
    }
}

/// Request for single-choice selection prompt.
///
/// # Example
///
/// ```
/// use scriba::{SelectRequest, SelectOption};
///
/// let request = SelectRequest::new(
///     "Choose environment",
///     vec![
///         SelectOption::new("dev", "Development"),
///         SelectOption::new("prod", "Production"),
///     ],
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectRequest {
    /// Prompt message to display.
    pub message: String,
    /// Available options to choose from.
    pub options: Vec<SelectOption>,
}

impl SelectRequest {
    /// Create a new select request.
    pub fn new(message: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            message: message.into(),
            options,
        }
    }
}

/// Request for multi-choice selection prompt.
///
/// Supports optional pagination via `with_page_size()` for large option lists.
///
/// # Example
///
/// ```
/// use scriba::{MultiSelectRequest, MultiSelectOption};
///
/// let request = MultiSelectRequest::new(
///     "Select features",
///     vec![
///         MultiSelectOption::new("logging", "Logging").selected(true),
///         MultiSelectOption::new("prompts", "Prompts"),
///     ],
/// )
/// .with_page_size(5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSelectRequest {
    /// Prompt message to display.
    pub message: String,
    /// Available options to choose from.
    pub options: Vec<MultiSelectOption>,
    /// Optional page size for pagination (default: 7).
    pub page_size: Option<usize>,
}

impl MultiSelectRequest {
    /// Create a new multi-select request.
    pub fn new(message: impl Into<String>, options: Vec<MultiSelectOption>) -> Self {
        Self {
            message: message.into(),
            options,
            page_size: None,
        }
    }

    /// Set pagination page size for large option lists.
    ///
    /// If the number of options exceeds this size, users can scroll through pages.
    /// When `None` (default), inquire uses its default page size of 7.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{MultiSelectRequest, MultiSelectOption};
    ///
    /// let request = MultiSelectRequest::new(
    ///     "Select items",
    ///     vec![MultiSelectOption::new("item1", "Item 1")],
    /// )
    /// .with_page_size(10);
    /// ```
    pub fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = Some(size);
        self
    }
}

fn theme<'a>() -> RenderConfig<'a> {
    RenderConfig::default_colored()
        .with_prompt_prefix(Styled::new("◇").with_fg(Color::LightCyan))
        .with_answered_prompt_prefix(Styled::new("✔").with_fg(Color::LightGreen))
        .with_canceled_prompt_indicator(Styled::new("✖").with_fg(Color::DarkGrey))
        .with_highlighted_option_prefix(Styled::new("›").with_fg(Color::LightCyan))
        .with_scroll_up_prefix(Styled::new("↑").with_fg(Color::DarkGrey))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(Color::DarkGrey))
        .with_selected_checkbox(Styled::new("●").with_fg(Color::LightGreen))
        .with_unselected_checkbox(Styled::new("○").with_fg(Color::DarkGrey))
        .with_help_message(StyleSheet::new().with_fg(Color::DarkGrey))
        .with_answer(StyleSheet::new().with_fg(Color::LightGreen))
        .with_option(StyleSheet::new().with_fg(Color::White))
        .with_selected_option(Some(StyleSheet::new().with_fg(Color::LightCyan)))
        .with_text_input(StyleSheet::new().with_fg(Color::White))
        .with_default_value(StyleSheet::new().with_fg(Color::DarkGrey))
}

/// Prompt for text input.
///
/// Returns `Error::InteractiveRequired` if not in interactive mode.
///
/// # Example
///
/// ```ignore
/// let response = scriba::prompt::text(
///     &config,
///     "Your name?",
///     Some("Anonymous"),
///     None,
/// )?;
/// ```
pub fn text(
    cfg: &Config,
    message: &str,
    default: Option<&str>,
    help: Option<&str>,
) -> Result<String> {
    if !cfg.interactive {
        return Err(Error::InteractiveRequired);
    }

    let mut prompt = Text::new(message).with_render_config(theme());

    if let Some(default) = default {
        prompt = prompt.with_default(default);
    }

    if let Some(help) = help {
        prompt = prompt.with_help_message(help);
    }

    prompt.prompt().map_err(map_inquire_error)
}

/// Prompt for yes/no confirmation.
///
/// Auto-returns `Ok(true)` if `config.auto_yes` is enabled.
/// Returns the default value if not in interactive mode.
///
/// # Example
///
/// ```ignore
/// if scriba::prompt::confirm(&config, "Continue?", false)? {
///     println!("Confirmed!");
/// }
/// ```
pub fn confirm(cfg: &Config, message: &str, default: bool) -> Result<bool> {
    if cfg.auto_yes {
        return Ok(true);
    }

    if !cfg.interactive {
        return Ok(default);
    }

    Confirm::new(message)
        .with_render_config(theme())
        .with_default(default)
        .prompt()
        .map_err(map_inquire_error)
}

/// Prompt user to select one option from a list.
///
/// Returns the `id` of the selected option.
///
/// # Example
///
/// ```ignore
/// use scriba::{prompt, SelectRequest, SelectOption};
///
/// let request = SelectRequest::new(
///     "Pick one",
///     vec![SelectOption::new("a", "Option A")],
/// );
/// let id = prompt::select(&config, &request)?;
/// ```
pub fn select(cfg: &Config, request: &SelectRequest) -> Result<String> {
    if !cfg.interactive {
        return Err(Error::InteractiveRequired);
    }

    let options = request
        .options
        .iter()
        .map(SelectOption::display)
        .collect::<Vec<_>>();

    let selected = Select::new(&request.message, options)
        .with_render_config(theme())
        .prompt()
        .map_err(map_inquire_error)?;

    let id = request
        .options
        .iter()
        .find(|option| option.display() == selected)
        .map(|option| option.id.clone())
        .ok_or_else(|| Error::Prompt("selected option was not found".to_string()))?;

    Ok(id)
}

/// Prompt user to select multiple options from a list.
///
/// Returns the `id`s of the selected options.
///
/// # Example
///
/// ```ignore
/// use scriba::{prompt, MultiSelectRequest, MultiSelectOption};
///
/// let request = MultiSelectRequest::new(
///     "Pick multiple",
///     vec![MultiSelectOption::new("a", "Option A")],
/// );
/// let ids = prompt::multiselect(&config, &request)?;
/// ```
pub fn multiselect(cfg: &Config, request: &MultiSelectRequest) -> Result<Vec<String>> {
    if !cfg.interactive {
        return Err(Error::InteractiveRequired);
    }

    let options = request
        .options
        .iter()
        .map(MultiSelectOption::display)
        .collect::<Vec<_>>();

    let defaults = request
        .options
        .iter()
        .enumerate()
        .filter_map(|(idx, option)| option.selected.then_some(idx))
        .collect::<Vec<_>>();

    let mut multiselect = MultiSelect::new(&request.message, options)
        .with_render_config(theme())
        .with_default(&defaults);

    if let Some(page_size) = request.page_size {
        multiselect = multiselect.with_page_size(page_size);
    }

    let selected = multiselect
        .prompt()
        .map_err(map_inquire_error)?;

    let ids = request
        .options
        .iter()
        .filter(|option| selected.contains(&option.display()))
        .map(|option| option.id.clone())
        .collect::<Vec<_>>();

    Ok(ids)
}

fn map_inquire_error(err: ::inquire::InquireError) -> Error {
    match err {
        ::inquire::InquireError::OperationCanceled
        | ::inquire::InquireError::OperationInterrupted => Error::PromptCancelled,
        other => Error::Prompt(other.to_string()),
    }
}
