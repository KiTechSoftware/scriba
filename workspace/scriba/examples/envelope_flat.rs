//! Flat JSON envelope — the default layout.
//!
//! All fields (`ok`, `format`, content, `meta`) appear at the top level.
//!
//! Run with:
//! ```sh
//! cargo run --example envelope_flat
//! ```

use scriba::{
    Format, Output,
    Ui,
    envelope::{EnvelopeConfig, EnvelopeLayout, EnvelopeMode, Meta},
};

fn main() -> scriba::Result<()> {
    let output = Output::new()
        .title("Deployment")
        .data("environment", "production")
        .data("version", "1.4.2")
        .paragraph("All services healthy.");

    let ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Flat),
        );

    println!("=== Flat envelope, no meta ===");
    ui.print(&output)?;

    println!("\n=== Flat envelope, with meta ===");
    let meta = Meta::default()
        .with_dry_run(false)
        .with_command("deploy".into())
        .with_duration_ms(312)
        .with_timestamp("2026-04-13T15:00:00Z".into());
    ui.print_with_meta(&output, Some(&meta), true)?;

    println!("\n=== Flat envelope, ok: false (error case) ===");
    let error_output = Output::new()
        .title("Deployment")
        .paragraph("Health check failed on eu-west-1.");
    ui.print_with_meta(&error_output, None, false)?;

    Ok(())
}
