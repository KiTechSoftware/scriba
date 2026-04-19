//! Diff parsing and rendering utilities for structured output.
//!
//! Provides utilities to parse unified diff format patches into structured
//! diff lines and format them for Scriba rendering across multiple output formats.

use serde::{Deserialize, Serialize};

// ANSI color codes for terminal output
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_RESET: &str = "\x1b[0m";

/// Represents a single diff line with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub content: String,
    pub line_number: Option<usize>,
}

/// Type of change in a diff line
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineKind {
    Added,
    Removed,
    Context,
    Header,
}

impl DiffLineKind {
    /// Get the language-specific prefix for rendering
    pub fn prefix(&self) -> &str {
        match self {
            DiffLineKind::Added => "+",
            DiffLineKind::Removed => "-",
            DiffLineKind::Context => " ",
            DiffLineKind::Header => "@@",
        }
    }
}

/// Parses a unified diff format patch into structured diff lines
pub fn parse_diff(patch: &str) -> Vec<DiffLine> {
    patch
        .lines()
        .map(|line| {
            let kind = if line.starts_with("+++") || line.starts_with("---")
                || line.starts_with("@@")
            {
                DiffLineKind::Header
            } else if line.starts_with('+') {
                DiffLineKind::Added
            } else if line.starts_with('-') {
                DiffLineKind::Removed
            } else {
                DiffLineKind::Context
            };

            DiffLine {
                kind,
                content: line.to_string(),
                line_number: None,
            }
        })
        .collect()
}

/// Formats diff content for rendering as a code block
pub fn format_diff_for_scriba(patch: &str) -> String {
    patch.trim_end().to_string()
}

/// Renders colored diff output for terminal display (when colors are enabled)
///
/// Applies ANSI color codes for terminal output:
/// - Green (+) for added lines
/// - Red (-) for removed lines
/// - Cyan (@@) for hunk headers
pub fn render_colored_diff(patch: &str, use_color: bool) -> String {
    if !use_color {
        return patch.to_string();
    }

    let mut output = String::new();
    for line in patch.lines() {
        if line.starts_with('+') && !line.starts_with("+++") {
            output.push_str(COLOR_GREEN);
            output.push_str(line);
            output.push_str(COLOR_RESET);
        } else if line.starts_with('-') && !line.starts_with("---") {
            output.push_str(COLOR_RED);
            output.push_str(line);
            output.push_str(COLOR_RESET);
        } else if line.starts_with("@@") {
            output.push_str(COLOR_CYAN);
            output.push_str(line);
            output.push_str(COLOR_RESET);
        } else {
            output.push_str(line);
        }
        output.push('\n');
    }
    output.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_diff_additions() {
        let patch = "--- a/file.rs\n+++ b/file.rs\n+added line\n context";
        let lines = parse_diff(patch);
        assert!(lines.iter().any(|l| l.kind == DiffLineKind::Header));
        assert!(lines.iter().any(|l| l.kind == DiffLineKind::Added));
        assert!(lines.iter().any(|l| l.kind == DiffLineKind::Context));
    }

    #[test]
    fn test_diff_line_kind_prefix() {
        assert_eq!(DiffLineKind::Added.prefix(), "+");
        assert_eq!(DiffLineKind::Removed.prefix(), "-");
        assert_eq!(DiffLineKind::Context.prefix(), " ");
        assert_eq!(DiffLineKind::Header.prefix(), "@@");
    }

    #[test]
    fn test_format_diff_for_scriba() {
        let patch = "line 1\nline 2\n";
        let formatted = format_diff_for_scriba(patch);
        assert_eq!(formatted, "line 1\nline 2");
    }
}
