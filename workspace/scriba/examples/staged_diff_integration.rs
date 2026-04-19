//! Integration example — replacing stderr diff viewing with Scriba output.
//!
//! This example demonstrates how to integrate the new diff viewer into your staging
//! workflow, replacing manual stderr printing with structured Scriba output.
//!
//! Run with:
//! ```sh
//! cargo run --example staged_diff_integration
//! ```

use scriba::{Format, Ui};

fn main() -> scriba::Result<()> {
    // Simulated git diff output
    let git_diff = r#"--- a/src/lib.rs
+++ b/src/lib.rs
@@ -10,8 +10,9 @@ pub struct Config {
     pub format: String,
 }
 
-impl Config {
+impl Config {
     pub fn new() -> Self {
+        // Initialize with defaults
         Self::default()
     }
 }
@@ -45,10 +46,12 @@ pub fn render() {
     println!("Rendering...");
 }
 
-pub fn cleanup() {
+pub fn cleanup() -> Result<()> {
     // Clean up resources
+    println!("Cleanup complete");
+    Ok(())
 }
"#;

    println!("=== BASIC USAGE ===\n");
    println!("Simple way to show a diff:\n");

    let ui = Ui::new().with_format(Format::Text);
    ui.show_diff("src/lib.rs", git_diff)?;

    println!("\n\n=== WITH METADATA ===\n");
    println!("Include diff in structured output with metadata:\n");

    let output = scriba::Output::new()
        .title("File Changes")
        .subtitle("Reviewing unstaged modifications")
        .data("file", "src/lib.rs")
        .data("status", "modified")
        .code(Some("diff".to_string()), git_diff.trim());

    ui.print(&output)?;

    println!("\n\n=== COLORED OUTPUT ===\n");
    println!("Terminal-friendly colored diff (for stderr fallback):\n");

    ui.show_diff_colored("src/lib.rs", git_diff, true)?;

    println!("\n\n=== MARKDOWN FORMAT ===\n");
    println!("Perfect for documentation or markdown reports:\n");

    let ui_md = Ui::new().with_format(Format::Markdown);
    ui_md.show_diff("src/lib.rs", git_diff)?;

    println!("\n\n=== PARSED DIFF ANALYSIS ===\n");
    println!("Work with structured diff data:\n");

    let diff_lines = scriba::parse_diff(git_diff);
    let mut analysis = scriba::Output::new()
        .title("Diff Analysis")
        .data("total_lines", diff_lines.len().to_string());

    let added = diff_lines.iter().filter(|l| l.kind == scriba::DiffLineKind::Added).count();
    let removed = diff_lines.iter().filter(|l| l.kind == scriba::DiffLineKind::Removed).count();

    analysis = analysis
        .data("lines_added", added.to_string())
        .data("lines_removed", removed.to_string())
        .data("net_change", format!("{:+}", added as i32 - removed as i32));

    ui.print(&analysis)?;

    Ok(())
}
