//! Table layout variants — Full, Compact, and Stacked.
//!
//! Demonstrates all three layout styles for rendering tabular data in different
//! scenarios. Full is the default; Compact minimizes spacing; Stacked shows
//! each row as key-value pairs, ideal for narrow terminals.
//!
//! Run with:
//! ```sh
//! cargo run --example table_layouts
//! ```

use scriba::{Format, Output, Table, Ui};

fn main() -> scriba::Result<()> {
    let headers = vec!["Product".into(), "Price".into(), "Stock".into()];
    let rows = vec![
        vec!["Widget A".into(), "$9.99".into(), "42".into()],
        vec!["Widget B".into(), "$14.50".into(), "18".into()],
        vec!["Widget C".into(), "$24.99".into(), "0".into()],
    ];

    let ui = Ui::new().with_format(Format::Text);

    // Full layout (default) — bordered, full width
    println!("=== FULL LAYOUT ===");
    println!("(Bordered, full width — best for normal terminals)\n");

    let table_full = Table::new(headers.clone(), rows.clone()).with_layout_full();
    let output_full = Output::new()
        .heading(2, "Inventory")
        .table(None, table_full);

    ui.print(&output_full)?;

    // Compact layout — minimal spacing, no borders
    println!("\n=== COMPACT LAYOUT ===");
    println!("(Minimal spacing, no borders — dense display)\n");

    let table_compact = Table::new(headers.clone(), rows.clone()).with_layout_compact();
    let output_compact = Output::new()
        .heading(2, "Inventory")
        .table(None, table_compact);

    ui.print(&output_compact)?;

    // Stacked layout — key-value per row
    println!("\n=== STACKED LAYOUT ===");
    println!("(Key-value per row — great for narrow terminals)\n");

    let table_stacked = Table::new(headers.clone(), rows.clone()).with_layout_stacked();
    let output_stacked = Output::new()
        .heading(2, "Inventory")
        .table(None, table_stacked);

    ui.print(&output_stacked)?;

    // Stacked with index
    println!("\n=== STACKED LAYOUT WITH INDEX ===");
    println!("(Row numbers for reference)\n");

    let table_stacked_idx = Table::new(headers.clone(), rows.clone())
        .with_index()
        .with_layout_stacked();
    let output_idx = Output::new()
        .heading(2, "Inventory")
        .table(None, table_stacked_idx);

    ui.print(&output_idx)?;

    // Compact with index
    println!("\n=== COMPACT LAYOUT WITH INDEX ===");
    println!("(Minimal display with row numbers)\n");

    let table_compact_idx = Table::new(headers.clone(), rows.clone())
        .with_index()
        .with_layout_compact();
    let output_compact_idx = Output::new()
        .heading(2, "Inventory")
        .table(None, table_compact_idx);

    ui.print(&output_compact_idx)?;

    Ok(())
}
