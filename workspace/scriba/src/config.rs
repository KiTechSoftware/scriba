//! Configuration types for output formatting, verbosity, and color handling.

/// Output format for rendering structured content.
///
/// Determines how `Output` is rendered to text. Can be customized via `Ui::with_format()`.
///
/// # Examples
///
/// ```
/// use scriba::Format;
///
/// let format = Format::Markdown;
/// assert_eq!(format.as_str(), "markdown");
/// assert!(format.is_human());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Plain scalar output (string, number, boolean, or null).
    Plain,
    /// Human-readable text with basic formatting.
    Text,
    /// JSON object representation.
    Json,
    /// Newline-delimited JSON records.
    Jsonl,
    /// Markdown-formatted output.
    Markdown,
}

impl Format {
    /// Convert format to string representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Text => "text",
            Self::Json => "json",
            Self::Jsonl => "jsonl",
            Self::Markdown => "markdown",
        }
    }

    /// Parse format from string (case-insensitive).
    ///
    /// Defaults to `Format::Text` if unrecognized.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "plain" => Self::Plain,
            "text" => Self::Text,
            "json" => Self::Json,
            "jsonl" => Self::Jsonl,
            "markdown" => Self::Markdown,
            // default to text if unrecognized
            _ => Self::Text,
        }
    }

    /// Check if format is structured (machine-readable JSON-based).
    pub fn is_structured(self) -> bool {
        matches!(self, Self::Json | Self::Jsonl)
    }

    /// Check if format is human-readable text.
    pub fn is_human(self) -> bool {
        matches!(self, Self::Plain | Self::Text | Self::Markdown)
    }
}

/// Verbosity level for logging and output.
///
/// Controls which messages are shown. Levels are ordered from least to most verbose:
/// `Silent < Quiet < Normal < Verbose < Debug < Trace`.
///
/// # Examples
///
/// ```
/// use scriba::Level;
///
/// assert!(Level::Verbose > Level::Normal);
/// assert_eq!(Level::from_verbose(2), Level::Debug);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Suppress all output.
    Silent,
    /// Show only critical messages.
    Quiet,
    /// Show standard output (default).
    Normal,
    /// Show additional output details.
    Verbose,
    /// Show debug information.
    Debug,
    /// Show trace information (most verbose).
    Trace,
}

impl Level {
    /// Parse level from string (case-insensitive).
    ///
    /// Defaults to `Level::Normal` if unrecognized.
    pub fn from_str(format: &str) -> Self {
        match format.to_lowercase().as_str() {
            "silent" => Self::Silent,
            "quiet" => Self::Quiet,
            "normal" => Self::Normal,
            "verbose" => Self::Verbose,
            "debug" => Self::Debug,
            "trace" => Self::Trace,
            _ => Self::Normal,
        }
    }

    /// Convert level to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Silent => "silent",
            Self::Quiet => "quiet",
            Self::Normal => "normal",
            Self::Verbose => "verbose",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }

    /// Create level from quiet flag count (e.g., `-qq` = 2).
    ///
    /// - 0 → `Normal`
    /// - 1 → `Quiet`
    /// - 2+ → `Silent`
    pub fn from_quiet(quiet: u8) -> Self {
        match quiet {
            0 => Self::Normal,
            1 => Self::Quiet,
            _ => Self::Silent,
        }
    }

    /// Create level from verbose flag count (e.g., `-vvv` = 3).
    ///
    /// - 0 → `Normal`
    /// - 1 → `Verbose`
    /// - 2 → `Debug`
    /// - 3+ → `Trace`
    pub fn from_verbose(verbose: u8) -> Self {
        match verbose {
            0 => Self::Normal,
            1 => Self::Verbose,
            2 => Self::Debug,
            _ => Self::Trace,
        }
    }

    /// Create level from both verbose and quiet flags, with quiet taking precedence.
    pub fn from_flags(verbose: u8, quiet: u8) -> Self {
        if quiet >= 2 {
            return Self::Silent;
        }
        if quiet == 1 {
            return Self::Quiet;
        }

        match verbose {
            0 => Self::Normal,
            1 => Self::Verbose,
            2 => Self::Debug,
            _ => Self::Trace,
        }
    }
}

/// Color support mode for terminal output.
///
/// # Examples
///
/// ```
/// use scriba::ColorMode;
///
/// let mode = ColorMode::from_str("always");
/// assert_eq!(mode, ColorMode::Always);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Auto-detect based on TTY and environment.
    Auto,
    /// Always output colors (even to non-TTY).
    Always,
    /// Never output colors.
    Never,
}

impl ColorMode {
    /// Parse color mode from string (case-insensitive).
    ///
    /// Defaults to `ColorMode::Auto` if unrecognized.
    pub fn from_str(color: &str) -> Self {
        match color.to_lowercase().as_str() {
            "auto" => Self::Auto,
            "always" => Self::Always,
            "never" => Self::Never,
            _ => Self::Auto,
        }
    }
    
    /// Convert mode to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Never => "never",
        }
    }
}

/// Global configuration for `Ui` behavior and rendering.
///
/// Customize output format, interactivity, colors, and logging verbosity.
///
/// # Examples
///
/// ```
/// use scriba::{Config, Format, Level, ColorMode};
///
/// let config = Config {
///     format: Format::Markdown,
///     interactive: true,
///     level: Level::Verbose,
///     color: ColorMode::Always,
///     auto_yes: false,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Enable interactive prompts (requires a TTY).
    pub interactive: bool,
    /// Auto-confirm prompts (useful for CI/CD).
    pub auto_yes: bool,
    /// Color output mode.
    pub color: ColorMode,
    /// Output format.
    pub format: Format,
    /// Verbosity level for logging.
    pub level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interactive: false,
            auto_yes: false,
            color: ColorMode::Auto,
            format: Format::Text,
            level: Level::Normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Format tests
    #[test]
    fn format_as_str_returns_correct_strings() {
        assert_eq!(Format::Plain.as_str(), "plain");
        assert_eq!(Format::Text.as_str(), "text");
        assert_eq!(Format::Json.as_str(), "json");
        assert_eq!(Format::Jsonl.as_str(), "jsonl");
        assert_eq!(Format::Markdown.as_str(), "markdown");
    }

    #[test]
    fn format_from_str_parses_correctly() {
        assert_eq!(Format::from_str("plain"), Format::Plain);
        assert_eq!(Format::from_str("text"), Format::Text);
        assert_eq!(Format::from_str("json"), Format::Json);
        assert_eq!(Format::from_str("jsonl"), Format::Jsonl);
        assert_eq!(Format::from_str("markdown"), Format::Markdown);
    }

    #[test]
    fn format_from_str_case_insensitive() {
        assert_eq!(Format::from_str("PLAIN"), Format::Plain);
        assert_eq!(Format::from_str("MaRkDoWn"), Format::Markdown);
        assert_eq!(Format::from_str("JSON"), Format::Json);
    }

    #[test]
    fn format_from_str_defaults_to_text_for_unknown() {
        assert_eq!(Format::from_str("unknown"), Format::Text);
        assert_eq!(Format::from_str("invalid"), Format::Text);
        assert_eq!(Format::from_str(""), Format::Text);
    }

    #[test]
    fn format_is_structured_returns_correct_values() {
        assert!(!Format::Plain.is_structured());
        assert!(!Format::Text.is_structured());
        assert!(Format::Json.is_structured());
        assert!(Format::Jsonl.is_structured());
        assert!(!Format::Markdown.is_structured());
    }

    #[test]
    fn format_is_human_returns_correct_values() {
        assert!(Format::Plain.is_human());
        assert!(Format::Text.is_human());
        assert!(!Format::Json.is_human());
        assert!(!Format::Jsonl.is_human());
        assert!(Format::Markdown.is_human());
    }

    // Level tests
    #[test]
    fn level_as_str_returns_correct_strings() {
        assert_eq!(Level::Silent.as_str(), "silent");
        assert_eq!(Level::Quiet.as_str(), "quiet");
        assert_eq!(Level::Normal.as_str(), "normal");
        assert_eq!(Level::Verbose.as_str(), "verbose");
        assert_eq!(Level::Debug.as_str(), "debug");
        assert_eq!(Level::Trace.as_str(), "trace");
    }

    #[test]
    fn level_from_str_parses_correctly() {
        assert_eq!(Level::from_str("silent"), Level::Silent);
        assert_eq!(Level::from_str("quiet"), Level::Quiet);
        assert_eq!(Level::from_str("normal"), Level::Normal);
        assert_eq!(Level::from_str("verbose"), Level::Verbose);
        assert_eq!(Level::from_str("debug"), Level::Debug);
        assert_eq!(Level::from_str("trace"), Level::Trace);
    }

    #[test]
    fn level_from_str_case_insensitive() {
        assert_eq!(Level::from_str("SILENT"), Level::Silent);
        assert_eq!(Level::from_str("VeRbOsE"), Level::Verbose);
        assert_eq!(Level::from_str("DEBUG"), Level::Debug);
    }

    #[test]
    fn level_from_str_defaults_to_normal_for_unknown() {
        assert_eq!(Level::from_str("unknown"), Level::Normal);
        assert_eq!(Level::from_str("invalid"), Level::Normal);
        assert_eq!(Level::from_str(""), Level::Normal);
    }

    #[test]
    fn level_from_quiet_flag() {
        assert_eq!(Level::from_quiet(0), Level::Normal);
        assert_eq!(Level::from_quiet(1), Level::Quiet);
        assert_eq!(Level::from_quiet(2), Level::Silent);
        assert_eq!(Level::from_quiet(3), Level::Silent);
    }

    #[test]
    fn level_from_verbose_flag() {
        assert_eq!(Level::from_verbose(0), Level::Normal);
        assert_eq!(Level::from_verbose(1), Level::Verbose);
        assert_eq!(Level::from_verbose(2), Level::Debug);
        assert_eq!(Level::from_verbose(3), Level::Trace);
        assert_eq!(Level::from_verbose(4), Level::Trace);
    }

    #[test]
    fn level_from_flags_quiet_takes_precedence() {
        assert_eq!(Level::from_flags(0, 0), Level::Normal);
        assert_eq!(Level::from_flags(3, 1), Level::Quiet);
        assert_eq!(Level::from_flags(3, 2), Level::Silent);
    }

    #[test]
    fn level_from_flags_verbose_when_no_quiet() {
        assert_eq!(Level::from_flags(0, 0), Level::Normal);
        assert_eq!(Level::from_flags(1, 0), Level::Verbose);
        assert_eq!(Level::from_flags(2, 0), Level::Debug);
        assert_eq!(Level::from_flags(3, 0), Level::Trace);
    }

    #[test]
    fn level_ordering() {
        assert!(Level::Silent < Level::Quiet);
        assert!(Level::Quiet < Level::Normal);
        assert!(Level::Normal < Level::Verbose);
        assert!(Level::Verbose < Level::Debug);
        assert!(Level::Debug < Level::Trace);
    }

    // ColorMode tests
    #[test]
    fn color_mode_as_str_returns_correct_strings() {
        assert_eq!(ColorMode::Auto.as_str(), "auto");
        assert_eq!(ColorMode::Always.as_str(), "always");
        assert_eq!(ColorMode::Never.as_str(), "never");
    }

    #[test]
    fn color_mode_from_str_parses_correctly() {
        assert_eq!(ColorMode::from_str("auto"), ColorMode::Auto);
        assert_eq!(ColorMode::from_str("always"), ColorMode::Always);
        assert_eq!(ColorMode::from_str("never"), ColorMode::Never);
    }

    #[test]
    fn color_mode_from_str_case_insensitive() {
        assert_eq!(ColorMode::from_str("AUTO"), ColorMode::Auto);
        assert_eq!(ColorMode::from_str("AlWaYs"), ColorMode::Always);
        assert_eq!(ColorMode::from_str("NEVER"), ColorMode::Never);
    }

    #[test]
    fn color_mode_from_str_defaults_to_auto_for_unknown() {
        assert_eq!(ColorMode::from_str("unknown"), ColorMode::Auto);
        assert_eq!(ColorMode::from_str("invalid"), ColorMode::Auto);
        assert_eq!(ColorMode::from_str(""), ColorMode::Auto);
    }

    // Config tests
    #[test]
    fn config_default_values() {
        let config = Config::default();
        assert!(!config.interactive);
        assert!(!config.auto_yes);
        assert_eq!(config.color, ColorMode::Auto);
        assert_eq!(config.format, Format::Text);
        assert_eq!(config.level, Level::Normal);
    }

    #[test]
    fn config_can_be_created_manually() {
        let config = Config {
            interactive: true,
            auto_yes: true,
            color: ColorMode::Always,
            format: Format::Markdown,
            level: Level::Debug,
        };

        assert!(config.interactive);
        assert!(config.auto_yes);
        assert_eq!(config.color, ColorMode::Always);
        assert_eq!(config.format, Format::Markdown);
        assert_eq!(config.level, Level::Debug);
    }

    #[test]
    fn config_is_copy() {
        let config1 = Config::default();
        let config2 = config1;
        assert_eq!(config1.format, config2.format);
    }
}
