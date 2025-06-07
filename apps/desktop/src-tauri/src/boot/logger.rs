use tauri::{plugin::TauriPlugin, Runtime};
pub use tauri_plugin_log::fern::colors::ColoredLevelConfig;

use crate::utils;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::new()
        .targets(utils::log::get_log_targets())
        .level(utils::log::get_log_level())
        .filter(|metadata| !metadata.target().starts_with("mdns_sd"))
        .with_colors(ColoredLevelConfig::default())
        .max_file_size(10_000_000) // 10 MB
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .build()
}
