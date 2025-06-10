use std::sync::Mutex;
use tauri::State;

use crate::boot::layer_shell::ShellState;
use gtk::traits::{GtkWindowExt, WidgetExt};

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

#[tauri::command]
#[specta::specta]
pub fn toggle_visibility(state: State<'_, Mutex<ShellState>>, visible_state: bool) {
    let shell_state = state.lock();

    if shell_state.is_ok() {
        let window = shell_state.unwrap().gtk_window.clone();

        if window.is_visible() {
            if !visible_state {
                window.hide();
            }
        } else {
            if visible_state {
                window.show();
                window.present();
            }
        }
    }
}
