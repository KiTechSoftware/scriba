//! Text styling — applying semantic styles to output content.
//!
//! Demonstrates bold, italic, underline, strikethrough, dim styles across
//! different formats (Text with ANSI codes, Markdown with syntax).
//!
//! Run with:
//! ```sh
//! cargo run --example styling
//! ```

use scriba::{Format, Output, TextStyle, Styled, Ui};

fn main() -> scriba::Result<()> {
    // Text format with ANSI styling
    println!("=== TEXT FORMAT (with ANSI styling) ===\n");

    let ui_text = Ui::new().with_format(Format::Text);

    let output_text = Output::new()
        .heading(1, "Text Styling Examples")
        .paragraph("Watch how different styles are rendered:")
        .line("")
        .styled_paragraph(Styled::new("This is bold text", TextStyle::Bold))
        .styled_paragraph(Styled::new("This is italic text", TextStyle::Italic))
        .styled_paragraph(Styled::new("This is bold and italic", TextStyle::BoldItalic))
        .styled_paragraph(Styled::new("This is underlined", TextStyle::Underline))
        .styled_paragraph(Styled::new("This is strikethrough", TextStyle::Strikethrough))
        .styled_paragraph(Styled::new("This is dimmed/faded", TextStyle::Dim));

    ui_text.print(&output_text)?;

    // Markdown format with Markdown syntax
    println!("\n=== MARKDOWN FORMAT ===\n");

    let ui_md = Ui::new().with_format(Format::Markdown);

    let output_md = Output::new()
        .heading(1, "Text Styling in Markdown")
        .paragraph("These styles use Markdown syntax:")
        .line("")
        .styled_paragraph(Styled::new("This is bold", TextStyle::Bold))
        .styled_paragraph(Styled::new("This is italic", TextStyle::Italic))
        .styled_paragraph(Styled::new("This is bold italic", TextStyle::BoldItalic))
        .styled_paragraph(Styled::new("This has strikethrough", TextStyle::Strikethrough));

    ui_md.print(&output_md)?;

    // JSON format (styles stored for downstream processing)
    println!("\n=== JSON FORMAT ===\n");

    let ui_json = Ui::new().with_format(Format::Json);

    let output_json = Output::new()
        .data("style_info", "Text styles are preserved in JSON for downstream processing")
        .data("bold_example", "Important text")
        .data("italic_example", "Optional detail");

    ui_json.print(&output_json)?;

    // Programmatic use
    println!("\n=== DIRECT STYLE RENDERING ===\n");

    let bold_styled = Styled::new("Warning", TextStyle::Bold);
    let italic_styled = Styled::new("Information", TextStyle::Italic);

    println!(
        "ANSI rendering: {}",
        bold_styled.render_ansi()
    );
    println!("Markdown rendering: {}", bold_styled.render_markdown());
    println!(
        "Italic ANSI: {}",
        italic_styled.render_ansi()
    );
    println!("Italic Markdown: {}", italic_styled.render_markdown());

    Ok(())
}
