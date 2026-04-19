# scriba

**Typed CLI output, prompts, and terminal rendering for Rust.**

`scriba` helps you build clean, structured command-line interfaces with composable output primitives, interactive prompts, styled logging, and optional ASCII banners.

## Features

- 📄 Multi-format rendering: Plain, Text, Markdown, JSON, JSONL
- 🧱 Typed output blocks and builders
- 💬 Interactive prompts (feature: `prompt`)
- 🎨 Styled logging (feature: `logger`)
- 🔤 ASCII banners / figlet rendering (feature: `figlet`)
- 📊 Structured diff viewer with format support
- ⚙️ Feature-gated integrations
- 🦀 Ergonomic Rust-first APIs

## Installation

```toml
[dependencies]
scriba = "0.5"

# optional features
scriba = { version = "0.5", features = ["prompt", "logger", "figlet"] }
```

### Feature Flags

| Feature  | Enables                                        |
| -------- | ---------------------------------------------- |
| `prompt` | Interactive terminal prompts via `inquire`     |
| `logger` | Styled stderr logging                          |
| `figlet` | ASCII banner rendering                         |
| `full`   | All optional features (prompt, logger, figlet) |

## Quick Start

```rust
use scriba::{Format, Output, Ui};

fn main() -> scriba::Result<()> {
    let ui = Ui::new().with_format(Format::Markdown);

    let output = Output::new()
        .heading(1, "scriba")
        .paragraph("Clean CLI rendering.");

    ui.print(&output)?;
    Ok(())
}
```

Output:

```md
# scriba

Clean CLI rendering.
```

## Output Formats

Supported render targets:

- `Format::Plain`
- `Format::Text`
- `Format::Markdown`
- `Format::Json`
- `Format::Jsonl`

```rust
let ui = Ui::new().with_format(Format::Text);
```

## Core Output Primitives

All output is composed with `Output::new()`.

## Titles and Headings

```rust
let output = Output::new()
    .title("Deploy")
    .subtitle("production")
    .heading(2, "Summary")
    .paragraph("Release completed.");
```

## Paragraphs, Lines, Lists

```rust
let output = Output::new()
    .paragraph("Main description.")
    .line("single line")
    .list(false, vec![
        "alpha".into(),
        "beta".into(),
    ]);
```

Ordered list:

```rust
.list(true, vec!["one".into(), "two".into()])
```

## Separators

```rust
let output = Output::new()
    .paragraph("Before")
    .separator()
    .paragraph("After");
```

## Code Blocks

```rust
let output = Output::new()
    .code(Some("rust".into()), r#"fn main() {}"#);
```

## Plain Output

Use when your command should emit one scalar value for scripts or shell pipelines.

```rust
use scriba::{Format, Output, Ui};

let ui = Ui::new().with_format(Format::Plain);
let output = Output::new().plain("hello");

ui.print(&output)?;
```

Output:

```text
hello
```

Supported scalar values:

- string
- number
- boolean
- null

## Structured Data

### Key / Value Data Map

Use top-level structured fields with `.data(...)`.

```rust
let output = Output::new()
    .data("version", "1.0")
    .data("ready", true);
```

Useful for JSON, Markdown summaries, and metadata.

### JSON Block

```rust
let output = Output::new()
    .json(serde_json::json!({
        "name": "scriba",
        "ok": true
    }));
```

### JSON Lines (JSONL)

Use `jsonl_record(...)` for streaming records or line-delimited events.

```rust
use scriba::{Format, Output, Ui};

let ui = Ui::new().with_format(Format::Jsonl);

let output = Output::new()
    .jsonl_record(serde_json::json!({
        "event": "start",
        "id": 1
    }))
    .jsonl_record(serde_json::json!({
        "event": "done",
        "id": 1
    }));

ui.print(&output)?;
```

Output:

```json
{"event":"start","id":1}
{"event":"done","id":1}
```

If no explicit records are provided, blocks can be emitted as JSONL entries.

### Key / Value Blocks

Use for compact metadata sections.

```rust
let output = Output::new()
    .key_value("project", "scriba")
    .key_value("env", "prod");
```

Markdown:

```md
- **project**: scriba
- **env**: prod
```

Text:

```text
project: scriba
env: prod
```

Sequential calls are grouped automatically.

### Definition Lists

Use for glossary-style output or descriptive labels.

```rust
let output = Output::new()
    .definition("Project", "scriba")
    .definition("Environment", "production");
```

Text:

```text
Project:
  scriba

Environment:
  production
```

### Status Messages

Use semantic states for results and summaries.

```rust
use scriba::{Output, StatusKind};

let output = Output::new()
    .status(StatusKind::Ok, "Deployment complete")
    .status(StatusKind::Warning, "Using cached config");
```

Available kinds:

- `StatusKind::Info`
- `StatusKind::Ok`
- `StatusKind::Warning`
- `StatusKind::Error`

### Tables

```rust
use scriba::{Output, Table};

let table = Table::new(
    vec!["name".into(), "value".into()],
    vec![
        vec!["alpha".into(), "1".into()],
        vec!["beta".into(), "2".into()],
    ],
);

let output = Output::new()
    .table(Some("Items".into()), table);
```

#### Indexed Tables

Add row numbers automatically.

```rust
let table = Table::new(headers, rows).with_index();
```

Custom header:

```rust
let table = Table::new(headers, rows)
    .with_index_header("row");
```

#### Table Layouts

Control spacing, borders, and overall presentation with `TableLayout`:

- `TableLayout::Full` (default): Bordered tables with full width and padding
- `TableLayout::Compact`: Minimal spacing, no borders — dense display
- `TableLayout::Stacked`: Key-value format per row — ideal for narrow terminals

```rust
use scriba::{Output, Table, TableLayout};

let table = Table::new(headers, rows)
    .with_layout(TableLayout::Compact);

let output = Output::new().table(Some("Items".into()), table);
```

Shorthand builders:

```rust
let table = Table::new(headers, rows).with_layout_compact();
let table = Table::new(headers, rows).with_layout_stacked();
let table = Table::new(headers, rows).with_layout_full();  // default
```

Stacked layout output:

```text
Name: alpha
Value: 1
---
Name: beta
Value: 2
```

See `cargo run --example table_layouts` for all variants with and without index.

## Text Styling

Apply semantic styles to text content: bold, italic, underline, strikethrough, and dim.
Styles render format-appropriately (ANSI codes for Text, Markdown syntax, etc.).

```rust
use scriba::{Output, TextStyle, Styled};

let output = Output::new()
    .styled_paragraph(Styled::new("Important", TextStyle::Bold))
    .styled_paragraph(Styled::new("Optional", TextStyle::Italic))
    .styled_paragraph(Styled::new("Striked", TextStyle::Strikethrough));
```

Available styles:

- `TextStyle::Bold` — Strong emphasis
- `TextStyle::Italic` — Emphasis
- `TextStyle::BoldItalic` — Combined
- `TextStyle::Underline` — Underlined
- `TextStyle::Strikethrough` — ~~Crossed out~~
- `TextStyle::Dim` — Faded/dimmed

Direct rendering:

```rust
let styled = Styled::new("Warning", TextStyle::Bold);

// ANSI codes for Text format
println!("{}", styled.render_ansi());

// Markdown syntax
println!("{}", styled.render_markdown());
```

See `cargo run --example styling` for comprehensive examples.

## Prompts (`prompt` feature)

```rust
use scriba::Ui;

fn main() -> scriba::Result<()> {
    let ui = Ui::new().interactive(true);

    let name = ui.text("Project name?", None, None)?;
    println!("Hello {name}");

    Ok(())
}
```

### Confirm

```rust
let proceed = ui.confirm("Continue?", true)?;
```

### Select

```rust
use scriba::{SelectOption, SelectRequest};

let env = ui.select(&SelectRequest::new(
    "Select environment",
    vec![
        SelectOption::new("dev", "Development"),
        SelectOption::new("prod", "Production"),
    ],
))?;
```

### Multi Select

```rust
use scriba::{MultiSelectOption, MultiSelectRequest};

let values = ui.multiselect(&MultiSelectRequest::new(
    "Choose targets",
    vec![
        MultiSelectOption::new("api", "API").selected(true),
        MultiSelectOption::new("web", "Web"),
    ],
))?;
```

### Pagination

Both `SelectRequest` and `MultiSelectRequest` support an optional page size for long option lists:

```rust
let env = ui.select(&SelectRequest::new("Select environment", options)
    .with_page_size(10))?;

let targets = ui.multiselect(&MultiSelectRequest::new("Choose targets", options)
    .with_page_size(10))?;
```

### Theming

Customize prompt colors and styles with `PromptTheme`:

```rust
use scriba::{Ui, prompt::PromptTheme};

// Built-in themes
let ui_dark = Ui::new().with_prompt_theme(PromptTheme::dark());
let ui_light = Ui::new().with_prompt_theme(PromptTheme::light());
let ui_mono = Ui::new().with_prompt_theme(PromptTheme::monochrome());

// Custom theme
let custom = PromptTheme::default()
    .with_question_color("magenta")
    .with_selected_color("cyan")
    .with_input_color("green");

let ui = Ui::new().with_prompt_theme(custom);
```

Available themes:

| Theme | Best For |
|-------|----------|
| `PromptTheme::default()` | Standard terminal colors |
| `PromptTheme::dark()` | Dark terminal backgrounds |
| `PromptTheme::light()` | Light terminal backgrounds |
| `PromptTheme::monochrome()` | Accessibility (no colors) |

Theme fields (all customizable):

- `question_color` — Prompt text
- `input_color` — User input
- `selected_color` — Highlighted items
- `unselected_color` — Non-highlighted items
- `hint_color` — Help text
- `success_color` — Success messages
- `error_color` — Error messages

Access the active theme:

```rust
let ui = Ui::new().with_prompt_theme(PromptTheme::dark());
let theme = ui.prompt_theme();
println!("Question color: {}", theme.question_color);
```

See `cargo run --example prompt_theming --features prompt` for all themes.

## Envelope

Wrap any output in a JSON envelope with a configurable layout and optional
execution metadata. Enabled by setting `EnvelopeMode::Json` on the `Ui`.

### Flat layout (default)

```rust
use scriba::{
    Format, Output, Ui,
    envelope::{EnvelopeConfig, EnvelopeLayout, EnvelopeMode, Meta},
};

fn main() -> scriba::Result<()> {
    let output = Output::new()
        .data("environment", "production")
        .data("version", "1.4.2");

    let meta = Meta::default()
        .with_command("deploy".into())
        .with_duration_ms(312)
        .with_dry_run(false);

    let ui = Ui::new()
        .with_format(Format::Json)
        .with_envelope(
            EnvelopeConfig::default()
                .with_mode(EnvelopeMode::Json)
                .with_layout(EnvelopeLayout::Flat),
        );

    ui.print_with_meta(&output, Some(&meta), true)?;
    Ok(())
}
```

Output:

```json
{
  "ok": true,
  "format": "json",
  "content": { "environment": "production", "version": "1.4.2" },
  "meta": { "command": "deploy", "duration_ms": 312, "dry_run": false }
}
```

### Nested layout

`ok`, `format`, and all metadata fields are merged under the `meta` key.

```rust
let ui = Ui::new()
    .with_format(Format::Json)
    .with_envelope(
        EnvelopeConfig::default()
            .with_mode(EnvelopeMode::Json)
            .with_layout(EnvelopeLayout::Nested),
    );

ui.print_with_meta(&output, Some(&meta), true)?;
```

Output:

```json
{
  "meta": { "ok": true, "format": "json", "command": "deploy", "dry_run": false },
  "content": { ... }
}
```

### Custom field names

```rust
use scriba::envelope::{EnvelopeConfig, EnvelopeFields, EnvelopeMode};

let ui = Ui::new()
    .with_format(Format::Json)
    .with_envelope(
        EnvelopeConfig::default()
            .with_mode(EnvelopeMode::Json)
            .with_fields(EnvelopeFields {
                ok_field: "success".into(),
                format_field: "type".into(),
                content_field: "result".into(),
                meta_field: "context".into(),
            }),
    );
```

### Meta

`Meta` carries optional structured context about the command invocation.

```rust
use scriba::envelope::Meta;

let meta = Meta::default()
    .with_command("deploy".into())
    .with_version("1.0.0".into())
    .with_scope("production".into())
    .with_duration_ms(342)
    .with_timestamp("2025-01-01T00:00:00Z".into())
    .with_dry_run(false)
    .with_extra("region", "us-east-1")
    .with_extra("actor", "ci")
    .with_extra_map([
        ("trace_id".into(), serde_json::json!("abc-123")),
        ("run_id".into(), serde_json::json!(42)),
    ]);
```

All fields are optional and omitted from the JSON output when not set.

### Runnable examples

```sh
cargo run --example envelope_flat
cargo run --example envelope_nested
cargo run --example envelope_custom_fields
cargo run --example envelope_meta
```

## Logger (`logger` feature)

```rust
use scriba::Ui;

let ui = Ui::new();

ui.logger().info("starting");
ui.logger().ok("done");
ui.logger().warn("careful");
ui.logger().error("failed");
ui.logger().detail("verbose detail");
ui.logger().debug("debug line");
ui.logger().trace("trace line");
ui.logger().kv("region", "us-east-1");
```

## Figlet (`figlet` feature)

```rust
use scriba::figlet;

let banner = figlet::render("scriba")?;
println!("{banner}");
```

Custom font:

```rust
let banner = figlet::render_with_font("scriba", "slant")?;
```

Built-in fonts include:

- `standard`
- `small`
- `big`
- `slant`
- `smblock`
- `mono12`
- `future`
- `wideterm`
- `mono9`

## Design Goals

`scriba` is built around:

- simple APIs first
- standard types where possible
- composable builders
- feature-gated integrations
- no macros required

## Recommended Primitive by Use Case

| Need                  | Use                    |
| --------------------- | ---------------------- |
| Single shell value    | `plain()`              |
| Human-readable report | headings + paragraphs  |
| Metadata              | `key_value()`          |
| Glossary / labels     | `definition()`         |
| State / result        | `status()`             |
| Structured object     | `data()` / `json()`    |
| Event stream          | `jsonl_record()`       |
| Tabular data          | `table()`              |
| Numbered rows         | `with_index()`         |
| Table presentation    | `TableLayout`          |
| Text formatting       | `TextStyle` + `Styled` |
| Prompt appearance     | `PromptTheme`          |
| JSON envelope         | `EnvelopeConfig`       |
| Execution metadata    | `Meta`                 |

## Roadmap

### Backlog

- [ ] output streaming
- [ ] optional derive macros

## License

MIT OR Apache-2.0
