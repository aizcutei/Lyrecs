#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]


#[macro_use]
extern crate lazy_static;
mod app;
mod player_info;
mod get_lyrics;
mod parse_lyric;
mod tauri_command;
mod api;
use std::env;
use api::connect;

use get_lyrics::cache::get_cache_manager;
use tauri_plugin_store::{PluginBuilder, StoreBuilder};

fn main() {

    env_logger::init();

    let setting_data = StoreBuilder::new(".settings".parse().unwrap())
        .default("Test-Item".to_string(), "Test-Value".into())
        .build();

    let _lyric_cache = tokio::spawn(get_cache_manager().update());

    let _app = tauri::Builder::default()
        .setup(app::window::shadow_effect) // Shadow effect
        .setup(app::window::vibrancy_effect)// Blur effect
        .plugin(PluginBuilder::default().stores([setting_data]).freeze().build())
        .system_tray(app::tray::tray_icon())
        .on_system_tray_event(app::tray::tray_handler)
        .invoke_handler(tauri::generate_handler![
            connect::connect_test,
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application in main");

}
