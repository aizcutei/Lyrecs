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
/* use tauri::Manager;
 */
use tauri_plugin_store::{PluginBuilder, StoreBuilder};
use env_logger;

fn main() {
//env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let setting_data = StoreBuilder::new(".settings".parse().unwrap())
        .default("Test-Item".to_string(), "Test-Value".into())
        .build();

    let app = tauri::Builder::default()
        // Blur effect
        .setup(app::window::vibrancy_effect)
        .plugin(PluginBuilder::default().stores([setting_data]).freeze().build())
        .system_tray(app::tray::tray_icon())
        .on_system_tray_event(app::tray::tray_handler)
        .invoke_handler(tauri::generate_handler![
            connect::connect_test,
            ])
        .setup(app::window::shadow_effect) // Shadow effect
        .run(tauri::generate_context!())
        .expect("Error while running tauri application in main");



}
