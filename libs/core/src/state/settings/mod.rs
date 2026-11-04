#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), derive(ts_rs::TS))]
#[cfg_attr(all(feature = "gen-binds", not(feature = "salvo")), ts(repr(enum = name)))]
#[serde(rename_all = "camelCase")]
pub enum GameModeSuspendBehavior {
    SuspendAll,
    KeepTaskSwitcher,
    PauseAltTabCapture,
}

impl Default for GameModeSuspendBehavior {
    fn default() -> Self {
        Self::KeepTaskSwitcher
    }
}
