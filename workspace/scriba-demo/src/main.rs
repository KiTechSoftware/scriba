use scriba::{
    Format, MultiSelectOption, MultiSelectRequest, Output, SelectOption, SelectRequest, StatusKind,
    Styled, Table, TableLayout, TextStyle, Ui,
    envelope::{EnvelopeConfig, EnvelopeLayout, EnvelopeMode, Meta},
    figlet,
    prompt::PromptTheme,
};

fn main() -> scriba::Result<()> {
    let ui = Ui::new().with_format(Format::Plain).interactive(true);

    ui.logger().heading("scriba demo");
    ui.logger().info("starting interactive demo");

    let banner = figlet::render("scriba")?;
    println!("{banner}");

    // Select prompt theme
    let theme_id = ui.select(&SelectRequest::new(
        "Select prompt theme",
        vec![
            SelectOption::new("default", "Default").description("standard terminal colors"),
            SelectOption::new("dark", "Dark").description("for dark terminal backgrounds"),
            SelectOption::new("light", "Light").description("for light terminal backgrounds"),
            SelectOption::new("mono", "Monochrome").description("no colors (accessibility)"),
        ],
    ))?;

    let selected_theme = match theme_id.as_str() {
        "dark" => PromptTheme::dark(),
        "light" => PromptTheme::light(),
        "mono" => PromptTheme::monochrome(),
        _ => PromptTheme::default(),
    };

    // Update UI immediately with selected theme
    let ui = ui
        .with_prompt_theme(selected_theme.clone())
        .interactive(true);

    // Select output format
    let format_id = ui.select(&SelectRequest::new(
        "Select output format",
        vec![
            SelectOption::new("plain", "Plain").description("scalar output only"),
            SelectOption::new("text", "Text").description("human-readable with basic formatting"),
            SelectOption::new("markdown", "Markdown").description("markdown-formatted output"),
            SelectOption::new("json", "JSON").description("pretty-printed JSON object"),
            SelectOption::new("jsonl", "JSONL").description("newline-delimited JSON records"),
        ],
    ))?;

    // Parse format selection and create new UI with that format
    let selected_format = match format_id.as_str() {
        "plain" => Format::Plain,
        "text" => Format::Text,
        "markdown" => Format::Markdown,
        "json" => Format::Json,
        "jsonl" => Format::Jsonl,
        _ => Format::Text,
    };

    // Create UI with selected format and theme
    let ui = Ui::new()
        .with_format(selected_format)
        .with_prompt_theme(selected_theme)
        .interactive(true);

    let project_name = ui.text(
        "Project name?",
        Some("scriba-demo"),
        Some("Name shown in output"),
    )?;

    let confirm_release = ui.confirm("Continue with demo output?", true)?;
    if !confirm_release {
        ui.logger().warn("demo cancelled by user");
        return Ok(());
    }

    let environment = ui.select(&SelectRequest::new(
        "Select environment",
        vec![
            SelectOption::new("dev", "Development").description("fast local iteration"),
            SelectOption::new("stage", "Staging").description("pre-production validation"),
            SelectOption::new("prod", "Production").description("live environment"),
        ],
    ))?;

    let features = ui.multiselect(
        &MultiSelectRequest::new(
            "Select enabled capabilities",
            vec![
                MultiSelectOption::new("prompt", "Prompt")
                    .description("interactive inquire wrapper"),
                MultiSelectOption::new("logger", "Logger").description("styled stderr logging"),
                MultiSelectOption::new("figlet", "Figlet")
                    .description("ascii banner rendering")
                    .selected(true),
                MultiSelectOption::new("markdown", "Markdown")
                    .description("structured markdown output")
                    .selected(true),
                MultiSelectOption::new("table", "Table").description("formatted tables"),
                MultiSelectOption::new("json", "JSON").description("json serialization"),
                MultiSelectOption::new("colors", "Colors").description("terminal colors"),
                MultiSelectOption::new("async", "Async").description("async/await support"),
            ],
        )
        .with_page_size(5),
    )?;

    ui.logger().ok("prompt phase complete");
    ui.logger().kv("project", &project_name);
    ui.logger().kv("environment", &environment);
    ui.logger().kv("selected_features", &features.join(", "));

    // Table layout
    let layout_id = ui.select(&SelectRequest::new(
        "Select table layout",
        vec![
            SelectOption::new("full", "Full").description("bordered with padding (default)"),
            SelectOption::new("compact", "Compact").description("minimal spacing, no borders"),
            SelectOption::new("stacked", "Stacked")
                .description("key-value per row, narrow-terminal friendly"),
        ],
    ))?;

    let selected_layout = match layout_id.as_str() {
        "compact" => TableLayout::Compact,
        "stacked" => TableLayout::Stacked,
        _ => TableLayout::Full,
    };

    // Envelope mode
    let envelope_id = ui.select(&SelectRequest::new(
        "Select envelope mode",
        vec![
            SelectOption::new("none", "None").description("raw output, no wrapping"),
            SelectOption::new("json_flat", "JSON flat")
                .description("wrap in JSON with fields at top level"),
            SelectOption::new("json_nested", "JSON nested")
                .description("wrap in JSON with ok/format nested under meta"),
        ],
    ))?;

    let envelope_cfg = match envelope_id.as_str() {
        "json_flat" => EnvelopeConfig::default()
            .with_mode(EnvelopeMode::Json)
            .with_layout(EnvelopeLayout::Flat),
        "json_nested" => EnvelopeConfig::default()
            .with_mode(EnvelopeMode::Json)
            .with_layout(EnvelopeLayout::Nested),
        _ => EnvelopeConfig::default(),
    };

    let envelope_active = envelope_cfg.mode.is_json();

    let dry_run = if envelope_active {
        ui.confirm("Dry run?", false)?
    } else {
        false
    };

    // Rebuild UI with format + envelope + theme
    let ui = Ui::new()
        .with_format(selected_format)
        .with_envelope(envelope_cfg)
        .with_prompt_theme(match theme_id.as_str() {
            "dark" => PromptTheme::dark(),
            "light" => PromptTheme::light(),
            "mono" => PromptTheme::monochrome(),
            _ => PromptTheme::default(),
        })
        .interactive(true);

    ui.logger().kv("envelope", envelope_id.as_str());
    ui.logger().kv("layout", layout_id.as_str());
    if envelope_active {
        ui.logger().kv("dry_run", &dry_run.to_string());
    }

    let feature_rows = features
        .iter()
        .enumerate()
        .map(|(idx, feature)| {
            vec![
                (idx + 1).to_string(),
                feature.clone(),
                "enabled".to_string(),
            ]
        })
        .collect::<Vec<_>>();

    let feature_table = Table::new(
        vec!["#".into(), "feature".into(), "status".into()],
        feature_rows,
    )
    .with_layout(selected_layout);

    let output = Output::new()
        .title("scriba demo")
        .plain("passed")
        .subtitle("full integration smoke test")
        .data("project", &project_name)
        .data("environment", &environment)
        .data("format", &format_id)
        .data("feature_count", features.len())
        .heading(1, "Summary")
        .styled_paragraph(Styled::new(
            format!("Project {} will run against {}.", project_name, environment),
            TextStyle::Bold,
        ))
        .styled_paragraph(Styled::new(
            format!(
                "Theme: {} · Layout: {} · Format: {}",
                theme_id, layout_id, format_id
            ),
            TextStyle::Dim,
        ))
        .heading(2, "Enabled features")
        .table(Some("Capabilities".into()), feature_table)
        .heading(2, "Raw config")
        .json(serde_json::json!({
            "project": project_name,
            "environment": environment,
            "format": format_id,
            "features": features,
        }))
        .heading(2, "Next steps")
        .list(
            true,
            vec![
                "Verify prompt behavior".into(),
                "Verify logger formatting".into(),
                "Verify markdown rendering".into(),
                "Verify table output".into(),
            ],
        )
        .separator()
        .code(Some("bash".into()), "cargo run -p scriba-demo".to_string())
        .heading(1, "Release summary")
        .status(
            StatusKind::Warning,
            "Tests failed but summary was generated",
        )
        .key_value("project", "scriba")
        .key_value("environment", "ci")
        .definition("Next action", "Review failed checks before publishing");

    if envelope_active {
        let meta = Meta::default()
            .with_dry_run(dry_run)
            .with_command("scriba-demo".into())
            .with_version(env!("CARGO_PKG_VERSION").into());
        ui.print_with_meta(&output, Some(&meta), true)?;
    } else {
        ui.print(&output)?;
    }

    ui.logger().ok("demo finished successfully");
    Ok(())
}
