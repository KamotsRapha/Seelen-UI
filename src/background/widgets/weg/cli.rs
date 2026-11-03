pub use slu_ipc::commands::WegCli;
use slu_ipc::commands::WegCommand;

use seelen_core::{
    state::{WegItem, WegItemData},
    system_state::UserAppWindow,
};
use windows::Win32::UI::WindowsAndMessaging::SW_MINIMIZE;

use crate::{
    error::Result,
    modules::apps::application::USER_APPS_MANAGER,
    state::application::WEG_ITEMS_MANAGER,
    windows_api::{WindowsApi, window::Window},
};

/// Mirrors `getWindowsForItem` from the frontend (`windows.ts`).
///
/// Prefer exact UMID matches, but fall back to the executable path when the window advertises
/// a different UMID than the pinned item. Some apps (including desktop apps like WeChat) use
/// different UMIDs on their shortcut and window, even though both belong to the same executable.
///
/// note: on update of this function check src\\ui\\svelte\\weg\\state\\windows.svelte.ts too.
fn get_windows_for_item<'a>(
    item: &WegItemData,
    interactables: &'a [UserAppWindow],
) -> Vec<&'a UserAppWindow> {
    let item_command = item.relaunch.as_ref().map(|r| r.command.to_lowercase());
    let item_path = item.path.to_string_lossy().to_lowercase();

    interactables
        .iter()
        .filter(|w| {
            if item.umid.is_some() && item.umid == w.umid {
                return true;
            }

            let win_path = w
                .process
                .path
                .as_ref()
                .map(|p| p.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            if win_path.is_empty() {
                return false;
            }

            item_command.as_deref() == Some(win_path.as_str()) || item_path == win_path
        })
        .collect()
}

pub fn process(cmd: WegCli) -> Result<()> {
    #[allow(irrefutable_let_patterns)]
    if let WegCommand::ForegroundOrRunApp { index } = cmd.subcommand {
        let weg_items = WEG_ITEMS_MANAGER.get();

        let all_items: Vec<&WegItem> = weg_items
            .left
            .iter()
            .chain(weg_items.center.iter())
            .chain(weg_items.right.iter())
            .filter(|item| matches!(item, WegItem::AppOrFile(_)))
            .collect();

        if all_items.len() <= index {
            return Ok(());
        }

        let WegItem::AppOrFile(inner_data) = all_items[index] else {
            return Ok(());
        };

        let interactables = USER_APPS_MANAGER.interactable_windows.to_vec();
        let windows = get_windows_for_item(inner_data, &interactables);

        if windows.is_empty() {
            let command = inner_data
                .relaunch
                .as_ref()
                .map(|r| r.command.clone())
                .unwrap_or_else(|| inner_data.path.to_string_lossy().to_string());
            let args = inner_data
                .relaunch
                .as_ref()
                .and_then(|r| r.args.as_ref())
                .map(|a| a.to_string());
            let working_dir = inner_data
                .relaunch
                .as_ref()
                .and_then(|r| r.working_dir.clone());
            WindowsApi::execute(command, args, working_dir, false)?;
        } else {
            let focused = windows.iter().find(|w| Window::from(w.hwnd).is_focused());
            if let Some(w) = focused {
                Window::from(w.hwnd).show_window_async(SW_MINIMIZE)?;
            } else if let Some(w) = windows.first() {
                let window = Window::from(w.hwnd);
                if window.is_window() {
                    window.unminimize()?;
                    window.focus()?;
                }
            }
        }
    }
    Ok(())
}
