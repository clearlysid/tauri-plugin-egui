use tauri_runtime::window::WindowId;
use tauri_runtime::UserEvent;

use tauri_runtime_wry::tao::window::WindowId as TaoWindowId;
use tauri_runtime_wry::EventLoopIterationContext;

/// Gets the WindowId from its TaoWindowId
pub(crate) fn get_id_from_tao_id<T: UserEvent>(
    tao_id: &TaoWindowId,
    context: &EventLoopIterationContext<'_, T>,
) -> Option<WindowId> {
    context.window_id_map.get(tao_id)
}

/// Gets the label of a Tauri window from its TaoWindowId
pub(crate) fn get_label_from_tao_id<T: UserEvent>(
    tao_id: &TaoWindowId,
    context: &EventLoopIterationContext<'_, T>,
) -> Option<String> {
    get_id_from_tao_id(tao_id, context)
        .and_then(|id| context.windows.0.borrow().get(&id).map(|ww| ww.label()))
}
