use lazy_static::lazy_static;
use log::info;
use tokio::sync::{Mutex, MutexGuard};

use crate::{api::model::Lrcx, player_info::link_system::get_player_info};

lazy_static! {
    static ref CACHE_MANAGER: CacheManager = CacheManager::new();
}

pub fn get_cache_manager() -> &'static CacheManager {
    &CACHE_MANAGER
}

pub struct CacheManager {
    fresh: Mutex<bool>, // 是否过期
    cache: Mutex<Lrcx>,
}

impl CacheManager {
    fn new() -> Self {
        CacheManager {
            fresh: Mutex::new(false),
            cache: Mutex::new(Default::default()),
        }
    }

    pub async fn get_cache(&self) -> MutexGuard<'_, Lrcx> {
        self.cache.lock().await
    }

    pub async fn set_cache(&self, lrc: Lrcx) {
        let mut l = self.cache.lock().await;
        let mut f = self.fresh.lock().await;
        *l = lrc;
        *f = true;
    }

    pub async fn is_fresh(&self) -> bool {
        *self.fresh.lock().await
    }

    pub async fn set_fresh(&mut self, fresh: bool) {
        let mut f = self.fresh.lock().await;
        *f = fresh;
    }

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
                let mut f = self.fresh.lock().await;
                *f = false;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }
}
