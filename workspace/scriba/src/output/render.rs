use serde_json::Value;
use std::fmt::Write as _;

use crate::{error::Result, error::ScribaError, Format};

use super::{table, Block, Output, StatusKind};

pub fn render_output(format: Format, output: &Output) -> Result<String> {
    match format {
        Format::Plain => render_plain(output),
        Format::Text => render_text(output),
        Format::Markdown => render_markdown(output),
        Format::Json => Ok(serde_json::to_string_pretty(output)?),
        Format::Jsonl => render_jsonl(output),
    }
}

pub fn render_output_value(format: Format, output: &Output) -> Result<Value> {
    match format {
        Format::Json => Ok(serde_json::to_value(output)?),
        _ => Ok(Value::String(render_output(format, output)?)),
    }
}

pub fn render_plain(output: &Output) -> Result<String> {
    match &output.plain {
        Some(Value::String(s)) => Ok(format!("{s}\n")),
        Some(Value::Number(n)) => Ok(format!("{n}\n")),
        Some(Value::Bool(b)) => Ok(format!("{b}\n")),
        Some(Value::Null) => Ok("null\n".to_string()),
        Some(_) => Err(ScribaError::Render(
            "plain output must be a string, number, boolean, or null".to_string(),
        )),
        None => Err(ScribaError::Render(
            "plain output requires a primary scalar value".to_string(),
        )),
    }
}

pub fn render_text(output: &Output) -> Result<String> {
    let mut out = String::new();

    if let Some(title) = &output.title {
        out.push_str(title);
        out.push('\n');
        out.push_str(&"=".repeat(title.chars().count()));
        out.push_str("\n\n");
    }

    if let Some(subtitle) = &output.subtitle {
        out.push_str(subtitle);
        out.push_str("\n\n");
    }

    if !output.data.is_empty() {
        for (key, value) in &output.data {
            writeln!(out, "{key}: {}", value_to_inline_string(value)).ok();
        }
        out.push('\n');
    }

    for block in &output.blocks {
        render_text_block(block, &mut out)?;
    }

    Ok(out.trim_end().to_string() + "\n")
}

pub fn render_markdown(output: &Output) -> Result<String> {
    let mut out = String::new();

    if let Some(title) = &output.title {
        writeln!(out, "# {title}").ok();
        out.push('\n');
    }

    if let Some(subtitle) = &output.subtitle {
        writeln!(out, "_{subtitle}_").ok();
        out.push('\n');
    }

    if !output.data.is_empty() {
        for (key, value) in &output.data {
            writeln!(out, "- **{key}**: {}", value_to_inline_string(value)).ok();
        }
        out.push('\n');
    }

    for block in &output.blocks {
        render_markdown_block(block, &mut out)?;
    }

    Ok(out.trim_end().to_string() + "\n")
}

pub fn render_jsonl(output: &Output) -> Result<String> {
    if !output.jsonl_records.is_empty() {
        let lines = output
            .jsonl_records
            .iter()
            .map(serde_json::to_string)
            .collect::<std::result::Result<Vec<_>, _>>()?;

        return Ok(lines.join("\n") + "\n");
    }

    if !output.blocks.is_empty() {
        let lines = output
            .blocks
            .iter()
            .map(|block| {
                serde_json::to_string(&serde_json::json!({
                    "title": output.title,
                    "subtitle": output.subtitle,
                    "block": block,
                }))
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

        return Ok(lines.join("\n") + "\n");
    }

    Err(ScribaError::Render(
        "jsonl output requires jsonl_records or blocks".to_string(),
    ))
}

fn render_text_block(block: &Block, out: &mut String) -> Result<()> {
    match block {
        Block::Heading { text, .. } => {
            out.push_str(text);
            out.push('\n');
            out.push_str(&"-".repeat(text.chars().count()));
            out.push_str("\n\n");
        }
        Block::Paragraph { text } => {
            out.push_str(text);
            out.push_str("\n\n");
        }
        Block::Line { text } => {
            out.push_str(text);
            out.push('\n');
        }
        Block::Separator => {
            out.push_str("----------------------------------------\n\n");
        }
        Block::List { ordered, items } => {
            for (idx, item) in items.iter().enumerate() {
                if *ordered {
                    writeln!(out, "{}. {}", idx + 1, item).ok();
                } else {
                    writeln!(out, "- {}", item).ok();
                }
            }
            out.push('\n');
        }
        Block::Code { code, .. } => {
            out.push_str(code);
            out.push_str("\n\n");
        }
        Block::Table { title, table: tbl } => {
            if let Some(title) = title {
                out.push_str(title);
                out.push('\n');
                out.push_str(&"-".repeat(title.chars().count()));
                out.push_str("\n");
            }

            out.push_str(&table::render_text_table(tbl)?);
            out.push_str("\n\n");
        }
        Block::Json { value } => {
            out.push_str(&serde_json::to_string_pretty(value)?);
            out.push_str("\n\n");
        }
        Block::KeyValue { entries } => {
            for entry in entries {
                writeln!(out, "{}: {}", entry.key, entry.value).ok();
            }
            out.push('\n');
        }
        Block::DefinitionList { entries } => {
            for entry in entries {
                writeln!(out, "{}:", entry.term).ok();
                writeln!(out, "  {}", entry.description).ok();
                out.push('\n');
            }
        }
        Block::Status { kind, text } => {
            writeln!(out, "[{}] {}", status_label(*kind), text).ok();
            out.push('\n');
        }
    }

    Ok(())
}

fn render_markdown_block(block: &Block, out: &mut String) -> Result<()> {
    match block {
        Block::Heading { level, text } => {
            let level = (*level).clamp(1, 6) as usize;
            writeln!(out, "{} {}", "#".repeat(level), text).ok();
            out.push('\n');
        }
        Block::Paragraph { text } => {
            out.push_str(text);
            out.push_str("\n\n");
        }
        Block::Line { text } => {
            out.push_str(text);
            out.push_str("  \n");
        }
        Block::Separator => {
            out.push_str("---\n\n");
        }
        Block::List { ordered, items } => {
            for (idx, item) in items.iter().enumerate() {
                if *ordered {
                    writeln!(out, "{}. {}", idx + 1, item).ok();
                } else {
                    writeln!(out, "- {}", item).ok();
                }
            }
            out.push('\n');
        }
        Block::Code { language, code } => {
            out.push_str("```");
            if let Some(language) = language {
                out.push_str(language);
            }
            out.push('\n');
            out.push_str(code);
            out.push_str("\n```\n\n");
        }
        Block::Table { title, table: tbl } => {
            if let Some(title) = title {
                writeln!(out, "## {title}").ok();
                out.push('\n');
            }

            out.push_str(&table::render_markdown_table(tbl)?);
            out.push_str("\n\n");
        }
        Block::Json { value } => {
            out.push_str("```json\n");
            out.push_str(&serde_json::to_string_pretty(value)?);
            out.push_str("\n```\n\n");
        }
        Block::KeyValue { entries } => {
            for entry in entries {
                writeln!(out, "- **{}**: {}", entry.key, entry.value).ok();
            }
            out.push('\n');
        }
        Block::DefinitionList { entries } => {
            for entry in entries {
                writeln!(out, "**{}**  ", entry.term).ok();
                writeln!(out, "{}", entry.description).ok();
                out.push('\n');
            }
        }
        Block::Status { kind, text } => {
            writeln!(out, "- **{}**: {}", status_label(*kind), text).ok();
            out.push('\n');
        }
    }

    Ok(())
}

fn value_to_inline_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => v.clone(),
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string(value).unwrap_or_else(|_| "<invalid json>".to_string())
        }
    }
}

fn status_label(kind: StatusKind) -> &'static str {
    match kind {
        StatusKind::Info => "info",
        StatusKind::Ok => "success",
        StatusKind::Warning => "warning",
        StatusKind::Error => "error",
        #[allow(deprecated)]
        StatusKind::Success => "success",
    }
}
