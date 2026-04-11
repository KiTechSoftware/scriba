use crate::{Format, Output, StatusKind, Table};

#[test]
fn plain_render_requires_primary_scalar() {
    let output = Output::new().data("message", "ok");
    let err = super::render::render_plain(&output).unwrap_err();

    assert!(err
        .to_string()
        .contains("plain output requires a primary scalar value"));
}

#[test]
fn plain_render_supports_string_scalar() {
    let output = Output::new().plain("hello");
    let rendered = super::render::render_plain(&output).unwrap();

    assert_eq!(rendered, "hello\n");
}

#[test]
fn markdown_heading_renders() {
    let output = Output::new().heading(1, "Title").paragraph("Body");

    let rendered = super::render::render_markdown(&output).unwrap();

    assert!(rendered.contains("# Title"));
    assert!(rendered.contains("Body"));
}

#[test]
fn text_title_renders_underline() {
    let output = Output::new().title("Hello");
    let rendered = super::render::render_text(&output).unwrap();

    assert!(rendered.contains("Hello"));
    assert!(rendered.contains("====="));
}

#[test]
fn markdown_table_renders_header_and_row() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![vec!["alpha".into(), "1".into()]],
    );

    let rendered = super::table::render_markdown_table(&table).unwrap();

    assert!(rendered.contains("| name | value |"));
    assert!(rendered.contains("| alpha | 1 |"));
}

#[test]
fn indexed_table_materializes_index_column() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![
            vec!["alpha".into(), "1".into()],
            vec!["beta".into(), "2".into()],
        ],
    )
    .with_index();

    let materialized = table.materialized();

    assert_eq!(materialized.headers[0], "#");
    assert_eq!(materialized.rows[0][0], "1");
    assert_eq!(materialized.rows[1][0], "2");
}

#[test]
fn json_output_is_object() {
    let output = Output::new().data("message", "ok").data("count", 2);

    let rendered = super::render::render_output_value(Format::Json, &output).unwrap();

    assert!(rendered.is_object());
}

#[test]
fn markdown_output_value_is_string() {
    let output = Output::new().data("message", "ok").data("count", 2);

    let rendered = super::render::render_output_value(Format::Markdown, &output).unwrap();

    assert!(rendered.is_string());
}

#[test]
fn jsonl_render_joins_records_with_newlines() {
    let output = Output::new()
        .jsonl_record(serde_json::json!({ "message": "a", "count": 1 }))
        .jsonl_record(serde_json::json!({ "message": "b", "count": 2 }));

    let rendered = super::render::render_jsonl(&output).unwrap();

    assert!(rendered.contains('\n'));
    assert_eq!(rendered.lines().count(), 2);
}

#[test]
fn jsonl_render_falls_back_to_blocks() {
    let output = Output::new().heading(1, "Hello").paragraph("World");

    let rendered = super::render::render_jsonl(&output).unwrap();

    assert!(rendered.lines().count() >= 2);
    assert!(rendered.contains("\"block\""));
}

#[test]
fn table_render_value_returns_string_for_markdown() {
    let table = Table::new(vec!["name".into()], vec![vec!["alpha".into()]]);

    let value = super::table::render_table_value(Format::Markdown, &table).unwrap();

    assert!(value.is_string());
}

#[test]
fn table_render_value_returns_json_for_json_format() {
    let table = Table::new(vec!["name".into()], vec![vec!["alpha".into()]]);

    let value = super::table::render_table_value(Format::Json, &table).unwrap();

    assert!(value.is_object());
}

#[test]
fn key_value_entries_group_into_single_block() {
    let output = Output::new()
        .key_value("project", "scriba")
        .key_value("env", "prod");

    assert_eq!(output.blocks.len(), 1);

    match &output.blocks[0] {
        super::content::Block::KeyValue { entries } => {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].key, "project");
            assert_eq!(entries[1].key, "env");
        }
        _ => panic!("expected key_value block"),
    }
}

#[test]
fn definition_entries_group_into_single_block() {
    let output = Output::new()
        .definition("Project", "scriba")
        .definition("Environment", "production");

    assert_eq!(output.blocks.len(), 1);

    match &output.blocks[0] {
        super::content::Block::DefinitionList { entries } => {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].term, "Project");
            assert_eq!(entries[1].term, "Environment");
        }
        _ => panic!("expected definition_list block"),
    }
}

#[test]
fn markdown_key_value_renders_as_bullets() {
    let output = Output::new()
        .key_value("project", "scriba")
        .key_value("env", "prod");

    let rendered = super::render::render_markdown(&output).unwrap();

    assert!(rendered.contains("- **project**: scriba"));
    assert!(rendered.contains("- **env**: prod"));
}

#[test]
fn text_definition_list_renders_term_and_description() {
    let output = Output::new()
        .definition("Project", "scriba")
        .definition("Environment", "production");

    let rendered = super::render::render_text(&output).unwrap();

    assert!(rendered.contains("Project:"));
    assert!(rendered.contains("  scriba"));
    assert!(rendered.contains("Environment:"));
}

#[test]
fn status_renders_in_markdown() {
    let output = Output::new().status(
        StatusKind::Warning,
        "Tests failed but summary was generated",
    );

    let rendered = super::render::render_markdown(&output).unwrap();

    assert!(rendered.contains("- **warning**: Tests failed but summary was generated"));
}

#[test]
fn status_renders_in_text() {
    let output = Output::new().status(StatusKind::Success, "Deployment complete");

    let rendered = super::render::render_text(&output).unwrap();

    assert!(rendered.contains("[success] Deployment complete"));
}
