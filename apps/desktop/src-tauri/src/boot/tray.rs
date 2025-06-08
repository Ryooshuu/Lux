use std::sync::Mutex;

use gtk::traits::{GtkWindowExt, WidgetExt};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, Runtime, State,
};

use crate::boot::layer_shell::ShellState;

fn toggle_main_window<R: Runtime>(app: &tauri::AppHandle<R>) {
    let state: State<'_, Mutex<ShellState>> = app.state();
    let shell_state = state.lock();

    if shell_state.is_ok() {
        let window = shell_state.unwrap().gtk_window.clone();

        if window.is_visible() {
            window.hide();
        } else {
            window.show();
            window.present();
        }
    }
}

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let toggle_i = MenuItem::with_id(app, "toggle", "Toggle", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu1 = Menu::with_items(app, &[&toggle_i, &quit_i])?;

    let _ = TrayIconBuilder::with_id("tray-1")
        .tooltip("Lux")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu1)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => toggle_main_window(app),
            "quit" => app.exit(0),
            _ => {
                log::warn!("unknown menu event: {:?}", event.id);
            }
        })
        .build(app);

    Ok(())
}
