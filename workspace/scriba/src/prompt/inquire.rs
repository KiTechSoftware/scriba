use ::inquire::{
    ui::{Color, RenderConfig, StyleSheet, Styled},
    Confirm, MultiSelect, Select, Text,
};

use crate::{Config, Error, Result};
use super::theme::PromptTheme;

/// Parse a color string to inquire color.
///
/// Supports color names: cyan, white, gray, black, red, etc.
/// Maps to available inquire::ui::Color variants.
fn parse_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" | "bright_red" => Color::LightRed,
        "green" | "bright_green" => Color::LightGreen,
        "yellow" | "bright_yellow" => Color::LightYellow,
        "blue" | "bright_blue" => Color::LightBlue,
        "magenta" | "bright_magenta" => Color::LightMagenta,
        "cyan" | "bright_cyan" => Color::LightCyan,
        "white" | "bright_white" => Color::White,
        "gray" | "grey" | "bright_black" => Color::DarkGrey,
        "light_gray" | "light_grey" => Color::White,
        _ => Color::White, // default
    }
}

/// Build a RenderConfig from a PromptTheme.
///
/// Maps theme colors to inquire color codes and applies them to the render configuration.
fn theme_from_prompt_theme(theme: &PromptTheme) -> RenderConfig<'_> {
    RenderConfig::default_colored()
        .with_prompt_prefix(Styled::new("◇").with_fg(parse_color(&theme.question_color)))
        .with_answered_prompt_prefix(Styled::new("✔").with_fg(parse_color(&theme.success_color)))
        .with_canceled_prompt_indicator(Styled::new("✖").with_fg(parse_color(&theme.hint_color)))
        .with_highlighted_option_prefix(Styled::new("›").with_fg(parse_color(&theme.selected_color)))
        .with_scroll_up_prefix(Styled::new("↑").with_fg(parse_color(&theme.hint_color)))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(parse_color(&theme.hint_color)))
        .with_selected_checkbox(Styled::new("●").with_fg(parse_color(&theme.selected_color)))
        .with_unselected_checkbox(Styled::new("○").with_fg(parse_color(&theme.unselected_color)))
        .with_help_message(StyleSheet::new().with_fg(parse_color(&theme.hint_color)))
        .with_answer(StyleSheet::new().with_fg(parse_color(&theme.success_color)))
        .with_option(StyleSheet::new().with_fg(parse_color(&theme.input_color)))
        .with_selected_option(Some(StyleSheet::new().with_fg(parse_color(&theme.selected_color))))
        .with_text_input(StyleSheet::new().with_fg(parse_color(&theme.input_color)))
        .with_default_value(StyleSheet::new().with_fg(parse_color(&theme.hint_color)))
}

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
/// Supports optional pagination via `with_page_size()` for large option lists.
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
/// )
/// .with_page_size(5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectRequest {
    /// Prompt message to display.
    pub message: String,
    /// Available options to choose from.
    pub options: Vec<SelectOption>,
    /// Optional page size for pagination (default: inquire's default of 7).
    pub page_size: Option<usize>,
}

impl SelectRequest {
    /// Create a new select request.
    pub fn new(message: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            message: message.into(),
            options,
            page_size: None,
        }
    }

    /// Set pagination page size for large option lists.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{SelectRequest, SelectOption};
    ///
    /// let request = SelectRequest::new(
    ///     "Pick one",
    ///     vec![SelectOption::new("a", "Option A")],
    /// )
    /// .with_page_size(5);
    /// assert_eq!(request.page_size, Some(5));
    /// ```
    pub fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = Some(size);
        self
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

/// Prompt for text input with theming support.
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
///     &theme,
/// )?;
/// ```
pub fn text(
    cfg: &Config,
    message: &str,
    default: Option<&str>,
    help: Option<&str>,
    theme: &PromptTheme,
) -> Result<String> {
    if !cfg.interactive {
        return Err(Error::InteractiveRequired);
    }

    let mut prompt = Text::new(message).with_render_config(theme_from_prompt_theme(theme));

    if let Some(default) = default {
        prompt = prompt.with_default(default);
    }

    if let Some(help) = help {
        prompt = prompt.with_help_message(help);
    }

    prompt.prompt().map_err(map_inquire_error)
}

/// Prompt for yes/no confirmation with theming support.
///
/// Auto-returns `Ok(true)` if `config.auto_yes` is enabled.
/// Returns the default value if not in interactive mode.
///
/// # Example
///
/// ```ignore
/// if scriba::prompt::confirm(&config, "Continue?", false, &theme)? {
///     println!("Confirmed!");
/// }
/// ```
pub fn confirm(cfg: &Config, message: &str, default: bool, theme: &PromptTheme) -> Result<bool> {
    if cfg.auto_yes {
        return Ok(true);
    }

    if !cfg.interactive {
        return Ok(default);
    }

    Confirm::new(message)
        .with_render_config(theme_from_prompt_theme(theme))
        .with_default(default)
        .prompt()
        .map_err(map_inquire_error)
}

/// Prompt user to select one option from a list with theming support.
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
/// let id = prompt::select(&config, &request, &theme)?;
/// ```
pub fn select(cfg: &Config, request: &SelectRequest, theme: &PromptTheme) -> Result<String> {
    if !cfg.interactive {
        return Err(Error::InteractiveRequired);
    }

    let options = request
        .options
        .iter()
        .map(SelectOption::display)
        .collect::<Vec<_>>();

    let mut prompt = Select::new(&request.message, options).with_render_config(theme_from_prompt_theme(theme));

    if let Some(page_size) = request.page_size {
        prompt = prompt.with_page_size(page_size);
    }

    let selected = prompt.prompt().map_err(map_inquire_error)?;

    let id = request
        .options
        .iter()
        .find(|option| option.display() == selected)
        .map(|option| option.id.clone())
        .ok_or_else(|| Error::Prompt("selected option was not found".to_string()))?;

    Ok(id)
}

/// Prompt user to select multiple options from a list with theming support.
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
/// let ids = prompt::multiselect(&config, &request, &theme)?;
/// ```
pub fn multiselect(cfg: &Config, request: &MultiSelectRequest, theme: &PromptTheme) -> Result<Vec<String>> {
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
        .with_render_config(theme_from_prompt_theme(theme))
        .with_default(&defaults);

    if let Some(page_size) = request.page_size {
        multiselect = multiselect.with_page_size(page_size);
    }

    let selected = multiselect.prompt().map_err(map_inquire_error)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_option_new_creates_with_id_and_label() {
        let option = SelectOption::new("dev", "Development");
        assert_eq!(option.id, "dev");
        assert_eq!(option.label, "Development");
        assert_eq!(option.description, None);
    }

    #[test]
    fn select_option_description_sets_description() {
        let option = SelectOption::new("dev", "Development").description("Local environment");
        assert_eq!(option.description, Some("Local environment".into()));
    }

    #[test]
    fn select_option_display_includes_description() {
        let option = SelectOption::new("dev", "Development").description("Local environment");
        assert_eq!(option.display(), "Development — Local environment");
    }

    #[test]
    fn select_option_display_without_description() {
        let option = SelectOption::new("dev", "Development");
        assert_eq!(option.display(), "Development");
    }

    #[test]
    fn multi_select_option_new_creates_unselected() {
        let option = MultiSelectOption::new("logger", "Logger");
        assert_eq!(option.id, "logger");
        assert_eq!(option.label, "Logger");
        assert!(!option.selected);
        assert_eq!(option.description, None);
    }

    #[test]
    fn multi_select_option_selected_sets_flag() {
        let option = MultiSelectOption::new("logger", "Logger").selected(true);
        assert!(option.selected);
    }

    #[test]
    fn multi_select_option_description_sets_description() {
        let option = MultiSelectOption::new("logger", "Logger").description("Logging system");
        assert_eq!(option.description, Some("Logging system".into()));
    }

    #[test]
    fn multi_select_option_builder_is_fluent() {
        let option = MultiSelectOption::new("logger", "Logger")
            .description("Logging system")
            .selected(true);
        assert!(option.selected);
        assert_eq!(option.description, Some("Logging system".into()));
    }

    #[test]
    fn multi_select_option_display_includes_description() {
        let option = MultiSelectOption::new("logger", "Logger").description("Logging system");
        assert_eq!(option.display(), "Logger — Logging system");
    }

    #[test]
    fn multi_select_option_display_without_description() {
        let option = MultiSelectOption::new("logger", "Logger");
        assert_eq!(option.display(), "Logger");
    }

    #[test]
    fn select_request_new_creates_with_message_and_options() {
        let options = vec![SelectOption::new("a", "Option A")];
        let request = SelectRequest::new("Choose", options.clone());
        assert_eq!(request.message, "Choose");
        assert_eq!(request.options, options);
        assert_eq!(request.page_size, None);
    }

    #[test]
    fn select_request_with_page_size_sets_page_size() {
        let options = vec![SelectOption::new("a", "Option A")];
        let request = SelectRequest::new("Choose", options).with_page_size(5);
        assert_eq!(request.page_size, Some(5));
    }

    #[test]
    fn select_request_page_size_can_be_changed() {
        let options = vec![SelectOption::new("a", "Option A")];
        let request = SelectRequest::new("Choose", options)
            .with_page_size(5)
            .with_page_size(10);
        assert_eq!(request.page_size, Some(10));
    }

    #[test]
    fn select_request_builder_is_fluent() {
        let options = vec![
            SelectOption::new("a", "Option A"),
            SelectOption::new("b", "Option B"),
        ];
        let request = SelectRequest::new("Choose", options).with_page_size(3);
        assert_eq!(request.message, "Choose");
        assert_eq!(request.options.len(), 2);
        assert_eq!(request.page_size, Some(3));
    }

    #[test]
    fn multi_select_request_new_creates_with_no_page_size() {
        let options = vec![MultiSelectOption::new("a", "Option A")];
        let request = MultiSelectRequest::new("Select", options.clone());
        assert_eq!(request.message, "Select");
        assert_eq!(request.options, options);
        assert_eq!(request.page_size, None);
    }

    #[test]
    fn multi_select_request_with_page_size_sets_page_size() {
        let options = vec![MultiSelectOption::new("a", "Option A")];
        let request = MultiSelectRequest::new("Select", options).with_page_size(5);
        assert_eq!(request.page_size, Some(5));
    }

    #[test]
    fn multi_select_request_builder_is_fluent() {
        let options = vec![
            MultiSelectOption::new("a", "Option A"),
            MultiSelectOption::new("b", "Option B"),
        ];
        let request = MultiSelectRequest::new("Select items", options).with_page_size(10);
        assert_eq!(request.message, "Select items");
        assert_eq!(request.options.len(), 2);
        assert_eq!(request.page_size, Some(10));
    }

    #[test]
    fn multi_select_request_page_size_can_be_changed() {
        let options = vec![MultiSelectOption::new("a", "Option A")];
        let request = MultiSelectRequest::new("Select", options)
            .with_page_size(5)
            .with_page_size(10);
        assert_eq!(request.page_size, Some(10));
    }
}
