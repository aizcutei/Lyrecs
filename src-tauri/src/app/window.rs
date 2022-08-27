use tauri::{App, Manager};
use window_shadows::set_shadow;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

pub fn shadow_effect(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").unwrap();

    set_shadow(&main_window, false).expect("Unsupported platform!");
    Ok(())
}

pub fn vibrancy_effect(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    apply_vibrancy(&main_window, NSVisualEffectMaterial::FullScreenUI, 1, 10.0)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    apply_blur(&main_window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}
