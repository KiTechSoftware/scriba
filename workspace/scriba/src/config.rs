#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Plain,
    Text,
    Json,
    Jsonl,
    Markdown,
}

impl Format {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Text => "text",
            Self::Json => "json",
            Self::Jsonl => "jsonl",
            Self::Markdown => "markdown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Silent,
    Quiet,
    Normal,
    Verbose,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub interactive: bool,
    pub auto_yes: bool,
    pub color: ColorMode,
    pub format: Format,
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
