use tauri_runtime::window::WindowId;
use tauri_runtime::UserEvent;

use tauri_runtime_wry::tao::window::WindowId as TaoWindowId;
use tauri_runtime_wry::EventLoopIterationContext;

/// Gets the WindowId from its TaoWindowId
/// - this has needed tweaks in `tauri-runtime-wry`
pub(crate) fn get_id_from_tao_window_id<T: UserEvent>(
    tao_win_id: &TaoWindowId,
    context: &EventLoopIterationContext<'_, T>,
) -> Option<WindowId> {
    context.window_id_map.get(tao_win_id)
}

/// Gets the label of a Tauri window from its TaoWindowId
/// - this has needed tweaks in `tauri-runtime-wry`
/// - TODO: is there an easier way to do this?
pub(crate) fn get_label_from_tao_window_id<T: UserEvent>(
    tao_win_id: &TaoWindowId,
    context: &EventLoopIterationContext<'_, T>,
) -> Option<String> {
    if let Some(id) = get_id_from_tao_window_id(tao_win_id, context) {
        if let Some(window_wrapper) = context.windows.0.borrow().get(&id) {
            let label = window_wrapper.label.clone();
            return Some(label);
        }
    }

    None
}
