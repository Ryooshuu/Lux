use tauri_plugin_log::{Target as LogTarget, TargetKind};

pub fn get_log_targets() -> Vec<LogTarget> {
    #[cfg(debug_assertions)]
    let log_targets = vec![
        LogTarget::new(TargetKind::Stdout),
        LogTarget::new(TargetKind::LogDir {
            file_name: Some(format!(
                "dev:lux-{}",
                chrono::Local::now().format("%Y-%m-%d")
            )),
        }),
        LogTarget::new(TargetKind::Webview),
    ];
    #[cfg(not(debug_assertions))]
    let log_targets = vec![
        LogTarget::new(TargetKind::Stdout),
        LogTarget::new(TargetKind::LogDir {
            file_name: Some(format!(
                "dev:lux-{}",
                chrono::Local::now().format("%Y-%m-%d")
            )),
        }),
    ];

    log_targets
}

pub fn get_log_level() -> log::LevelFilter {
    #[cfg(debug_assertions)]
    return log::LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    return log::LevelFilter::Info;
}
