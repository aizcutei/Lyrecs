#[tauri::command]
pub fn connect_test(text: &str) -> String {
    format!("Hello {}!", text)
}
