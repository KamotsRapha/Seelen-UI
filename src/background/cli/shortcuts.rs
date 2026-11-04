use std::sync::atomic::{AtomicBool, Ordering};

use seelen_core::{
    handlers::SeelenEvent,
    state::{
        GameModeSuspendBehavior,
        shortcuts::{ResolvedShortcut, resolve_shortcuts, system_shortcut_declarations},
    },
};
use slu_ipc::messages::SvcAction;

use crate::{
    app::emit_to_webviews, cli::ServicePipe, error::Result, resources::RESOURCES,
    state::application::FULL_STATE,
};

pub static SHORTCUTS_PAUSED: AtomicBool = AtomicBool::new(false);

pub fn toggle_pause() -> Result<()> {
    let was_paused = SHORTCUTS_PAUSED.fetch_xor(true, Ordering::SeqCst);
    let now_paused = !was_paused;
    if now_paused {
        send_pause_only()?;
    } else {
        send_all()?;
    }
    emit_to_webviews(SeelenEvent::ShortcutsPaused, now_paused);
    Ok(())
}

pub fn sync_shortcuts_for_game_mode(
    game_mode_active: bool,
    behavior: GameModeSuspendBehavior,
) -> Result<()> {
    if SHORTCUTS_PAUSED.load(Ordering::Acquire) {
        return Ok(());
    }

    let state = FULL_STATE.load();
    let widgets = RESOURCES.widgets();
    let widget_refs: Vec<_> = widgets.iter().map(|w| w.as_ref()).collect();
    let (resolved, _) = resolve_shortcuts(&state.settings, &widget_refs);

    let filtered = if game_mode_active && behavior == GameModeSuspendBehavior::PauseAltTabCapture {
        resolved
            .into_iter()
            .filter(|shortcut| {
                !(shortcut.command.len() >= 3
                    && shortcut.command[0] == "task-switcher"
                    && matches!(shortcut.command[1].as_str(), "select-next-task" | "select-previous-task"))
            })
            .collect()
    } else {
        resolved
    };

    ServicePipe::request(SvcAction::SetShortcuts(filtered))
}

fn send_all() -> Result<()> {
    let state = FULL_STATE.load();
    let widgets = RESOURCES.widgets();
    let widget_refs: Vec<_> = widgets.iter().map(|w| w.as_ref()).collect();
    let (resolved, _) = resolve_shortcuts(&state.settings, &widget_refs);
    ServicePipe::request(SvcAction::SetShortcuts(resolved))
}

fn send_pause_only() -> Result<()> {
    let state = FULL_STATE.load();
    let decls = system_shortcut_declarations();
    let shortcuts = decls
        .iter()
        .find(|d| d.id == "shortcuts-pause-toggle")
        .and_then(|d| {
            let keys = state
                .settings
                .shortcuts
                .shortcuts
                .get("shortcuts-pause-toggle")
                .cloned()
                .unwrap_or_else(|| d.default_keys.clone());
            if keys.is_empty() {
                None
            } else {
                Some(ResolvedShortcut {
                    command: d.command.clone(),
                    keys,
                })
            }
        })
        .into_iter()
        .collect();
    ServicePipe::request(SvcAction::SetShortcuts(shortcuts))
}
