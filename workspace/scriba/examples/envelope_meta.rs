//! Full `Meta` builder — all fields including arbitrary extras.
//!
//! Demonstrates every `Meta` field and the `with_extra()` method for arbitrary
//! additional key-value pairs in both flat and nested layouts.
//!
//! Run with:
//! ```sh
//! cargo run --example envelope_meta
//! ```

use scriba::{
    Format, Output,
    Ui,
    envelope::{EnvelopeConfig, EnvelopeLayout, EnvelopeMode, Meta},
};

fn main() -> scriba::Result<()> {
    let output = Output::new()
        .title("Release")
        .data("tag", "v0.3.0")
        .paragraph("Release pipeline completed.");

    let meta = Meta::default()
        .with_dry_run(false)
        .with_command("release publish".into())
        .with_duration_ms(4821)
        .with_timestamp("2026-04-13T18:00:00Z".into())
        .with_scope("production".into())
        .with_version("0.3.0".into())
        // Arbitrary extra fields — any serialisable value
        .with_extra("region", "eu-west-1")
        .with_extra("actor", "github-actions")
        .with_extra("run_id", 9_981_234_u64);

    println!("=== Full meta — flat layout ===");
    let flat_ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Flat),
        );
    flat_ui.print_with_meta(&output, Some(&meta), true)?;

    println!("\n=== Full meta — nested layout (all fields merged under meta) ===");
    let nested_ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Nested),
        );
    nested_ui.print_with_meta(&output, Some(&meta), true)?;

    println!("\n=== Meta with non-JSON output format (text rendered inside envelope) ===");
    let text_ui = Ui::new()
        .with_format(Format::Text)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Flat),
        );
    text_ui.print_with_meta(&output, Some(&meta), true)?;

    Ok(())
}
