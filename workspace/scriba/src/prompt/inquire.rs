use ::inquire::{
    ui::{Color, RenderConfig, StyleSheet, Styled},
    Confirm, MultiSelect, Select, Text,
};

use crate::{Config, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
}

impl SelectOption {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
        }
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSelectOption {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub selected: bool,
}

impl MultiSelectOption {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            selected: false,
        }
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectRequest {
    pub message: String,
    pub options: Vec<SelectOption>,
}

impl SelectRequest {
    pub fn new(message: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            message: message.into(),
            options,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSelectRequest {
    pub message: String,
    pub options: Vec<MultiSelectOption>,
}

impl MultiSelectRequest {
    pub fn new(message: impl Into<String>, options: Vec<MultiSelectOption>) -> Self {
        Self {
            message: message.into(),
            options,
        }
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

    let selected = MultiSelect::new(&request.message, options)
        .with_render_config(theme())
        .with_default(&defaults)
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
