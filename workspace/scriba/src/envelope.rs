//! Envelope configuration for wrapping output in a JSON container.
//!
//! An envelope wraps rendered output in a JSON object with metadata fields
//! (`ok`, `format`, content, and optional `meta`). Supports flat and nested
//! layouts, fully customisable field names, and optional user-defined metadata.
//!
//! # Examples
//!
//! ```
//! use scriba::envelope::{EnvelopeConfig, EnvelopeMode, EnvelopeLayout};
//!
//! // Flat JSON envelope (default field names)
//! let cfg = EnvelopeConfig::default().with_mode(EnvelopeMode::Json);
//! assert_eq!(cfg.mode, EnvelopeMode::Json);
//! assert_eq!(cfg.layout, EnvelopeLayout::Flat);
//! ```

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Whether output is wrapped in a JSON envelope.
///
/// # Examples
///
/// ```
/// use scriba::envelope::EnvelopeMode;
///
/// let mode = EnvelopeMode::Json;
/// assert!(mode.is_json());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnvelopeMode {
    /// No envelope — raw rendered output only (default).
    #[default]
    None,
    /// Wrap output in a JSON object with metadata fields.
    Json,
}

impl EnvelopeMode {
    /// Returns `true` if the mode is `Json`.
    pub fn is_json(self) -> bool {
        matches!(self, Self::Json)
    }
}

/// Layout style for JSON envelope fields.
///
/// Controls where `ok`, `format`, and user metadata appear relative to content.
///
/// # Examples
///
/// ```
/// use scriba::envelope::EnvelopeLayout;
///
/// // Flat: {"ok": true, "format": "json", "content": {...}, "meta": {...}}
/// let flat = EnvelopeLayout::Flat;
///
/// // Nested: {"meta": {"ok": true, "format": "json", ...}, "data": {...}}
/// let nested = EnvelopeLayout::Nested;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnvelopeLayout {
    /// All fields at the top level.
    ///
    /// `{"ok": true, "format": "json", "content": {...}, "meta": {...}}`
    #[default]
    Flat,
    /// `ok`, `format`, and metadata are nested under the `meta_field`.
    ///
    /// `{"meta": {"ok": true, "format": "json", ...}, "data": {...}}`
    Nested,
}

/// Customisable field names used in the JSON envelope.
///
/// Override any field name to match your API conventions.
///
/// # Examples
///
/// ```
/// use scriba::envelope::EnvelopeFields;
///
/// let fields = EnvelopeFields {
///     ok_field: "success".into(),
///     format_field: "type".into(),
///     content_field: "result".into(),
///     meta_field: "context".into(),
/// };
/// assert_eq!(fields.ok_field, "success");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvelopeFields {
    /// Field name for the success flag (default: `"ok"`).
    pub ok_field: String,
    /// Field name for the format name (default: `"format"`).
    pub format_field: String,
    /// Field name for the content payload (default: `"content"` for Flat, `"data"` for Nested).
    pub content_field: String,
    /// Field name for the metadata object (default: `"meta"`).
    pub meta_field: String,
}

impl Default for EnvelopeFields {
    fn default() -> Self {
        Self {
            ok_field: "ok".into(),
            format_field: "format".into(),
            content_field: "content".into(),
            meta_field: "meta".into(),
        }
    }
}

/// Optional metadata to include inside a JSON envelope.
///
/// All fields are optional and serialised only when `Some`. Add arbitrary
/// additional key-value pairs via `extra`.
///
/// # Examples
///
/// ```
/// use scriba::envelope::Meta;
///
/// let meta = Meta::default()
///     .with_dry_run(true)
///     .with_command("deploy".into())
///     .with_extra("region", "eu-west-1");
///
/// assert_eq!(meta.dry_run, Some(true));
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meta {
    /// Whether the operation was a dry run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The command that produced this output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Execution duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// RFC 3339 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Logical scope or context label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Library/CLI version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Additional arbitrary key-value pairs.
    #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, Value>,
}

impl Meta {
    /// Set the `dry_run` flag.
    pub fn with_dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    /// Set the `command` field.
    pub fn with_command(mut self, value: String) -> Self {
        self.command = Some(value);
        self
    }

    /// Set the `duration_ms` field.
    pub fn with_duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = Some(ms);
        self
    }

    /// Set the `timestamp` field.
    pub fn with_timestamp(mut self, ts: String) -> Self {
        self.timestamp = Some(ts);
        self
    }

    /// Set the `scope` field.
    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    /// Set the `version` field.
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Add an arbitrary extra key-value pair.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::envelope::Meta;
    ///
    /// let meta = Meta::default().with_extra("region", "eu-west-1");
    /// assert_eq!(meta.extra.get("region").unwrap(), "eu-west-1");
    /// ```
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let v = serde_json::to_value(value).unwrap_or(Value::Null);
        self.extra.insert(key.into(), v);
        self
    }

    /// Add multiple extra key-value pairs from any iterator of `(key, value)` tuples.
    ///
    /// Useful for inserting a batch of arbitrary metadata without chaining
    /// multiple `with_extra()` calls.
    ///
    /// # Example
    ///
    /// ```
    /// use scriba::envelope::Meta;
    /// use std::collections::BTreeMap;
    ///
    /// let mut extras = BTreeMap::new();
    /// extras.insert("region".to_string(), serde_json::json!("eu-west-1"));
    /// extras.insert("actor".to_string(), serde_json::json!("ci-bot"));
    ///
    /// let meta = Meta::default().with_extra_map(extras);
    /// assert_eq!(meta.extra.get("region").unwrap(), "eu-west-1");
    /// assert_eq!(meta.extra.get("actor").unwrap(), "ci-bot");
    /// ```
    pub fn with_extra_map(
        mut self,
        map: impl IntoIterator<Item = (String, Value)>,
    ) -> Self {
        self.extra.extend(map);
        self
    }

    /// Returns `true` if all fields are `None` and `extra` is empty.
    pub fn is_empty(&self) -> bool {
        self.dry_run.is_none()
            && self.command.is_none()
            && self.duration_ms.is_none()
            && self.timestamp.is_none()
            && self.scope.is_none()
            && self.version.is_none()
            && self.extra.is_empty()
    }
}

/// Full configuration for JSON envelope wrapping.
///
/// When `mode` is `EnvelopeMode::None` (the default) nothing changes. Set
/// `mode` to `EnvelopeMode::Json` to wrap rendered output in a JSON object.
///
/// # Examples
///
/// ```
/// use scriba::envelope::{EnvelopeConfig, EnvelopeMode, EnvelopeLayout, EnvelopeFields};
///
/// let cfg = EnvelopeConfig::default()
///     .with_mode(EnvelopeMode::Json)
///     .with_layout(EnvelopeLayout::Nested)
///     .with_show_ok(true)
///     .with_show_format(true);
///
/// assert!(cfg.mode.is_json());
/// assert_eq!(cfg.layout, EnvelopeLayout::Nested);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvelopeConfig {
    /// Whether and how to wrap output.
    pub mode: EnvelopeMode,
    /// Whether fields are at the top level or nested under `meta_field`.
    pub layout: EnvelopeLayout,
    /// Customisable field names.
    pub fields: EnvelopeFields,
    /// Include the `ok` boolean field in the envelope (default: `true`).
    pub show_ok: bool,
    /// Include the `format` string field in the envelope (default: `true`).
    pub show_format: bool,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        Self {
            mode: EnvelopeMode::default(),
            layout: EnvelopeLayout::default(),
            fields: EnvelopeFields::default(),
            show_ok: true,
            show_format: true,
        }
    }
}

impl EnvelopeConfig {
    /// Set the envelope mode.
    pub fn with_mode(mut self, mode: EnvelopeMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the layout style.
    pub fn with_layout(mut self, layout: EnvelopeLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set custom field names.
    pub fn with_fields(mut self, fields: EnvelopeFields) -> Self {
        self.fields = fields;
        self
    }

    /// Control whether the `ok` field is included.
    pub fn with_show_ok(mut self, show: bool) -> Self {
        self.show_ok = show;
        self
    }

    /// Control whether the `format` field is included.
    pub fn with_show_format(mut self, show: bool) -> Self {
        self.show_format = show;
        self
    }
}

/// Wrap a rendered content value in a JSON envelope according to config.
///
/// - `ok` — success flag
/// - `format` — format name (e.g. `"json"`)
/// - `content_field` — the rendered payload
/// - optional `meta` object
///
/// Layout controls whether `ok`/`format` appear at the top level (`Flat`) or
/// nested inside the `meta_field` object (`Nested`).
pub fn wrap(
    cfg: &EnvelopeConfig,
    format_name: &str,
    content: Value,
    meta: Option<&Meta>,
    ok: bool,
) -> Value {
    match cfg.layout {
        EnvelopeLayout::Flat => wrap_flat(cfg, format_name, content, meta, ok),
        EnvelopeLayout::Nested => wrap_nested(cfg, format_name, content, meta, ok),
    }
}

fn wrap_flat(
    cfg: &EnvelopeConfig,
    format_name: &str,
    content: Value,
    meta: Option<&Meta>,
    ok: bool,
) -> Value {
    let mut map = serde_json::Map::new();

    if cfg.show_ok {
        map.insert(cfg.fields.ok_field.clone(), Value::Bool(ok));
    }
    if cfg.show_format {
        map.insert(
            cfg.fields.format_field.clone(),
            Value::String(format_name.to_string()),
        );
    }

    let content_key = cfg.fields.content_field.clone();
    map.insert(content_key, content);

    if let Some(meta) = meta {
        if !meta.is_empty() {
            if let Ok(meta_val) = serde_json::to_value(meta) {
                map.insert(cfg.fields.meta_field.clone(), meta_val);
            }
        }
    }

    Value::Object(map)
}

fn wrap_nested(
    cfg: &EnvelopeConfig,
    format_name: &str,
    content: Value,
    meta: Option<&Meta>,
    ok: bool,
) -> Value {
    let mut meta_obj = serde_json::Map::new();

    if cfg.show_ok {
        meta_obj.insert(cfg.fields.ok_field.clone(), Value::Bool(ok));
    }
    if cfg.show_format {
        meta_obj.insert(
            cfg.fields.format_field.clone(),
            Value::String(format_name.to_string()),
        );
    }

    // Merge user meta fields into the nested meta object
    if let Some(meta) = meta {
        if !meta.is_empty() {
            if let Ok(Value::Object(extra)) = serde_json::to_value(meta) {
                meta_obj.extend(extra);
            }
        }
    }

    let content_key = cfg.fields.content_field.clone();

    let mut map = serde_json::Map::new();
    map.insert(cfg.fields.meta_field.clone(), Value::Object(meta_obj));
    map.insert(content_key, content);

    Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- EnvelopeMode ---

    #[test]
    fn envelope_mode_default_is_none() {
        assert_eq!(EnvelopeMode::default(), EnvelopeMode::None);
    }

    #[test]
    fn envelope_mode_is_json() {
        assert!(EnvelopeMode::Json.is_json());
        assert!(!EnvelopeMode::None.is_json());
    }

    // --- EnvelopeLayout ---

    #[test]
    fn envelope_layout_default_is_flat() {
        assert_eq!(EnvelopeLayout::default(), EnvelopeLayout::Flat);
    }

    // --- EnvelopeFields ---

    #[test]
    fn envelope_fields_default_names() {
        let f = EnvelopeFields::default();
        assert_eq!(f.ok_field, "ok");
        assert_eq!(f.format_field, "format");
        assert_eq!(f.content_field, "content");
        assert_eq!(f.meta_field, "meta");
    }

    #[test]
    fn envelope_fields_custom() {
        let f = EnvelopeFields {
            ok_field: "success".into(),
            format_field: "type".into(),
            content_field: "result".into(),
            meta_field: "context".into(),
        };
        assert_eq!(f.ok_field, "success");
        assert_eq!(f.content_field, "result");
    }

    // --- Meta ---

    #[test]
    fn meta_default_is_empty() {
        assert!(Meta::default().is_empty());
    }

    #[test]
    fn meta_with_dry_run() {
        let m = Meta::default().with_dry_run(true);
        assert_eq!(m.dry_run, Some(true));
        assert!(!m.is_empty());
    }

    #[test]
    fn meta_with_command() {
        let m = Meta::default().with_command("deploy".into());
        assert_eq!(m.command, Some("deploy".into()));
    }

    #[test]
    fn meta_with_extra() {
        let m = Meta::default().with_extra("region", "eu-west-1");
        assert_eq!(m.extra.get("region").unwrap(), "eu-west-1");
    }

    #[test]
    fn meta_with_extra_map_inserts_all_entries() {
        let map = [
            ("region".to_string(), serde_json::json!("eu-west-1")),
            ("actor".to_string(), serde_json::json!("ci-bot")),
            ("run_id".to_string(), serde_json::json!(42u64)),
        ];
        let m = Meta::default().with_extra_map(map);
        assert_eq!(m.extra.get("region").unwrap(), "eu-west-1");
        assert_eq!(m.extra.get("actor").unwrap(), "ci-bot");
        assert_eq!(m.extra.get("run_id").unwrap(), 42u64);
        assert!(!m.is_empty());
    }

    #[test]
    fn meta_with_extra_map_empty_iterator() {
        let m = Meta::default().with_extra_map(std::iter::empty());
        assert!(m.extra.is_empty());
    }

    #[test]
    fn meta_with_extra_map_merges_with_existing_extra() {
        let m = Meta::default()
            .with_extra("a", "first")
            .with_extra_map([("b".to_string(), serde_json::json!("second"))]);
        assert_eq!(m.extra.get("a").unwrap(), "first");
        assert_eq!(m.extra.get("b").unwrap(), "second");
    }

    #[test]
    fn meta_builder_is_fluent() {
        let m = Meta::default()
            .with_dry_run(true)
            .with_command("deploy".into())
            .with_duration_ms(120)
            .with_timestamp("2026-04-13T00:00:00Z".into())
            .with_scope("prod".into())
            .with_version("0.3.0".into())
            .with_extra("region", "eu-west-1");
        assert_eq!(m.dry_run, Some(true));
        assert_eq!(m.duration_ms, Some(120));
        assert_eq!(m.scope, Some("prod".into()));
        assert!(!m.is_empty());
    }

    // --- EnvelopeConfig ---

    #[test]
    fn envelope_config_default() {
        let cfg = EnvelopeConfig::default();
        assert_eq!(cfg.mode, EnvelopeMode::None);
        assert_eq!(cfg.layout, EnvelopeLayout::Flat);
        assert!(cfg.show_ok);
        assert!(cfg.show_format);
    }

    #[test]
    fn envelope_config_builder() {
        let cfg = EnvelopeConfig::default()
            .with_mode(EnvelopeMode::Json)
            .with_layout(EnvelopeLayout::Nested)
            .with_show_ok(false)
            .with_show_format(false);
        assert!(cfg.mode.is_json());
        assert_eq!(cfg.layout, EnvelopeLayout::Nested);
        assert!(!cfg.show_ok);
        assert!(!cfg.show_format);
    }

    // --- wrap() flat ---

    fn default_cfg_json() -> EnvelopeConfig {
        EnvelopeConfig::default().with_mode(EnvelopeMode::Json)
    }

    #[test]
    fn wrap_flat_has_ok_and_format() {
        let cfg = default_cfg_json();
        let content = serde_json::json!({"title": "test"});
        let result = wrap(&cfg, "json", content, None, true);
        assert_eq!(result["ok"], true);
        assert_eq!(result["format"], "json");
        assert!(result["content"].is_object());
        assert!(result.get("meta").is_none());
    }

    #[test]
    fn wrap_flat_with_meta() {
        let cfg = default_cfg_json();
        let meta = Meta::default().with_dry_run(true);
        let result = wrap(&cfg, "json", serde_json::json!({}), Some(&meta), true);
        assert_eq!(result["meta"]["dry_run"], true);
    }

    #[test]
    fn wrap_flat_meta_omitted_when_empty() {
        let cfg = default_cfg_json();
        let meta = Meta::default();
        let result = wrap(&cfg, "json", serde_json::json!({}), Some(&meta), true);
        assert!(result.get("meta").is_none());
    }

    #[test]
    fn wrap_flat_hide_ok_and_format() {
        let cfg = default_cfg_json()
            .with_show_ok(false)
            .with_show_format(false);
        let result = wrap(&cfg, "json", serde_json::json!({}), None, true);
        assert!(result.get("ok").is_none());
        assert!(result.get("format").is_none());
        assert!(result.get("content").is_some());
    }

    #[test]
    fn wrap_flat_custom_field_names() {
        let cfg = default_cfg_json().with_fields(EnvelopeFields {
            ok_field: "success".into(),
            format_field: "type".into(),
            content_field: "result".into(),
            meta_field: "context".into(),
        });
        let result = wrap(&cfg, "json", serde_json::json!({}), None, true);
        assert_eq!(result["success"], true);
        assert_eq!(result["type"], "json");
        assert!(result.get("result").is_some());
    }

    // --- wrap() nested ---

    #[test]
    fn wrap_nested_puts_ok_format_under_meta() {
        let cfg = default_cfg_json().with_layout(EnvelopeLayout::Nested);
        let result = wrap(&cfg, "json", serde_json::json!({"x": 1}), None, true);
        assert_eq!(result["meta"]["ok"], true);
        assert_eq!(result["meta"]["format"], "json");
        assert!(result["content"].is_object());
        assert!(result.get("ok").is_none());
    }

    #[test]
    fn wrap_nested_merges_user_meta_into_meta_object() {
        let cfg = default_cfg_json().with_layout(EnvelopeLayout::Nested);
        let meta = Meta::default()
            .with_dry_run(true)
            .with_timestamp("2026-04-13T00:00:00Z".into());
        let result = wrap(&cfg, "json", serde_json::json!({}), Some(&meta), true);
        assert_eq!(result["meta"]["dry_run"], true);
        assert_eq!(result["meta"]["timestamp"], "2026-04-13T00:00:00Z");
        assert_eq!(result["meta"]["ok"], true);
    }

    #[test]
    fn wrap_nested_custom_field_names() {
        let cfg = default_cfg_json()
            .with_layout(EnvelopeLayout::Nested)
            .with_fields(EnvelopeFields {
                ok_field: "success".into(),
                format_field: "type".into(),
                content_field: "data".into(),
                meta_field: "header".into(),
            });
        let result = wrap(&cfg, "text", serde_json::json!({"x": 1}), None, false);
        assert_eq!(result["header"]["success"], false);
        assert_eq!(result["header"]["type"], "text");
        assert!(result.get("data").is_some());
    }

    #[test]
    fn wrap_nested_with_extra_in_meta() {
        let cfg = default_cfg_json().with_layout(EnvelopeLayout::Nested);
        let meta = Meta::default().with_extra("region", "eu-west-1");
        let result = wrap(&cfg, "json", serde_json::json!({}), Some(&meta), true);
        assert_eq!(result["meta"]["region"], "eu-west-1");
    }
}
