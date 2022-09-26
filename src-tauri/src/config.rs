use std::path::Path;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use log::{info, warn};
use notify::{Config, Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};

use crate::cache::CacheManager;

lazy_static! {
    static ref SETTINGS: CacheManager<Settings> = {
        Figment::new()
            .merge(Yaml::file("settings.yml"))
            .extract::<Settings>()
            .map_or_else(
                |err| {
                    warn!(
                        "Reading user settings error, restore to default {}",
                        err.to_string()
                    );
                    let m = CacheManager::from(Settings::default());

                    tauri::async_runtime::block_on(m.save("settings.yml"))
                        .map_err(|err| {
                            panic!("Error! {:?}", err);
                        })
                        .unwrap();
                    m
                },
                CacheManager::from,
            )
    };
}

pub async fn get_settings_manager() -> &'static CacheManager<Settings> {
    &SETTINGS
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Settings {
    pub genuis: GeniusCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeniusCredential {
    pub token: String,
    pub id: String,
}

impl Default for GeniusCredential {
    fn default() -> Self {
        Self {
            token: "default".to_string(),
            id: "default".to_string(),
        }
    }
}

struct OtherSetting {
    pub a: String,
}

impl From<Settings> for CacheManager<Settings> {
    fn from(s: Settings) -> Self {
        CacheManager {
            fresh: Mutex::new(false),
            cache: Mutex::new(s),
        }
    }
}

impl CacheManager<Settings> {
    pub async fn update(&self) {
        let conf = {
            let conf = Figment::new().merge(Yaml::file("settings.yml")).extract();
            conf.map_or(self.get_cache().await.clone(), |c| c)
        };
        self.set_cache(conf).await;
    }
}

pub async fn init() {
    let (tx, mut rx) = channel(4);
    let mut watcher: RecommendedWatcher = {
        let a = RecommendedWatcher::new(
            move |result: std::result::Result<Event, Error>| {
                tx.blocking_send(result).expect("Failed to send event");
            },
            Config::default(),
        );
        a.unwrap()
    };
    watcher
        .watch(Path::new("settings.yml"), RecursiveMode::NonRecursive)
        .unwrap();
    while let Some(res) = rx.recv().await {
        match res {
            Ok(e) if e.kind.is_modify() => {
                println!("update!");
                SETTINGS.set_fresh(false).await;
                SETTINGS.update().await;
                info!("{:?}", SETTINGS.get_cache().await);
            }
            Err(e) => {
                warn!("channel recv error {}", e);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::init;

    #[test]
    pub fn test_init() {
        let r = tokio::runtime::Runtime::new().unwrap();
        r.block_on(init());
    }
}
