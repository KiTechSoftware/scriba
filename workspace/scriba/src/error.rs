//! Error types and result type for scriba operations.

use thiserror::Error;

/// Result type for scriba operations.
///
/// # Example
///
/// ```
/// use scriba::Result;
///
/// fn setup_output() -> Result<String> {
///     Ok("Hello".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, ScribaError>;

/// Error types returned by scriba operations.
///
/// # Examples
///
/// ```
/// use scriba::Error;
///
/// let err = Error::InteractiveRequired;
/// assert_eq!(err.to_string(), "interactive prompt required");
/// ```
#[derive(Debug, Error)]
pub enum ScribaError {
    /// Interactive mode was required but not enabled in config.
    #[error("interactive prompt required")]
    InteractiveRequired,

    /// User cancelled an interactive prompt.
    #[error("prompt cancelled")]
    PromptCancelled,

    /// Prompt operation failed.
    #[error("prompt failed: {0}")]
    Prompt(String),

    /// Output rendering failed.
    #[error("render failed: {0}")]
    Render(String),

    /// JSON serialization failed.
    #[error("serialization failed: {0}")]
    Serialization(String),

    /// I/O operation failed.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn error_interactive_required_message() {
        let err = ScribaError::InteractiveRequired;
        assert_eq!(err.to_string(), "interactive prompt required");
    }

    #[test]
    fn error_prompt_cancelled_message() {
        let err = ScribaError::PromptCancelled;
        assert_eq!(err.to_string(), "prompt cancelled");
    }

    #[test]
    fn error_prompt_message() {
        let err = ScribaError::Prompt("invalid input".to_string());
        assert!(err.to_string().contains("invalid input"));
    }

    #[test]
    fn error_render_message() {
        let err = ScribaError::Render("failed to render".to_string());
        assert!(err.to_string().contains("failed to render"));
    }

    #[test]
    fn error_serialization_message() {
        let err = ScribaError::Serialization("bad json".to_string());
        assert!(err.to_string().contains("bad json"));
    }

    #[test]
    fn error_io_message() {
        let err = ScribaError::Io("file not found".to_string());
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn io_error_converts_to_scriba_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test error");
        let scriba_err: ScribaError = io_err.into();
        
        match scriba_err {
            ScribaError::Io(msg) => assert!(msg.contains("test error")),
            _ => panic!("expected Io error"),
        }
    }

    #[test]
    fn json_error_converts_to_scriba_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let scriba_err: ScribaError = json_err.into();
        
        match scriba_err {
            ScribaError::Serialization(_) => {}, // Expected
            _ => panic!("expected Serialization error"),
        }
    }

    #[test]
    fn result_type_is_correct() {
        fn returns_result() -> Result<String> {
            Ok("success".to_string())
        }

        let result = returns_result();
        assert!(result.is_ok());
    }

    #[test]
    fn result_error() {
        fn returns_error() -> Result<String> {
            Err(ScribaError::Render("test".to_string()))
        }

        let result = returns_error();
        assert!(result.is_err());
    }
}
