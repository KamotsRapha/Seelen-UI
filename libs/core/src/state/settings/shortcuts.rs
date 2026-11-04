use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Widget;

/// Declaration for a system-level shortcut (not attached to any specific widget definition).
/// Hardcoded in Rust; exposed to the frontend via the `StateGetSystemShortcuts` command.
/// The user can override keys via `SluShortcutsSettings.shortcuts`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), ts(export))]
pub struct SystemShortcutDeclaration {
    pub id: String,
    pub command: Vec<String>,
    pub label: String,
    pub default_keys: Vec<String>,
    /// If true, user cannot change the keys for this shortcut.
    pub readonly: bool,
}

/// Minimal struct sent to the service after the background has resolved all shortcut overrides.
/// Contains only what the service needs to register hotkeys — no widget info, no enums.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), derive(ts_rs::TS))]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), ts(export))]
pub struct ResolvedShortcut {
    pub command: Vec<String>,
    pub keys: Vec<String>,
}

/// User-facing shortcut settings.
/// - `enabled`: global on/off toggle.
/// - `shortcuts`: key overrides for **system-level** shortcut declarations (`id -> keys`).
///   Widget shortcut overrides live inside each widget's `$shortcuts` in `SettingsByWidget`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), derive(ts_rs::TS))]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), ts(export))]
pub struct SluShortcutsSettings {
    pub enabled: bool,
    pub shortcuts: HashMap<String, Vec<String>>,
}

impl Default for SluShortcutsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            shortcuts: HashMap::new(),
        }
    }
}
