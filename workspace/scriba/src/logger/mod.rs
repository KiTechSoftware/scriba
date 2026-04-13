//! Styled stderr logging with verbosity control.
//!
//! Requires the `logger` feature. Access via `Ui::logger()`.

use console::style;

use crate::{ColorMode, Config, Level};

/// Logger for styled, verbosity-aware stderr output.
///
/// Created via `Ui::logger()`. Respects config for colors and verbosity level.
///
/// # Examples
///
/// ```ignore
/// let ui = Ui::new();
/// let logger = ui.logger();
///
/// logger.heading("Building");
/// logger.ok("Build succeeded");
/// logger.info("Deployment starting");
/// ```
pub struct Logger<'a> {
    cfg: &'a Config,
}

impl<'a> Logger<'a> {
    /// Create a new logger with the given config.
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg }
    }

    /// Log a bold heading (shown unless level is `Quiet` or `Silent`).
    pub fn heading(&self, message: &str) {
        if self.cfg.level <= Level::Quiet {
            return;
        }

        if use_color(self.cfg) {
            eprintln!("{}", style(message).bold());
        } else {
            eprintln!("{message}");
        }
    }

    /// Log an info message (shown unless level is `Silent`).
    pub fn info(&self, message: &str) {
        if self.cfg.level == Level::Silent {
            return;
        }

        eprintln!("{} {}", tag(self.cfg, "info"), message);
    }

    /// Log a success/ok message (shown unless level is `Silent`).
    pub fn ok(&self, message: &str) {
        if self.cfg.level == Level::Silent {
            return;
        }

        eprintln!("{} {}", tag(self.cfg, "ok"), message);
    }

    /// Log a warning message (shown unless level is `Quiet` or `Silent`).
    pub fn warn(&self, message: &str) {
        if self.cfg.level <= Level::Quiet {
            return;
        }

        eprintln!("{} {}", tag(self.cfg, "warn"), message);
    }
    
    /// Log a warning key-value pair (shown unless level is `Quiet` or `Silent`).
    pub fn warn_kv(&self, key: &str, value: &str) {
        if self.cfg.level <= Level::Quiet {
            return;
        }

        if use_color(self.cfg) {
            eprintln!("  {}: {}", style(key).dim(), value);
        } else {
            eprintln!("  {key}: {value}");
        }
    }

    /// Log an error message (shown unless level is `Silent`).
    pub fn error(&self, message: &str) {
        if self.cfg.level == Level::Silent {
            return;
        }

        eprintln!("{} {}", tag(self.cfg, "error"), message);
    }

    /// Log an error key-value pair (shown unless level is `Silent`).
    pub fn error_kv(&self, key: &str, value: &str) {
        if self.cfg.level == Level::Silent {
            return;
        }

        if use_color(self.cfg) {
            eprintln!("  {}: {}", style(key).dim(), value);
        } else {
            eprintln!("  {key}: {value}");
        }
    }

    /// Log a detail message (shown only at `Verbose` or higher).
    pub fn detail(&self, message: &str) {
        if self.cfg.level < Level::Verbose {
            return;
        }

        if use_color(self.cfg) {
            eprintln!("{} {}", style("›").dim(), message);
        } else {
            eprintln!("{message}");
        }
    }

    /// Log a debug message (shown only at `Debug` or higher).
    pub fn debug(&self, message: &str) {
        if self.cfg.level < Level::Debug {
            return;
        }

        eprintln!("{} {}", tag(self.cfg, "debug"), message);
    }

    /// Log a trace message (shown only at `Trace` level).
    pub fn trace(&self, message: &str) {
        if self.cfg.level < Level::Trace {
            return;
        }

        eprintln!("{} {}", tag(self.cfg, "trace"), message);
    }

    /// Log a key-value pair detail (shown only at `Verbose` or higher).
    pub fn kv(&self, key: &str, value: &str) {
        if self.cfg.level < Level::Verbose {
            return;
        }

        if use_color(self.cfg) {
            eprintln!("{}: {}", style(key).dim(), value);
        } else {
            eprintln!("{key}: {value}");
        }
    }

    /// Log a list item (shown only at `Verbose` or higher).
    pub fn list_item(&self, message: &str) {
        if self.cfg.level < Level::Verbose {
            return;
        }

        eprintln!("- {message}");
    }

    /// Log a summary with title and multiple lines (shown only at `Verbose` or higher).
    pub fn summary(&self, title: &str, lines: &[String]) {
        if self.cfg.level < Level::Verbose {
            return;
        }

        if use_color(self.cfg) {
            eprintln!("{}", style(title).bold());
        } else {
            eprintln!("{title}");
        }
        for line in lines {
            eprintln!("{line}");
        }
    }
}

fn use_color(cfg: &Config) -> bool {
    let auto = cfg.interactive;
    match cfg.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => auto,
    }
}

fn tag(cfg: &Config, level: &str) -> String {
    if !use_color(cfg) {
        return format!("[{level}]");
    }

    match level {
        "info" => format!("[{}]", style(level).cyan().bold()),
        "ok" => format!("[{}]", style(level).green().bold()),
        "warn" => format!("[{}]", style(level).yellow().bold()),
        "error" => format!("[{}]", style(level).red().bold()),
        "debug" => format!("[{}]", style(level).blue().bold()),
        "trace" => format!("[{}]", style(level).dim().bold()),
        _ => format!("[{}]", style(level).bold()),
    }
}
