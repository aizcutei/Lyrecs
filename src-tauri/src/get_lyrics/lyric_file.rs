use std::cell::{RefCell, Cell, Ref};
use std::fs::File;
use std::io::Read;
use tokio::sync::{Mutex, MutexGuard};
use std::path::PathBuf;
use anyhow::{Result as AnyResult, Ok};
use directories::UserDirs;
use log::info;
use reqwest::Client;

use crate::api::model::Lrcx;
use crate::get_lyrics::kugou::get_lyrics::kugou_save_lyric_file;
use crate::get_lyrics::netease::get_lyrics::save_lyric_file;

use super::cache::get_cache_manager;
use super::kugou::model::KugouSong;
use super::netease::model::NeteaseSong;

lazy_static! {
    static ref CLIENT_PROVIDER: ClientProvider = ClientProvider::new();
}

pub enum LyricSource {
    Netease(NeteaseSong),
    Kugou(KugouSong),
}

pub struct ClientProvider {
    client: Mutex<Client>,
}

pub fn get_client_provider() -> &'static ClientProvider {
    &CLIENT_PROVIDER
}

impl ClientProvider {
    pub fn new() -> Self{
        ClientProvider{
            client: Mutex::new(Client::new()),
        }
    }

    pub async fn get(&self) -> MutexGuard<'_, Client> {
        self.client.lock().await
    }

    pub async fn set(&self, client:Client){
        let mut c = self.client.lock().await;
        *c = client;
    }
}


pub fn lyric_file_path(artist : &str, title : &str) -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    let document_path = user_dirs.document_dir().unwrap();
    let lyrecs_path = document_path.join("Lyrecs");
    if lyrecs_path.exists() {
        if !lyrecs_path.is_dir() {
            panic!("lyrecs path is not a directory");
        }
    } else {
        std::fs::create_dir_all(lyrecs_path.clone()).unwrap();
    }
    let file_name = format!("{} - {}.lrcx", artist, title);
    lyrecs_path.join(file_name)
}


pub fn lyric_file_exists(artist : &str, title : &str) -> bool {
    lyric_file_path(artist, title).exists()
}

pub async fn activate_lyric(song: LyricSource) -> AnyResult<Lrcx> {
    let cache_manager = get_cache_manager();
    if cache_manager.is_fresh().await {
        return Ok(cache_manager.get_cache().await.clone())
    }
    // if netease blabla if kugou blabla
    let lyric_str = get_lyric_file(song).await.unwrap();
    if lyric_str.is_empty() {
        return Err(anyhow::anyhow!("lyric file is empty"))
    }
    let lrc = serde_json::from_str::<Lrcx>(&lyric_str)?;
    cache_manager.set_cache(lrc.clone()).await;
    Ok(lrc)
}

pub async fn get_lyric_file(song: LyricSource) -> AnyResult<String> {
    let (artist, title) = match song {
        LyricSource::Netease(n) => {
            if !lyric_file_exists(&n.artist, &n.name) {
                save_lyric_file(&n).await?;
            }
            (n.artist, n.name)
        },
        LyricSource::Kugou(k) => {
            if !lyric_file_exists(&k.artist, &k.name) {
                kugou_save_lyric_file(&k).await?;
            }
            (k.artist, k.name)
        },
    };

    let lyric_path = lyric_file_path(&artist, &title);
    let mut file = File::open(lyric_path)?;
    let mut lyric = String::new();
    file.read_to_string(&mut lyric)?;
    Ok(lyric)
}

