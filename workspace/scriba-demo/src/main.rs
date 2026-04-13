use scriba::{
    Format, MultiSelectOption, MultiSelectRequest, Output, SelectOption, SelectRequest, StatusKind,
    Table, Ui, figlet,
};

fn main() -> scriba::Result<()> {
    let ui = Ui::new()
        .with_format(Format::Plain)
        .interactive(true);

    ui.logger().heading("scriba demo");
    ui.logger().info("starting interactive demo");

    let banner = figlet::render("scriba")?;
    println!("{banner}");

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

    let features = ui.multiselect(&MultiSelectRequest::new(
        "Select enabled capabilities",
        vec![
            MultiSelectOption::new("prompt", "Prompt").description("interactive inquire wrapper"),
            MultiSelectOption::new("logger", "Logger").description("styled stderr logging"),
            MultiSelectOption::new("figlet", "Figlet")
                .description("ascii banner rendering")
                .selected(true),
            MultiSelectOption::new("markdown", "Markdown")
                .description("structured markdown output")
                .selected(true),
        ],
    ))?;

    ui.logger().ok("prompt phase complete");
    ui.logger().kv("project", &project_name);
    ui.logger().kv("environment", &environment);
    ui.logger().kv("selected_features", &features.join(", "));

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
    );

    let output = Output::new()
        .title("scriba demo")
        .plain("passed")
        .subtitle("full integration smoke test")
        .data("project", &project_name)
        .data("environment", &environment)
        .data("feature_count", features.len())
        .heading(1, "Summary")
        .paragraph(format!(
            "Project **{}** will run against **{}**.",
            project_name, environment
        ))
        .heading(2, "Enabled features")
        .table(Some("Capabilities".into()), feature_table)
        .heading(2, "Raw config")
        .json(serde_json::json!({
            "project": project_name,
            "environment": environment,
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

    ui.print(&output)?;

    ui.logger().ok("demo finished successfully");
    Ok(())
}
