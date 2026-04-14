//! Text styling — applying semantic styles to output content.
//!
//! Demonstrates bold, italic, underline, strikethrough, dim styles across
//! different formats (Text with ANSI codes, Markdown with syntax).
//!
//! Run with:
//! ```sh
//! cargo run --example styling
//! ```

use scriba::{Format, Output, Styled, TextStyle, Ui};

fn main() -> scriba::Result<()> {
    // Text format — ANSI escape codes applied
    println!("=== TEXT FORMAT (ANSI codes) ===\n");

    let ui_text = Ui::new().with_format(Format::Text);

    let output_text = Output::new()
        .heading(1, "Text Styling Examples")
        .styled_paragraph(Styled::new("This is bold text", TextStyle::Bold))
        .styled_paragraph(Styled::new("This is italic text", TextStyle::Italic))
        .styled_paragraph(Styled::new("This is bold and italic", TextStyle::BoldItalic))
        .styled_paragraph(Styled::new("This is underlined", TextStyle::Underline))
        .styled_paragraph(Styled::new("This is strikethrough", TextStyle::Strikethrough))
        .styled_paragraph(Styled::new("This is dimmed/faded", TextStyle::Dim));

    ui_text.print(&output_text)?;

    // Markdown format — Markdown syntax applied
    println!("\n=== MARKDOWN FORMAT ===\n");

    let ui_md = Ui::new().with_format(Format::Markdown);

    let output_md = Output::new()
        .heading(1, "Text Styling in Markdown")
        .styled_paragraph(Styled::new("This is bold", TextStyle::Bold))
        .styled_paragraph(Styled::new("This is italic", TextStyle::Italic))
        .styled_paragraph(Styled::new("This is bold italic", TextStyle::BoldItalic))
        .styled_paragraph(Styled::new("This has strikethrough", TextStyle::Strikethrough))
        .styled_paragraph(Styled::new("This is underlined", TextStyle::Underline));

    ui_md.print(&output_md)?;

    // Direct use of Styled API
    println!("\n=== DIRECT API ===\n");

    let bold_styled = Styled::new("Warning: check your config", TextStyle::Bold);
    let italic_styled = Styled::new("Optional: skip if unneeded", TextStyle::Italic);
    let dim_styled = Styled::new("Hint: use --verbose for more detail", TextStyle::Dim);

    println!("ANSI:");
    println!("  {}", bold_styled.render_ansi());
    println!("  {}", italic_styled.render_ansi());
    println!("  {}", dim_styled.render_ansi());

    println!("\nMarkdown:");
    println!("  {}", bold_styled.render_markdown());
    println!("  {}", italic_styled.render_markdown());
    println!("  {}", dim_styled.render_markdown());

    Ok(())
}
