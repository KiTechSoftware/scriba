use crate::{Format, Output, StatusKind, Table, TableLayout};

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
fn table_full_layout_renders_with_borders() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![vec!["alpha".into(), "1".into()]],
    )
    .with_layout_full();

    let rendered = super::table::render_text_table(&table).unwrap();

    // Full layout should have borders (│, ┌, └, etc)
    assert!(rendered.contains('│'));
}

#[test]
fn table_compact_layout_minimal_spacing() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![vec!["alpha".into(), "1".into()]],
    )
    .with_layout_compact();

    let rendered = super::table::render_text_table(&table).unwrap();

    // Compact layout should have headers and row with minimal spacing
    assert!(rendered.contains("name  value"));
    assert!(rendered.contains("alpha  1"));
}

#[test]
fn table_compact_layout_empty_shows_headers() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![],
    )
    .with_layout_compact();

    let rendered = super::table::render_text_table(&table).unwrap();

    assert_eq!(rendered, "name  value");
}

#[test]
fn table_stacked_layout_key_value_format() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![
            vec!["alpha".into(), "1".into()],
            vec!["beta".into(), "2".into()],
        ],
    )
    .with_layout_stacked();

    let rendered = super::table::render_text_table(&table).unwrap();

    // Stacked layout should have key: value format
    assert!(rendered.contains("name: alpha"));
    assert!(rendered.contains("value: 1"));
    assert!(rendered.contains("---"));
    assert!(rendered.contains("name: beta"));
    assert!(rendered.contains("value: 2"));
}

#[test]
fn table_stacked_layout_empty_returns_empty_string() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![],
    )
    .with_layout_stacked();

    let rendered = super::table::render_text_table(&table).unwrap();

    assert_eq!(rendered, "");
}

#[test]
fn table_with_index_and_compact_layout() {
    let table = Table::new(
        vec!["name".into(), "value".into()],
        vec![
            vec!["alpha".into(), "1".into()],
            vec!["beta".into(), "2".into()],
        ],
    )
    .with_index()
    .with_layout_compact();

    let rendered = super::table::render_text_table(&table).unwrap();

    // Should have index column and compact spacing
    assert!(rendered.contains("#  name  value"));
    assert!(rendered.contains("1  alpha  1"));
}

#[test]
fn table_default_layout_is_full() {
    let table = Table::new(
        vec!["name".into()],
        vec![vec!["test".into()]],
    );

    assert_eq!(table.layout, TableLayout::Full);
}

#[test]
fn table_layout_builders() {
    let table_full = Table::new(vec![], vec![]).with_layout_full();
    let table_compact = Table::new(vec![], vec![]).with_layout_compact();
    let table_stacked = Table::new(vec![], vec![]).with_layout_stacked();

    assert_eq!(table_full.layout, TableLayout::Full);
    assert_eq!(table_compact.layout, TableLayout::Compact);
    assert_eq!(table_stacked.layout, TableLayout::Stacked);
}

#[test]
fn styled_text_renders_ansi_in_text_format() {
    let output = Output::new()
        .styled_paragraph(crate::Styled::new("Important", crate::TextStyle::Bold));

    let rendered = super::render::render_text(&output).unwrap();

    // Should contain ANSI bold codes
    assert!(rendered.contains("\x1b[1m"));
    assert!(rendered.contains("Important"));
    assert!(rendered.contains("\x1b[0m"));
}

#[test]
fn styled_text_renders_markdown_syntax() {
    let output = Output::new()
        .styled_paragraph(crate::Styled::new("Notice", crate::TextStyle::Italic));

    let rendered = super::render::render_markdown(&output).unwrap();

    assert!(rendered.contains("*Notice*"));
}

#[test]
fn styled_text_bold_italic_renders_correctly() {
    let output = Output::new()
        .styled_paragraph(crate::Styled::new("Critical", crate::TextStyle::BoldItalic));

    let text = super::render::render_text(&output).unwrap();
    let md = super::render::render_markdown(&output).unwrap();

    assert!(text.contains("\x1b[1;3m"));
    assert!(md.contains("***Critical***"));
}

#[test]
fn styled_text_strikethrough_and_underline() {
    let out_strike = Output::new()
        .styled_paragraph(crate::Styled::new("Removed", crate::TextStyle::Strikethrough));
    let out_under = Output::new()
        .styled_paragraph(crate::Styled::new("Linked", crate::TextStyle::Underline));

    let md_strike = super::render::render_markdown(&out_strike).unwrap();
    let md_under = super::render::render_markdown(&out_under).unwrap();

    assert!(md_strike.contains("~~Removed~~"));
    assert!(md_under.contains("<u>Linked</u>"));
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
    let output = Output::new().status(StatusKind::Ok, "Deployment complete");

    let rendered = super::render::render_text(&output).unwrap();

    assert!(rendered.contains("[success] Deployment complete"));
}

// Additional comprehensive tests

#[test]
fn output_new_creates_empty() {
    let output = Output::new();
    assert!(output.blocks.is_empty());
    assert!(output.data.is_empty());
    assert!(output.title.is_none());
    assert!(output.subtitle.is_none());
}

#[test]
fn output_title_and_subtitle_render_together() {
    let output = Output::new().title("Main Title").subtitle("Subtitle Text");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("# Main Title"));
    assert!(rendered.contains("_Subtitle Text_"));
}

#[test]
fn plain_render_with_number() {
    let output = Output::new().plain(42);
    let rendered = super::render::render_plain(&output).unwrap();
    assert_eq!(rendered, "42\n");
}

#[test]
fn plain_render_with_bool() {
    let output = Output::new().plain(true);
    let rendered = super::render::render_plain(&output).unwrap();
    assert_eq!(rendered, "true\n");
}

#[test]
fn plain_render_with_null() {
    let output = Output::new().plain(serde_json::Value::Null);
    let rendered = super::render::render_plain(&output).unwrap();
    assert_eq!(rendered, "null\n");
}

#[test]
fn plain_render_with_array_fails() {
    let output = Output::new().plain(vec![1, 2, 3]);
    let err = super::render::render_plain(&output).unwrap_err();
    assert!(err.to_string().contains("plain output must be"));
}

#[test]
fn plain_render_with_object_fails() {
    let output = Output::new().data("test", "value");
    let err = super::render::render_plain(&output).unwrap_err();
    assert!(err.to_string().contains("plain output requires"));
}

#[test]
fn text_heading_with_different_levels() {
    for level in 1..=6 {
        let output = Output::new().heading(level, "Title");
        let rendered = super::render::render_text(&output).unwrap();
        assert!(rendered.contains("Title"));
    }
}

#[test]
fn markdown_heading_with_different_levels() {
    let output = Output::new()
        .heading(1, "H1")
        .heading(2, "H2")
        .heading(3, "H3");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("# H1"));
    assert!(rendered.contains("## H2"));
    assert!(rendered.contains("### H3"));
}

#[test]
fn list_unordered() {
    let output = Output::new().list(
        false,
        vec!["Item 1".into(), "Item 2".into(), "Item 3".into()],
    );

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("- Item 1"));
    assert!(rendered.contains("- Item 2"));
    assert!(rendered.contains("- Item 3"));
}

#[test]
fn list_ordered() {
    let output = Output::new().list(true, vec!["First".into(), "Second".into(), "Third".into()]);

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("1. First"));
    assert!(rendered.contains("2. Second"));
    assert!(rendered.contains("3. Third"));
}

#[test]
fn list_empty() {
    let output = Output::new().list(false, vec![]);

    let rendered = super::render::render_markdown(&output).unwrap();
    // Empty list should still render cleanly
    assert!(rendered.len() < 100);
}

#[test]
fn code_without_language() {
    let output = Output::new().code(None, "let x = 42;");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("let x = 42;"));
}

#[test]
fn code_with_language() {
    let output = Output::new().code(Some("rust".into()), "fn main() {}");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("rust"));
    assert!(rendered.contains("fn main() {}"));
}

#[test]
fn code_multiline() {
    let code = "fn main() {\n  println!(\"Hello\");\n}".to_string();
    let output = Output::new().code(Some("rust".into()), code.clone());

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("main"));
    assert!(rendered.contains("Hello"));
}

#[test]
fn separator_renders() {
    let output = Output::new()
        .paragraph("Before")
        .separator()
        .paragraph("After");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("Before"));
    assert!(rendered.contains("After"));
}

#[test]
fn line_block_renders() {
    let output = Output::new().line("Line 1").line("Line 2").line("Line 3");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("Line 1"));
    assert!(rendered.contains("Line 2"));
}

#[test]
fn json_block_renders() {
    let output = Output::new().json(serde_json::json!({ "key": "value", "count": 42 }));

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("key"));
    assert!(rendered.contains("value"));
}

#[test]
fn table_empty_rows() {
    let table = Table::new(vec!["Column 1".into(), "Column 2".into()], vec![]);

    let rendered = super::table::render_markdown_table(&table).unwrap();
    assert!(rendered.contains("Column 1"));
    assert!(rendered.contains("Column 2"));
    // Should not crash with empty rows
    assert!(!rendered.is_empty());
}

#[test]
fn table_single_row() {
    let table = Table::new(
        vec!["Name".into(), "Age".into()],
        vec![vec!["Alice".into(), "30".into()]],
    );

    let rendered = super::table::render_markdown_table(&table).unwrap();
    assert!(rendered.contains("Alice"));
    assert!(rendered.contains("30"));
}

#[test]
fn table_many_rows() {
    let mut rows = vec![];
    for i in 0..10 {
        rows.push(vec![i.to_string(), format!("Row {}", i)]);
    }
    let table = Table::new(vec!["Index".into(), "Name".into()], rows);

    let rendered = super::table::render_markdown_table(&table).unwrap();
    assert!(rendered.contains("0"));
    assert!(rendered.contains("9"));
}

#[test]
fn table_with_index_and_custom_header() {
    let table = Table::new(
        vec!["Name".into(), "Value".into()],
        vec![vec!["Option".into(), "1".into()]],
    )
    .with_index_header("No.");

    let materialized = table.materialized();
    assert_eq!(materialized.headers[0], "No.");
}

#[test]
fn table_without_index() {
    let table = Table::new(
        vec!["Name".into(), "Value".into()],
        vec![vec!["A".into(), "1".into()]],
    );

    let materialized = table.materialized();
    assert_eq!(materialized.headers.len(), 2);
    assert_eq!(materialized.rows[0].len(), 2);
}

#[test]
fn table_json_value() {
    let table = Table::new(vec!["Name".into()], vec![vec!["Alice".into()]]);

    let value = table.to_json_value();
    assert!(value.is_object());
}

#[test]
fn status_all_kinds_render() {
    let output = Output::new()
        .status(StatusKind::Info, "Info message")
        .status(StatusKind::Ok, "Success message")
        .status(StatusKind::Warning, "Warning message")
        .status(StatusKind::Error, "Error message");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("info"));
    assert!(rendered.contains("success"));
    assert!(rendered.contains("warning"));
    assert!(rendered.contains("error"));
}

#[test]
fn key_value_single() {
    let output = Output::new().key_value("key", "value");

    assert_eq!(output.blocks.len(), 1);
    match &output.blocks[0] {
        super::content::Block::KeyValue { entries } => {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].key, "key");
            assert_eq!(entries[0].value, "value");
        }
        _ => panic!("expected key_value block"),
    }
}

#[test]
fn multiple_key_value_blocks() {
    let output = Output::new()
        .key_value("a", "1")
        .heading(1, "Title")
        .key_value("b", "2");

    assert_eq!(output.blocks.len(), 3);
}

#[test]
fn from_serializable_with_object() {
    let json = serde_json::json!({ "name": "test", "count": 42 });
    let output = Output::from_serializable(json);

    assert_eq!(output.data.len(), 2);
    assert!(output.data.contains_key("name"));
    assert!(output.data.contains_key("count"));
}

#[test]
fn from_serializable_with_non_object() {
    let output = Output::from_serializable("string value");

    assert_eq!(output.data.len(), 1);
    assert!(output.data.contains_key("value"));
}

#[test]
fn section_creates_heading_and_code() {
    let output = Output::new().section("Setup", "npm install", Some("bash".into()));

    assert_eq!(output.blocks.len(), 2);

    // First block should be heading
    match &output.blocks[0] {
        super::content::Block::Heading { level, text } => {
            assert_eq!(*level, 2);
            assert_eq!(text, "Setup");
        }
        _ => panic!("expected heading block"),
    }

    // Second block should be code
    match &output.blocks[1] {
        super::content::Block::Code { language, code } => {
            assert_eq!(language, &Some("bash".into()));
            assert_eq!(code, "npm install");
        }
        _ => panic!("expected code block"),
    }
}

#[test]
fn jsonl_with_no_records_errors() {
    let output = Output::new();
    let err = super::render::render_jsonl(&output).unwrap_err();
    assert!(err.to_string().contains("requires"));
}

#[test]
fn data_with_serialize() {
    #[derive(serde::Serialize)]
    struct Config {
        port: u16,
        #[allow(dead_code)]
        host: String,
    }

    let config = Config {
        port: 8080,
        host: "localhost".to_string(),
    };

    let output = Output::new().data("config", config);
    let rendered = super::render::render_output(Format::Json, &output).unwrap();
    assert!(rendered.contains("8080"));
}

#[test]
fn text_paragraph_with_long_content() {
    let long_text = "a".repeat(200);
    let output = Output::new().paragraph(&long_text);

    let rendered = super::render::render_text(&output).unwrap();
    assert!(rendered.contains(&long_text));
}

#[test]
fn markdown_with_special_characters() {
    let output = Output::new()
        .heading(1, "Features & Benefits")
        .paragraph("**Bold** and *italic* text");

    let rendered = super::render::render_markdown(&output).unwrap();
    assert!(rendered.contains("Features & Benefits"));
    assert!(rendered.contains("**Bold**"));
}
