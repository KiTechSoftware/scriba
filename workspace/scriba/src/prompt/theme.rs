//! Prompt theming — customizable colors and styles for interactive prompts.
//!
//! Control presentation of text, select, multiselect, and confirm prompts with
//! customizable themes. Built-in themes: "default", "dark", "light".

use serde::{Deserialize, Serialize};

/// Theme for interactive prompts.
///
/// Customizes colors and styles for prompt components. Applies to Text, Confirm,
/// Select, and MultiSelect prompts.
///
/// # Examples
///
/// ```
/// use scriba::prompt::PromptTheme;
///
/// let dark_theme = PromptTheme::dark();
/// let light_theme = PromptTheme::light();
/// let custom = PromptTheme::default();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptTheme {
    /// Theme name (e.g., "default", "dark", "light").
    pub name: String,
    /// Color for prompt questions (hex or ANSI color name).
    pub question_color: String,
    /// Color for input text (hex or ANSI color name).
    pub input_color: String,
    /// Color for selected/highlighted items (hex or ANSI color name).
    pub selected_color: String,
    /// Color for unselected items.
    pub unselected_color: String,
    /// Color for help text / hints.
    pub hint_color: String,
    /// Color for success/confirmation messages.
    pub success_color: String,
    /// Color for error messages.
    pub error_color: String,
}

impl Default for PromptTheme {
    fn default() -> Self {
        Self::default_theme()
    }
}

impl PromptTheme {
    /// Create the default theme (standard terminal colors).
    pub fn default_theme() -> Self {
        Self {
            name: "default".into(),
            question_color: "bright_cyan".into(),
            input_color: "white".into(),
            selected_color: "bright_green".into(),
            unselected_color: "gray".into(),
            hint_color: "gray".into(),
            success_color: "bright_green".into(),
            error_color: "bright_red".into(),
        }
    }

    /// Create a dark theme (optimized for dark terminals).
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            question_color: "bright_magenta".into(),
            input_color: "bright_white".into(),
            selected_color: "bright_yellow".into(),
            unselected_color: "bright_black".into(),
            hint_color: "bright_black".into(),
            success_color: "bright_green".into(),
            error_color: "bright_red".into(),
        }
    }

    /// Create a light theme (optimized for light terminals).
    pub fn light() -> Self {
        Self {
            name: "light".into(),
            question_color: "bright_blue".into(),
            input_color: "white".into(),
            selected_color: "bright_cyan".into(),
            unselected_color: "gray".into(),
            hint_color: "gray".into(),
            success_color: "bright_blue".into(),
            error_color: "bright_red".into(),
        }
    }

    /// Create a monochrome theme (no colors).
    pub fn monochrome() -> Self {
        Self {
            name: "monochrome".into(),
            question_color: "white".into(),
            input_color: "white".into(),
            selected_color: "white".into(),
            unselected_color: "white".into(),
            hint_color: "white".into(),
            success_color: "white".into(),
            error_color: "white".into(),
        }
    }

    /// Set the question/prompt text color.
    pub fn with_question_color(mut self, color: impl Into<String>) -> Self {
        self.question_color = color.into();
        self
    }

    /// Set the input text color.
    pub fn with_input_color(mut self, color: impl Into<String>) -> Self {
        self.input_color = color.into();
        self
    }

    /// Set the selected/highlighted item color.
    pub fn with_selected_color(mut self, color: impl Into<String>) -> Self {
        self.selected_color = color.into();
        self
    }

    /// Set the unselected item color.
    pub fn with_unselected_color(mut self, color: impl Into<String>) -> Self {
        self.unselected_color = color.into();
        self
    }

    /// Set the hint text color.
    pub fn with_hint_color(mut self, color: impl Into<String>) -> Self {
        self.hint_color = color.into();
        self
    }

    /// Set the success message color.
    pub fn with_success_color(mut self, color: impl Into<String>) -> Self {
        self.success_color = color.into();
        self
    }

    /// Set the error message color.
    pub fn with_error_color(mut self, color: impl Into<String>) -> Self {
        self.error_color = color.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_theme_default() {
        let theme = PromptTheme::default();
        assert_eq!(theme.name, "default");
        assert_eq!(theme.question_color, "bright_cyan");
    }

    #[test]
    fn prompt_theme_dark() {
        let theme = PromptTheme::dark();
        assert_eq!(theme.name, "dark");
        assert_eq!(theme.question_color, "bright_magenta");
    }

    #[test]
    fn prompt_theme_light() {
        let theme = PromptTheme::light();
        assert_eq!(theme.name, "light");
        assert_eq!(theme.question_color, "bright_blue");
    }

    #[test]
    fn prompt_theme_builders() {
        let theme = PromptTheme::default()
            .with_question_color("red")
            .with_selected_color("yellow");

        assert_eq!(theme.question_color, "red");
        assert_eq!(theme.selected_color, "yellow");
    }

    #[test]
    fn prompt_theme_all_builders() {
        let theme = PromptTheme::default()
            .with_question_color("q")
            .with_input_color("i")
            .with_selected_color("s")
            .with_unselected_color("u")
            .with_hint_color("h")
            .with_success_color("ok")
            .with_error_color("err");

        assert_eq!(theme.question_color, "q");
        assert_eq!(theme.input_color, "i");
        assert_eq!(theme.selected_color, "s");
        assert_eq!(theme.unselected_color, "u");
        assert_eq!(theme.hint_color, "h");
        assert_eq!(theme.success_color, "ok");
        assert_eq!(theme.error_color, "err");
    }

    #[test]
    fn prompt_theme_monochrome() {
        let theme = PromptTheme::monochrome();
        assert_eq!(theme.name, "monochrome");
        assert_eq!(theme.question_color, "white");
        assert_eq!(theme.selected_color, "white");
    }

    #[test]
    fn prompt_theme_serde_round_trip() {
        let original = PromptTheme::dark();
        let json = serde_json::to_string(&original).unwrap();
        let restored: PromptTheme = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn prompt_theme_clone_is_independent() {
        let a = PromptTheme::default();
        let mut b = a.clone();
        b.question_color = "purple".into();
        assert_eq!(a.question_color, "bright_cyan");
        assert_eq!(b.question_color, "purple");
    }
}
