#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;
mod api;
mod app;
mod cache;
mod config;
mod get_lyrics;
mod parse_lyric;
mod player_info;
use api::{connect, lyric_line};
use std::env;

use get_lyrics::cache::get_cache_manager;
use tauri_plugin_store::{PluginBuilder, StoreBuilder};

fn main() {
    env_logger::init();

    // let setting_data = StoreBuilder::new(".settings".parse().unwrap())
    //     .default("Test-Item".to_string(), "Test-Value".into())
    //     .build();
    tauri::async_runtime::spawn(get_cache_manager().update());
    tauri::async_runtime::spawn(config::init());

    tauri::Builder::default()
        .setup(app::window::shadow_effect) // Shadow effect
        .setup(app::window::vibrancy_effect) // Blur effect
        .plugin(
            PluginBuilder::default()
                .stores([app::setting::init_setting()])
                .freeze()
                .build(),
        )
        .system_tray(app::tray::tray_icon())
        .on_system_tray_event(app::tray::tray_handler)
        .invoke_handler(tauri::generate_handler![
            connect::connect_test,
            lyric_line::get_next_inline_lyrics,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application in main");
}
