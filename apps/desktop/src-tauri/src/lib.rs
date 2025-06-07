use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder as SpectaBuilder};

mod boot;
mod commands;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_log::Builder::new().build());

    let commands_builder = SpectaBuilder::<tauri::Wry>::new().commands(collect_commands![
        commands::utils::fix_transparency,
        commands::utils::exit_app
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
        .plugin(tauri_plugin_cli::init());

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
