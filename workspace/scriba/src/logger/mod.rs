use console::style;

use crate::{ColorMode, Config, Level, Result};

pub struct Logger<'a> {
    cfg: &'a Config,
}

impl<'a> Logger<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg }
    }

    pub fn heading(&self, message: &str) -> Result<()> {
        if self.cfg.level <= Level::Quiet {
            return Ok(());
        }

        if use_color(self.cfg) {
            eprintln!("{}", style(message).bold());
        } else {
            eprintln!("{message}");
        }

        Ok(())
    }

    pub fn info(&self, message: &str) -> Result<()> {
        if self.cfg.level == Level::Silent {
            return Ok(());
        }

        eprintln!("{} {}", tag(self.cfg, "info"), message);
        Ok(())
    }

    pub fn ok(&self, message: &str) -> Result<()> {
        if self.cfg.level == Level::Silent {
            return Ok(());
        }

        eprintln!("{} {}", tag(self.cfg, "ok"), message);
        Ok(())
    }

    pub fn warn(&self, message: &str) -> Result<()> {
        if self.cfg.level <= Level::Quiet {
            return Ok(());
        }

        eprintln!("{} {}", tag(self.cfg, "warn"), message);
        Ok(())
    }

    pub fn error(&self, message: &str) -> Result<()> {
        if self.cfg.level == Level::Silent {
            return Ok(());
        }

        eprintln!("{} {}", tag(self.cfg, "error"), message);
        Ok(())
    }

    pub fn detail(&self, message: &str) -> Result<()> {
        if self.cfg.level < Level::Verbose {
            return Ok(());
        }

        if use_color(self.cfg) {
            eprintln!("{} {}", style("›").dim(), message);
        } else {
            eprintln!("{message}");
        }

        Ok(())
    }

    pub fn debug(&self, message: &str) -> Result<()> {
        if self.cfg.level < Level::Debug {
            return Ok(());
        }

        eprintln!("{} {}", tag(self.cfg, "debug"), message);
        Ok(())
    }

    pub fn trace(&self, message: &str) -> Result<()> {
        if self.cfg.level < Level::Trace {
            return Ok(());
        }

        eprintln!("{} {}", tag(self.cfg, "trace"), message);
        Ok(())
    }

    pub fn kv(&self, key: &str, value: &str) -> Result<()> {
        if self.cfg.level < Level::Verbose {
            return Ok(());
        }

        if use_color(self.cfg) {
            eprintln!("{}: {}", style(key).dim(), value);
        } else {
            eprintln!("{key}: {value}");
        }

        Ok(())
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
