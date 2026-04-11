use serde_json::Value;
use tabled::{builder::Builder, settings::Style};

use crate::{error::Result, Format};

use super::Table;

pub fn render_table(format: Format, table: &Table) -> Result<String> {
    match format {
        Format::Plain | Format::Text => render_text_table(table),
        Format::Markdown => render_markdown_table(table),
        Format::Json => render_json_table(table),
        Format::Jsonl => render_jsonl_table(table),
    }
}

pub fn render_table_value(format: Format, table: &Table) -> Result<Value> {
    match format {
        Format::Json => Ok(table_to_json_value(table)),
        _ => Ok(Value::String(render_table(format, table)?)),
    }
}

pub fn render_text_table(table: &Table) -> Result<String> {
    let table = table.materialized();

    let mut builder = Builder::default();
    builder.push_record(table.headers.iter().map(|s| s.as_str()));

    for row in &table.rows {
        builder.push_record(row.iter().map(|s| s.as_str()));
    }

    let mut built = builder.build();
    built.with(Style::rounded());

    Ok(built.to_string())
}

pub fn render_markdown_table(table: &Table) -> Result<String> {
    let table = table.materialized();

    let header = format!("| {} |", table.headers.join(" | "));
    let separator = format!(
        "|{}|",
        table
            .headers
            .iter()
            .map(|_| " --- ")
            .collect::<Vec<_>>()
            .join("|")
    );

    let body = table
        .rows
        .iter()
        .map(|row| format!("| {} |", row.join(" | ")))
        .collect::<Vec<_>>()
        .join("\n");

    if body.is_empty() {
        Ok(format!("{header}\n{separator}"))
    } else {
        Ok(format!("{header}\n{separator}\n{body}"))
    }
}

pub fn render_json_table(table: &Table) -> Result<String> {
    Ok(serde_json::to_string_pretty(&table_to_json_value(table))?)
}

pub fn render_jsonl_table(table: &Table) -> Result<String> {
    let table = table.materialized();

    let lines = rows_to_records(&table)
        .into_iter()
        .map(|row| serde_json::to_string(&row))
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(lines.join("\n"))
}

pub fn table_to_json_value(table: &Table) -> Value {
    serde_json::to_value(table.materialized()).unwrap_or(Value::Null)
}

fn rows_to_records(table: &Table) -> Vec<serde_json::Map<String, Value>> {
    let table = table.materialized();

    table
        .rows
        .iter()
        .map(|row| {
            table
                .headers
                .iter()
                .zip(row.iter())
                .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                .collect::<serde_json::Map<String, Value>>()
        })
        .collect()
}
