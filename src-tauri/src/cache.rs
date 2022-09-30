use anyhow::{Ok, Result as AnyResult};
use serde::Serialize;
use tokio::{
    fs::OpenOptions,
    io::{AsyncWrite, AsyncWriteExt},
    sync::{Mutex, MutexGuard},
};

pub struct CacheManager<T> {
    pub fresh: Mutex<bool>,
    pub cache: Mutex<T>,
}

impl<T: Default> CacheManager<T> {
    pub fn new() -> Self {
        CacheManager {
            fresh: Mutex::new(false),
            cache: Mutex::new(Default::default()),
        }
    }

    pub async fn get_cache(&self) -> MutexGuard<'_, T> {
        self.cache.lock().await
    }

    pub async fn set_cache(&self, t: T) {
        let mut l = self.cache.lock().await;
        let mut f = self.fresh.lock().await;
        *l = t;
        *f = true;
    }

    pub async fn is_fresh(&self) -> bool {
        *self.fresh.lock().await
    }

    pub async fn set_fresh(&self, fresh: bool) {
        let mut f = self.fresh.lock().await;
        *f = fresh;
    }
}

impl<T: Serialize> CacheManager<T> {
    pub async fn save(&self, path: &str) -> AnyResult<()> {
        let cache = self.cache.lock().await;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(path)
            .await?;
        let s = serde_yaml::to_string(&*cache)?;
        file.write_all(s.as_bytes()).await?;
        Ok(())
    }
}
