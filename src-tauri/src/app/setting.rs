use tauri::{App, AppHandle};
use tauri_plugin_store::{StoreBuilder, Store};

pub fn init_setting() -> Store {
    StoreBuilder::new(".settings".parse().unwrap())
    //.default("Test-Item".to_string(), "Test-Value".into())
    .build()
}
