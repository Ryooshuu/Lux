use std::sync::Mutex;

use gtk::traits::WidgetExt;
use specta_typescript::Typescript;
use tauri::{Emitter, Manager, State};
use tauri_specta::{collect_commands, Builder as SpectaBuilder};

use crate::boot::layer_shell::ShellState;

mod boot;
mod commands;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    let commands_builder = SpectaBuilder::<tauri::Wry>::new().commands(collect_commands![
        commands::utils::fix_transparency,
        commands::utils::exit_app,
        commands::utils::toggle_visibility,
    ]);

    #[cfg(debug_assertions)]
    commands_builder
        .export(
            Typescript::default(),
            "../../../packages/tauri-types/src/bindings.ts",
        )
        .expect("failed to export commands");

    builder = builder.plugin(boot::logger::init());

    builder = builder
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let state: State<'_, Mutex<ShellState>> = app.state();
            let state = state.lock();

            if state.is_ok() {
                let window = state.unwrap().gtk_window.clone();
                window.show();
                window.present();

                app.emit("app_shown", ()).unwrap();
            }
        }));
    use gtk::traits::GtkWindowExt;

    builder = builder
        .invoke_handler(commands_builder.invoke_handler())
        .setup(|app| {
            boot::tray::create_tray(app.handle())?;

            Ok(())
        });

    builder
        .plugin(boot::layer_shell::init())
        .run(context)
        .expect("error while running tauri application");
}
