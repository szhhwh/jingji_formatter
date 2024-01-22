// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use jingji_formatter::app_root;
use tauri_plugin_log::{
    fern::colors::{Color, ColoredLevelConfig},
    LogTarget,
};

mod app;
use app::{cmd, setup};

fn main() {
    let mut log = tauri_plugin_log::Builder::default()
        .targets([
            LogTarget::Stdout,
            LogTarget::Webview,
            LogTarget::Folder(app_root()),
        ])
        .level(log::LevelFilter::Trace);

    if cfg!(debug_assertions) {
        log = log.with_colors(ColoredLevelConfig {
            error: Color::Red,
            warn: Color::Yellow,
            debug: Color::Blue,
            info: Color::BrightGreen,
            trace: Color::Cyan,
        });
    }
    tauri::Builder::default()
        .plugin(log.build())
        .invoke_handler(tauri::generate_handler![cmd::format])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
