//! Prompt theming — customizable colors and styles for interactive prompts.
//!
//! Demonstrates the four built-in themes: default, dark, light, and monochrome,
//! as well as custom theme creation. Requires `prompt` feature.
//!
//! Run with:
//! ```sh
//! cargo run --example prompt_theming --features prompt
//! ```

use scriba::{Format, Output, Ui};

#[cfg(feature = "prompt")]
use scriba::prompt::PromptTheme;

fn main() -> scriba::Result<()> {
    #[cfg(not(feature = "prompt"))]
    {
        println!("This example requires the 'prompt' feature: run with --features prompt");
        return Ok(());
    }

    #[cfg(feature = "prompt")]
    {
        let ui = Ui::new().with_format(Format::Text);

        // Show theme information
        let output = Output::new()
            .heading(1, "Prompt Themes")
            .paragraph("scriba supports customizable themes for interactive prompts.")
            .line("")
            .heading(2, "Built-in Themes")
            .key_value("default", "Standard terminal colors")
            .key_value("dark", "For dark terminal backgrounds")
            .key_value("light", "For light terminal backgrounds")
            .key_value("monochrome", "No colors (accessibility)")
            .line("")
            .heading(2, "Active Themes in This Run");

        ui.print(&output)?;

        // Show theme details
        let themes = vec![
            ("Default", PromptTheme::default()),
            ("Dark", PromptTheme::dark()),
            ("Light", PromptTheme::light()),
            ("Monochrome", PromptTheme::monochrome()),
        ];

        for (name, theme) in themes {
            let theme_output = Output::new()
                .heading(3, name)
                .key_value("Name", &theme.name)
                .key_value("Question Color", &theme.question_color)
                .key_value("Input Color", &theme.input_color)
                .key_value("Selected Color", &theme.selected_color)
                .key_value("Unselected Color", &theme.unselected_color)
                .key_value("Hint Color", &theme.hint_color)
                .key_value("Success Color", &theme.success_color)
                .key_value("Error Color", &theme.error_color)
                .line("");

            ui.print(&theme_output)?;
        }

        // Show custom theme creation
        let custom_theme = PromptTheme::default()
            .with_question_color("magenta")
            .with_selected_color("cyan")
            .with_input_color("green");

        let custom_output = Output::new()
            .heading(2, "Custom Theme Example")
            .paragraph("Create custom themes by combining builder methods:")
            .line("")
            .heading(3, "Custom Theme")
            .key_value("Question Color", &custom_theme.question_color)
            .key_value("Selected Color", &custom_theme.selected_color)
            .key_value("Input Color", &custom_theme.input_color)
            .line("")
            .heading(2, "Usage in Ui")
            .code(
                Some("rust".into()),
                r#"let ui = Ui::new()
    .with_format(Format::Text)
    .with_prompt_theme(PromptTheme::dark());

// All prompts will now use the dark theme
let name = ui.text("Your name?", None, None)?;
"#,
            );

        ui.print(&custom_output)?;

        // Show accessing theme
        let ui_with_theme = Ui::new().with_prompt_theme(PromptTheme::light());
        let active_theme = ui_with_theme.prompt_theme();

        let access_output = Output::new()
            .heading(2, "Accessing Active Theme")
            .key_value("Current Theme", &active_theme.name)
            .key_value("Question Color", &active_theme.question_color)
            .line("");

        ui.print(&access_output)?;
    }

    Ok(())
}
