use std::io::{self, Write};

use crate::{output::render, Config, Format, Output, Result};

pub struct Ui {
    config: Config,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn with_config(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.config.format = format;
        self
    }

    pub fn interactive(mut self, value: bool) -> Self {
        self.config.interactive = value;
        self
    }

    pub fn auto_yes(mut self, value: bool) -> Self {
        self.config.auto_yes = value;
        self
    }

    #[cfg(feature = "logger")]
    pub fn logger(&self) -> crate::logger::Logger<'_> {
        crate::logger::Logger::new(&self.config)
    }

    #[cfg(feature = "prompt")]
    pub fn text(&self, message: &str, default: Option<&str>, help: Option<&str>) -> Result<String> {
        crate::prompt::text(&self.config, message, default, help)
    }

    #[cfg(feature = "prompt")]
    pub fn confirm(&self, message: &str, default: bool) -> Result<bool> {
        crate::prompt::confirm(&self.config, message, default)
    }

    #[cfg(feature = "prompt")]
    pub fn select(&self, request: &crate::prompt::SelectRequest) -> Result<String> {
        crate::prompt::select(&self.config, request)
    }

    #[cfg(feature = "prompt")]
    pub fn multiselect(&self, request: &crate::prompt::MultiSelectRequest) -> Result<Vec<String>> {
        crate::prompt::multiselect(&self.config, request)
    }

    pub fn render(&self, output: &Output) -> Result<String> {
        render::render_output(self.config.format, output)
    }

    pub fn print(&self, output: &Output) -> Result<()> {
        let rendered = self.render(output)?;
        let mut stdout = io::stdout();
        stdout.write_all(rendered.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
