use std::io::{self, Write};

use crate::{
    envelope::{self, EnvelopeConfig, EnvelopeFields, EnvelopeLayout, EnvelopeMode, Meta},
    output::render,
    Config, Format, Output, Result,
};

/// Main interface for building CLI output, prompts, and logging.
///
/// `Ui` handles rendering, prompts, and logging based on a `Config`. Use it to:
/// - Render `Output` to different formats (Markdown, JSON, etc.)
/// - Display interactive prompts
/// - Write styled logs
///
/// # Examples
///
/// ```ignore
/// use scriba::{Ui, Format, Output};
///
/// let ui = Ui::new().with_format(Format::Markdown);
/// let output = Output::new().heading(1, "Welcome");
/// ui.print(&output)?;
/// ```
#[derive(Debug, Clone)]
pub struct Ui {
    config: Config,
    envelope: EnvelopeConfig,
    #[cfg(feature = "prompt")]
    prompt_theme: crate::prompt::PromptTheme,
}

impl Ui {
    /// Create a new `Ui` with default configuration.
    ///
    /// Default config:
    /// - Format: `Text`
    /// - Interactive: `false`
    /// - Auto-yes: `false`
    /// - Colors: `Auto`
    /// - Level: `Normal`
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            envelope: EnvelopeConfig::default(),
            #[cfg(feature = "prompt")]
            prompt_theme: crate::prompt::PromptTheme::default(),
        }
    }

    /// Create a new `Ui` with custom configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{Ui, Config, Format};
    ///
    /// let config = Config {
    ///     format: Format::Markdown,
    ///     interactive: false,
    ///     auto_yes: false,
    ///     color: scriba::ColorMode::Auto,
    ///     level: scriba::Level::Normal,
    /// };
    /// let ui = Ui::with_config(config);
    /// ```
    pub fn with_config(config: Config) -> Self {
        Self {
            config,
            envelope: EnvelopeConfig::default(),
            #[cfg(feature = "prompt")]
            prompt_theme: crate::prompt::PromptTheme::default(),
        }
    }

    /// Get reference to the current configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get reference to the current envelope configuration.
    pub fn envelope(&self) -> &EnvelopeConfig {
        &self.envelope
    }

    /// Set the envelope configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{Ui, envelope::{EnvelopeConfig, EnvelopeMode}};
    ///
    /// let ui = Ui::new().with_envelope(
    ///     EnvelopeConfig::default().with_mode(EnvelopeMode::Json)
    /// );
    /// ```
    pub fn with_envelope(mut self, config: EnvelopeConfig) -> Self {
        self.envelope = config;
        self
    }

    /// Set the envelope mode directly.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{Ui, envelope::EnvelopeMode};
    ///
    /// let ui = Ui::new().with_envelope_mode(EnvelopeMode::Json);
    /// ```
    pub fn with_envelope_mode(mut self, mode: EnvelopeMode) -> Self {
        self.envelope.mode = mode;
        self
    }

    /// Set the envelope layout (Flat or Nested).
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{Ui, envelope::{EnvelopeMode, EnvelopeLayout}};
    ///
    /// let ui = Ui::new()
    ///     .with_envelope_mode(EnvelopeMode::Json)
    ///     .with_envelope_layout(EnvelopeLayout::Nested);
    /// ```
    pub fn with_envelope_layout(mut self, layout: EnvelopeLayout) -> Self {
        self.envelope.layout = layout;
        self
    }

    /// Set custom field names for the envelope.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{Ui, envelope::{EnvelopeMode, EnvelopeFields}};
    ///
    /// let ui = Ui::new()
    ///     .with_envelope_mode(EnvelopeMode::Json)
    ///     .with_envelope_fields(EnvelopeFields {
    ///         ok_field: "success".into(),
    ///         format_field: "type".into(),
    ///         content_field: "result".into(),
    ///         meta_field: "context".into(),
    ///     });
    /// ```
    pub fn with_envelope_fields(mut self, fields: EnvelopeFields) -> Self {
        self.envelope.fields = fields;
        self
    }

    /// Set the theme for interactive prompts.
    ///
    /// Requires the `prompt` feature. Customize colors and styles for Text, Confirm,
    /// Select, and MultiSelect prompts.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use scriba::{Ui, prompt::PromptTheme};
    ///
    /// let ui = Ui::new()
    ///     .with_prompt_theme(PromptTheme::dark());
    /// ```
    #[cfg(feature = "prompt")]
    pub fn with_prompt_theme(mut self, theme: crate::prompt::PromptTheme) -> Self {
        self.prompt_theme = theme;
        self
    }

    /// Get reference to the current prompt theme.
    ///
    /// Requires the `prompt` feature.
    #[cfg(feature = "prompt")]
    pub fn prompt_theme(&self) -> &crate::prompt::PromptTheme {
        &self.prompt_theme
    }

    /// Set the output format (e.g., Markdown, JSON).
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{Ui, Format};
    ///
    /// let ui = Ui::new().with_format(Format::Markdown);
    /// ```
    pub fn with_format(mut self, format: Format) -> Self {
        self.config.format = format;
        self
    }

    /// Enable or disable interactive mode for prompts.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::Ui;
    ///
    /// let ui = Ui::new().interactive(true);
    /// ```
    pub fn interactive(mut self, value: bool) -> Self {
        self.config.interactive = value;
        self
    }

    /// Enable auto-confirmation for interactive prompts (useful for CI/CD).
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::Ui;
    ///
    /// let ui = Ui::new().auto_yes(true);
    /// ```
    pub fn auto_yes(mut self, value: bool) -> Self {
        self.config.auto_yes = value;
        self
    }

    /// Get a logger configured with the UI's settings.
    ///
    /// Requires the `logger` feature.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ui = Ui::new();
    /// ui.logger().info("Starting setup");
    /// ```
    #[cfg(feature = "logger")]
    pub fn logger(&self) -> crate::logger::Logger<'_> {
        crate::logger::Logger::new(&self.config)
    }

    /// Prompt for text input.
    ///
    /// Requires the `prompt` feature and `interactive` mode enabled.
    ///
    /// # Arguments
    ///
    /// - `message`: Question to display
    /// - `default`: Default value if user presses Enter
    /// - `help`: Optional help text
    ///
    /// # Errors
    ///
    /// Returns `Error::InteractiveRequired` if not in interactive mode.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ui = Ui::new().interactive(true);
    /// let name = ui.text("Your name?", Some("Anonymous"), None)?;
    /// ```
    #[cfg(feature = "prompt")]
    pub fn text(&self, message: &str, default: Option<&str>, help: Option<&str>) -> Result<String> {
        crate::prompt::text(&self.config, message, default, help, &self.prompt_theme)
    }

    /// Prompt for yes/no confirmation.
    ///
    /// Requires the `prompt` feature. Auto-returns Ok(true) if `auto_yes` is enabled.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ui = Ui::new().interactive(true);
    /// if ui.confirm("Continue?", false)? {
    ///     println!("Confirmed!");
    /// }
    /// ```
    #[cfg(feature = "prompt")]
    pub fn confirm(&self, message: &str, default: bool) -> Result<bool> {
        crate::prompt::confirm(&self.config, message, default, &self.prompt_theme)
    }

    /// Prompt user to select one option from a list.
    ///
    /// Requires the `prompt` feature and `interactive` mode enabled.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use scriba::{Ui, SelectRequest, SelectOption};
    ///
    /// let ui = Ui::new().interactive(true);
    /// let request = SelectRequest::new(
    ///     "Pick one",
    ///     vec![SelectOption::new("a", "Option A")],
    /// );
    /// let selected = ui.select(&request)?;
    /// ```
    #[cfg(feature = "prompt")]
    pub fn select(&self, request: &crate::prompt::SelectRequest) -> Result<String> {
        crate::prompt::select(&self.config, request, &self.prompt_theme)
    }

    /// Prompt user to select multiple options from a list.
    ///
    /// Requires the `prompt` feature and `interactive` mode enabled.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use scriba::{Ui, MultiSelectRequest, MultiSelectOption};
    ///
    /// let ui = Ui::new().interactive(true);
    /// let request = MultiSelectRequest::new(
    ///     "Pick multiple",
    ///     vec![MultiSelectOption::new("a", "Option A")],
    /// );
    /// let selected = ui.multiselect(&request)?;
    /// ```
    #[cfg(feature = "prompt")]
    pub fn multiselect(&self, request: &crate::prompt::MultiSelectRequest) -> Result<Vec<String>> {
        crate::prompt::multiselect(&self.config, request, &self.prompt_theme)
    }

    /// Render `Output` to a formatted string without printing.
    ///
    /// Format is determined by the configured `Format` (Markdown, JSON, etc.).
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::{Ui, Format, Output};
    ///
    /// let ui = Ui::new().with_format(Format::Markdown);
    /// let output = Output::new().paragraph("Hello, world!");
    /// let rendered = ui.render(&output)?;
    /// # Ok::<(), scriba::Error>(())
    /// ```
    pub fn render(&self, output: &Output) -> Result<String> {
        render::render_output(self.config.format, output)
    }

    /// Render `Output` and print to stdout.
    ///
    /// When envelope mode is `Json`, wraps the rendered output in a JSON
    /// object with configurable field names and optional metadata.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let ui = Ui::new();
    /// let output = Output::new().heading(1, "Status");
    /// ui.print(&output)?;
    /// ```
    pub fn print(&self, output: &Output) -> Result<()> {
        self.print_with_meta(output, None, true)
    }

    /// Render `Output` with metadata and print to stdout.
    ///
    /// When envelope mode is `Json`, attaches `meta` to the envelope.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use scriba::envelope::{EnvelopeMode, Meta};
    ///
    /// let ui = Ui::new().with_envelope_mode(EnvelopeMode::Json);
    /// let meta = Meta::default().with_dry_run(true);
    /// let output = Output::new().heading(1, "Deploy");
    /// ui.print_with_meta(&output, Some(&meta), true)?;
    /// ```
    pub fn print_with_meta(
        &self,
        output: &Output,
        meta: Option<&Meta>,
        ok: bool,
    ) -> Result<()> {
        let text = if self.envelope.mode.is_json() {
            let content = render::render_output_value(self.config.format, output)?;
            let wrapped = envelope::wrap(
                &self.envelope,
                self.config.format.as_str(),
                content,
                meta,
                ok,
            );
            serde_json::to_string_pretty(&wrapped)? + "\n"
        } else {
            self.render(output)?
        };
        let mut stdout = io::stdout();
        stdout.write_all(text.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_new_creates_default_config() {
        let ui = Ui::new();
        let config = ui.config();

        assert!(!config.interactive);
        assert!(!config.auto_yes);
        assert_eq!(config.format, Format::Text);
    }

    #[test]
    fn ui_with_config_uses_provided_config() {
        let custom_config = Config {
            interactive: true,
            auto_yes: true,
            format: Format::Markdown,
            color: crate::ColorMode::Always,
            level: crate::Level::Debug,
        };

        let ui = Ui::with_config(custom_config);
        assert_eq!(ui.config(), &custom_config);
    }

    #[test]
    fn ui_with_format_changes_format() {
        let ui = Ui::new()
            .with_format(Format::Markdown)
            .with_format(Format::Json);

        assert_eq!(ui.config().format, Format::Json);
    }

    #[test]
    fn ui_interactive_true() {
        let ui = Ui::new().interactive(true);
        assert!(ui.config().interactive);
    }

    #[test]
    fn ui_interactive_false() {
        let ui = Ui::new().interactive(true).interactive(false);
        assert!(!ui.config().interactive);
    }

    #[test]
    fn ui_auto_yes_true() {
        let ui = Ui::new().auto_yes(true);
        assert!(ui.config().auto_yes);
    }

    #[test]
    fn ui_auto_yes_false() {
        let ui = Ui::new().auto_yes(true).auto_yes(false);
        assert!(!ui.config().auto_yes);
    }

    #[test]
    fn ui_builder_is_fluent() {
        let ui = Ui::new()
            .with_format(Format::Markdown)
            .interactive(true)
            .auto_yes(true);

        assert_eq!(ui.config().format, Format::Markdown);
        assert!(ui.config().interactive);
        assert!(ui.config().auto_yes);
    }

    #[test]
    fn ui_render_plain_format() {
        let ui = Ui::new().with_format(Format::Plain);
        let output = Output::new().plain("test");
        let rendered = ui.render(&output).unwrap();
        assert_eq!(rendered, "test\n");
    }

    #[test]
    fn ui_render_text_format() {
        let ui = Ui::new().with_format(Format::Text);
        let output = Output::new().paragraph("Hello");
        let rendered = ui.render(&output).unwrap();
        assert!(rendered.contains("Hello"));
    }

    #[test]
    fn ui_render_markdown_format() {
        let ui = Ui::new().with_format(Format::Markdown);
        let output = Output::new().heading(1, "Title");
        let rendered = ui.render(&output).unwrap();
        assert!(rendered.contains("# Title"));
    }

    #[test]
    fn ui_render_json_format() {
        let ui = Ui::new().with_format(Format::Json);
        let output = Output::new().data("key", "value");
        let rendered = ui.render(&output).unwrap();
        assert!(rendered.contains("key"));
        assert!(rendered.contains("value"));
    }

    #[test]
    fn ui_render_jsonl_format() {
        let ui = Ui::new().with_format(Format::Jsonl);
        let output = Output::new()
            .jsonl_record(serde_json::json!({"a": 1}))
            .jsonl_record(serde_json::json!({"b": 2}));
        let rendered = ui.render(&output).unwrap();
        assert!(rendered.contains("\"a\""));
        assert!(rendered.contains("\"b\""));
    }

    #[test]
    fn ui_render_title_in_text() {
        let ui = Ui::new().with_format(Format::Text);
        let output = Output::new().title("Status");
        let rendered = ui.render(&output).unwrap();
        assert!(rendered.contains("Status"));
        assert!(rendered.contains("======"));
    }

    #[test]
    fn ui_render_subtitle_in_markdown() {
        let ui = Ui::new().with_format(Format::Markdown);
        let output = Output::new().subtitle("Subtitle text");
        let rendered = ui.render(&output).unwrap();
        assert!(rendered.contains("_Subtitle text_"));
    }

    #[test]
    fn ui_render_empty_output() {
        let ui = Ui::new().with_format(Format::Markdown);
        let output = Output::new();
        let rendered = ui.render(&output).unwrap();
        assert_eq!(rendered.trim(), "");
    }

    #[test]
    fn ui_render_multiple_blocks() {
        let ui = Ui::new().with_format(Format::Markdown);
        let output = Output::new()
            .heading(1, "H1")
            .paragraph("P1")
            .heading(2, "H2")
            .paragraph("P2");
        let rendered = ui.render(&output).unwrap();
        assert!(rendered.contains("# H1"));
        assert!(rendered.contains("P1"));
        assert!(rendered.contains("## H2"));
        assert!(rendered.contains("P2"));
    }

    #[test]
    fn ui_default_is_same_as_new() {
        let ui1 = Ui::new();
        let ui2 = Ui::default();
        assert_eq!(ui1.config(), ui2.config());
    }

    #[test]
    fn ui_copy() {
        // Ui is Clone but not Copy (holds EnvelopeConfig/PromptTheme)
        let ui1 = Ui::new().with_format(Format::Json);
        let ui2 = ui1.clone();
        assert_eq!(ui2.config().format, Format::Json);
    }

    #[cfg(feature = "prompt")]
    #[test]
    fn ui_with_prompt_theme_dark() {
        use crate::prompt::PromptTheme;
        let ui = Ui::new().with_prompt_theme(PromptTheme::dark());
        assert_eq!(ui.prompt_theme().name, "dark");
        assert_eq!(ui.prompt_theme().question_color, "bright_magenta");
    }

    #[cfg(feature = "prompt")]
    #[test]
    fn ui_with_prompt_theme_custom() {
        use crate::prompt::PromptTheme;
        let theme = PromptTheme::default().with_question_color("magenta");
        let ui = Ui::new().with_prompt_theme(theme);
        assert_eq!(ui.prompt_theme().question_color, "magenta");
    }

    #[cfg(feature = "prompt")]
    #[test]
    fn ui_prompt_theme_default_on_new() {
        let ui = Ui::new();
        assert_eq!(ui.prompt_theme().name, "default");
    }
}
