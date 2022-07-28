#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod player_info;
mod get_lyrics;
mod parse_lyric;
mod tauri_command;
use std::env;

use player_info::link_system;
use get_lyrics::netease;

/* use tauri::Manager;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial}; */
use tauri::Manager;
use window_shadows::set_shadow;
use env_logger;

fn main() {
  //env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();
  tauri::Builder::default()
    // Blur effect
    /* .setup(|app| {
      let win = app.get_window("main").unwrap();

      #[cfg(target_os = "macos")]
      apply_vibrancy(&win, NSVisualEffectMaterial::AppearanceBased)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

      #[cfg(target_os = "windows")]
      apply_blur(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

      Ok(())
    }) */
    .invoke_handler(tauri::generate_handler![
      tauri_command::connect_test,
      tauri_command::get_next_inline_lyric
      ])
    .setup(|app| {
      let win = app.get_window("main").unwrap();
      set_shadow(&win, false).expect("Unsupported platform!");
      Ok(())
    }) // Shadow effect
    .run(tauri::generate_context!())
    .expect("Error while running tauri application in main");
}
