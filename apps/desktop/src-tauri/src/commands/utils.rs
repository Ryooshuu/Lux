use gtk::traits::WidgetExt;
use std::sync::Mutex;
use tauri::State;

use crate::boot::layer_shell::ShellState;

#[tauri::command]
#[specta::specta]
pub fn fix_transparency(state: State<'_, Mutex<ShellState>>, dir: i32) {
    let window = state.lock().unwrap().gtk_window.clone();

    if dir == 1 {
        window.set_margin_bottom(0);
    } else {
        window.set_margin_bottom(1);
    }
}

#[tauri::command]
#[specta::specta]
pub fn exit_app(window: tauri::Window) {
    window.close().unwrap();
}
