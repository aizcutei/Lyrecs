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
mod api;
use std::env;
use api::{connect, lyric_line};


use tauri_plugin_store::{PluginBuilder, StoreBuilder};

fn main() {

    env_logger::init();



    let _app = tauri::Builder::default()
        .setup(app::window::shadow_effect) // Shadow effect
        .setup(app::window::vibrancy_effect)// Blur effect
        .plugin(PluginBuilder::default().stores([app::setting::init_setting()]).freeze().build())
        .system_tray(app::tray::tray_icon())
        .on_system_tray_event(app::tray::tray_handler)
        .invoke_handler(tauri::generate_handler![
            connect::connect_test,
            lyric_line::get_next_inline_lyrics,
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application in main");

}
