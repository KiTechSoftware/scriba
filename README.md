# scriba

**Typed CLI output, prompts, and terminal rendering for Rust.**

`scriba` helps you build clean, structured command-line interfaces with composable output primitives, interactive prompts, styled logging, and optional ASCII banners.

## Features

- 📄 Multi-format rendering: Plain, Text, Markdown, JSON, JSONL
- 🧱 Typed output blocks and builders
- 💬 Interactive prompts (feature: `prompt`)
- 🎨 Styled logging (feature: `logger`)
- 🔤 ASCII banners / figlet rendering (feature: `figlet`)
- ⚙️ Feature-gated integrations
- 🦀 Ergonomic Rust-first APIs

## Installation

```toml
[dependencies]
scriba = "0.1"

# optional features
scriba = { version = "0.2", features = ["prompt", "logger", "figlet"] }
```

### Feature Flags

| Feature  | Enables                                    |
| -------- | ------------------------------------------ |
| `prompt` | Interactive terminal prompts via `inquire` |
| `logger` | Styled stderr logging                      |
| `figlet` | ASCII banner rendering                     |

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

## Logger (`logger` feature)

```rust
use scriba::Ui;

let ui = Ui::new();

ui.logger().info("starting")?;
ui.logger().ok("done")?;
ui.logger().warn("careful")?;
ui.logger().error("failed")?;
ui.logger().detail("verbose detail")?;
ui.logger().debug("debug line")?;
ui.logger().trace("trace line")?;
ui.logger().kv("region", "us-east-1")?;
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

| Need                  | Use                   |
| --------------------- | --------------------- |
| Single shell value    | `plain()`             |
| Human-readable report | headings + paragraphs |
| Metadata              | `key_value()`         |
| Glossary / labels     | `definition()`        |
| State / result        | `status()`            |
| Structured object     | `data()` / `json()`   |
| Event stream          | `jsonl_record()`      |
| Tabular data          | `table()`             |
| Numbered rows         | `with_index()`        |

## Roadmap

- [ ] richer styling options
- [ ] table layout variants
- [ ] output streaming
- [ ] optional derive macros

## License

MIT OR Apache-2.0
