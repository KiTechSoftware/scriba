use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScribaError>;

#[derive(Debug, Error)]
pub enum ScribaError {
    #[error("interactive prompt required")]
    InteractiveRequired,

    #[error("prompt cancelled")]
    PromptCancelled,

    #[error("prompt failed: {0}")]
    Prompt(String),

    #[error("render failed: {0}")]
    Render(String),

    #[error("serialization failed: {0}")]
    Serialization(String),

    #[error("io failed: {0}")]
    Io(String),
}

impl From<std::io::Error> for ScribaError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<serde_json::Error> for ScribaError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}