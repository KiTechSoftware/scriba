//! Nested JSON envelope layout.
//!
//! `ok`, `format`, and all user metadata are merged under the `meta` key.
//! The content payload sits alongside at the top level.
//!
//! Output shape:
//! ```json
//! {
//!   "meta": { "ok": true, "format": "json", "dry_run": true, "timestamp": "..." },
//!   "content": { ... }
//! }
//! ```
//!
//! Run with:
//! ```sh
//! cargo run --example envelope_nested
//! ```

use scriba::{
    Format, Output,
    Ui,
    envelope::{EnvelopeConfig, EnvelopeLayout, EnvelopeMode, Meta},
};

fn main() -> scriba::Result<()> {
    let output = Output::new()
        .title("Pipeline")
        .data("stage", "build")
        .data("commit", "a3f9c12")
        .paragraph("Build succeeded in 42s.");

    let ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Nested),
        );

    println!("=== Nested envelope, no meta ===");
    ui.print(&output)?;

    println!("\n=== Nested envelope, with meta merged in ===");
    let meta = Meta::default()
        .with_dry_run(true)
        .with_command("ci run".into())
        .with_timestamp("2026-04-13T09:30:00Z".into())
        .with_scope("ci".into())
        .with_version("0.3.0".into());
    ui.print_with_meta(&output, Some(&meta), true)?;

    Ok(())
}
