//! Custom envelope field names.
//!
//! Override the default `ok`, `format`, `content`, and `meta` field names to
//! match your existing API conventions.
//!
//! Run with:
//! ```sh
//! cargo run --example envelope_custom_fields
//! ```

use scriba::{
    Format, Output,
    Ui,
    envelope::{EnvelopeConfig, EnvelopeFields, EnvelopeLayout, EnvelopeMode, Meta},
};

fn main() -> scriba::Result<()> {
    let output = Output::new()
        .title("Users")
        .data("count", 42u64)
        .paragraph("Query executed successfully.");

    // --- Custom flat field names ---
    let flat_ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Flat)
                .with_fields(EnvelopeFields {
                    ok_field: "success".into(),
                    format_field: "type".into(),
                    content_field: "result".into(),
                    meta_field: "context".into(),
                }),
        );

    println!("=== Custom field names, flat ===");
    println!(r#"  fields: success / type / result / context"#);
    flat_ui.print(&output)?;

    // --- Custom nested field names ---
    let nested_ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Nested)
                .with_fields(EnvelopeFields {
                    ok_field: "success".into(),
                    format_field: "type".into(),
                    content_field: "data".into(),
                    meta_field: "header".into(),
                }),
        );

    let meta = Meta::default()
        .with_command("users list".into())
        .with_extra("region", "eu-west-1");

    println!("\n=== Custom field names, nested ===");
    println!(r#"  fields: success / type / data / header"#);
    nested_ui.print_with_meta(&output, Some(&meta), true)?;

    // --- Omit ok and format fields entirely ---
    let minimal_ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_show_ok(false)
                .with_show_format(false),
        );

    println!("\n=== Envelope without ok or format fields ===");
    minimal_ui.print(&output)?;

    Ok(())
}
