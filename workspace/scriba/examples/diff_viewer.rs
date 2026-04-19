//! Diff viewer — display and inspect unified diffs with syntax highlighting.
//!
//! Demonstrates how to use the new diff viewing capabilities with Scriba's
//! structured output. Shows both simple and colored diff rendering.
//!
//! Run with:
//! ```sh
//! cargo run --example diff_viewer
//! ```

use scriba::{Format, Output, Ui};

fn main() -> scriba::Result<()> {
    // Example unified diff content
    let patch = r#"--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,6 @@
 fn main() {
-    println!("Hello, world!");
+    println!("Hello, Scriba!");
+    println!("Diff viewer is great!");
     
     let x = 42;
-    println!("x = {}", x);
+    println!("x = {}", x * 2);
"#;

    println!("=== TEXT FORMAT (default) ===\n");
    let ui_text = Ui::new().with_format(Format::Text);
    ui_text.show_diff("src/main.rs", patch)?;

    println!("\n=== MARKDOWN FORMAT ===\n");
    let ui_markdown = Ui::new().with_format(Format::Markdown);
    ui_markdown.show_diff("src/main.rs", patch)?;

    println!("\n=== COLORED DIFF (for terminal output) ===\n");
    let ui_colored = Ui::new().with_format(Format::Text);
    ui_colored.show_diff_colored("src/main.rs", patch, true)?;

    println!("\n=== JSON FORMAT ===\n");
    let ui_json = Ui::new().with_format(Format::Json);
    ui_json.show_diff("src/main.rs", patch)?;

    // Example: Working with parsed diff lines
    println!("\n=== PARSED DIFF LINES ===\n");
    let diff_lines = scriba::parse_diff(patch);
    let mut output = Output::new()
        .title("Parsed Diff Analysis")
        .subtitle("Breaking down the diff into structured lines");

    let mut stats = String::new();
    stats.push_str(&format!(
        "Total lines: {}\n",
        diff_lines.len()
    ));

    let added = diff_lines
        .iter()
        .filter(|l| l.kind == scriba::DiffLineKind::Added)
        .count();
    let removed = diff_lines
        .iter()
        .filter(|l| l.kind == scriba::DiffLineKind::Removed)
        .count();
    let context = diff_lines
        .iter()
        .filter(|l| l.kind == scriba::DiffLineKind::Context)
        .count();

    stats.push_str(&format!("Added: {}\n", added));
    stats.push_str(&format!("Removed: {}\n", removed));
    stats.push_str(&format!("Context: {}", context));

    output = output.section("Statistics", stats, "text".to_string());

    let ui_stats = Ui::new().with_format(Format::Markdown);
    ui_stats.print(&output)?;

    Ok(())
}
