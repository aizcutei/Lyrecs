use crate::cache::CacheManager;
use lazy_static::lazy_static;
use log::info;

use crate::{api::model::Lrcx, player_info::link_system::get_player_info};

lazy_static! {
    static ref CACHE_MANAGER: CacheManager<Lrcx> = CacheManager::new();
}

pub fn get_cache_manager() -> &'static CacheManager<Lrcx> {
    &CACHE_MANAGER
}

impl CacheManager<Lrcx> {
    #[cfg(target_os = "macos")]
    pub async fn update(&self) {
        use std::{ffi::{ c_void}};
        unsafe{
            extern "C" fn callback(_:c_void) {
                tauri::async_runtime::block_on(async {
                    get_cache_manager().set_fresh(false).await});
            }

            extern "C" {
                fn register_playstate_change_callback(callback: extern "C" fn(_:c_void));
            }
            register_playstate_change_callback(callback);
    }
    }
    #[cfg(target_os = "windows")]
    pub async fn update(&self) -> ! {
        let mut current_song: String = Default::default();
        loop {
            let player_info = get_player_info().await;
            if player_info.is_err() {
                continue;
            }
            let player_info = player_info.unwrap();
            let song = player_info.track.title.clone() + &player_info.track.artist.clone();
            if song != current_song {
                info!("cache out of date!");
                current_song = song;
                self.set_fresh(false).await;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }
}
