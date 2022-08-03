use std::io::Read;

use flate2::read::ZlibDecoder;
use reqwest::header::{COOKIE, CONTENT_TYPE, USER_AGENT};
use base64::{decode_config, STANDARD_NO_PAD};
use serde::Deserialize;
use serde_json::Value;
use anyhow::Ok;
use anyhow::Result as AnyResult;
use strsim::levenshtein;
use log::info;

use crate::get_lyrics::song_struct::Song;
use crate::get_lyrics::song_struct::{KugouSong, KugouSongList, KugouSongLyrics};

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36";
const SEARCH_URL: &str = "http://msearchcdn.kugou.com/api/v3/search/song?plat=0&version=9108&keyword=";
const LYRIC_SEARCH_URL: &str = "http://krcs.kugou.com/search?ver=1&man=yes&hash=";
const LYRIC_URL: &str = "http://lyrics.kugou.com/download?ver=1&client=pc&id=";
const KEYS: [u8; 16] = [64, 71, 97, 119, 94, 50, 116, 71, 81, 54, 49, 45, 206, 210, 110, 105];

pub async fn get_song_list(key_word: &str, number: i32) -> AnyResult<KugouSongList> {

    let requrl = SEARCH_URL.to_string() + key_word + "&pagesize=" + &number.to_string();

    let client = reqwest::Client::new();

    let resp = client.post(requrl)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send().await?;

        let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str())?;

        if json["errcode"].as_i64() != serde::__private::Some(0) {
            return Err(anyhow::anyhow!("get_song_list error"));
        }

        let mut song_list = KugouSongList::new();

        info!("reveived song list");
        for song in json["data"]["info"].as_array().unwrap() {
            info!("{:?}", song["songname"]);
            let song = KugouSong::new(song);
            song_list.push(song);
        }

    Ok(song_list)
}

pub async fn get_default_song(song_name: &str) -> KugouSong {
    let song_list = get_song_list(song_name, 1).await.unwrap();
    if song_list.len() == 0 {
        info!("no song found");
        return KugouSong::new_empty();
    }
    song_list[0].clone()
}

pub async fn get_lyrics_list(song: &KugouSong) -> AnyResult<KugouSongList> {

    let requrl = LYRIC_SEARCH_URL.to_string() + &song.hash;

    let client = reqwest::Client::new();

    let resp = client.get(requrl)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send().await?;

    let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str())?;

    if json["errcode"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_lyrics_list error"));
    }

    let mut lyrics_list = KugouSongList::new();

    info!("reveived lyrics list");

    for song in json["candidates"].as_array().unwrap() {
        info!("{:?}, {:?}", song["song"], song["product_from"]);
        let song = KugouSong::new(song);
        lyrics_list.push(song);
    }

    Ok(lyrics_list)
}

pub async fn get_song_lyric(song: &KugouSong) -> AnyResult<KugouSongLyrics> {
    
    let requrl = LYRIC_URL.to_string() + &song.id + "&accesskey=" + &song.access_key;

    let client = reqwest::Client::new();

    let resp = client.get(requrl)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send().await?;

    let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str())?;

    if json["status"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_song_lyric error"));
    }

    let mut lyric = KugouSongLyrics::new(&json);

    lyric = decode_lyric(&mut lyric).await?;
    
    Ok(lyric)
}

pub async fn decode_lyric(lyric: &mut KugouSongLyrics) -> AnyResult<KugouSongLyrics> {

    let mut decoded = decode_config(lyric.content.as_bytes(), STANDARD_NO_PAD).expect("decode error");

    if String::from_utf8_lossy(&decoded[..4]) != "krc1" {
        return Err(anyhow::anyhow!("decode error"));
    }

    let (_, input) = decoded.split_at_mut(4);

    for i in 0..input.len() {
        input[i] ^= KEYS[i % 16];
    }

    let mut decoder = ZlibDecoder::new(&input[..]);

    let mut result = String::new();

    decoder.read_to_string(&mut result).unwrap();

    lyric.decoded = result;

    Ok(lyric.to_owned())
}