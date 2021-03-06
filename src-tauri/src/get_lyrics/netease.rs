use reqwest::header::USER_AGENT;
use reqwest::header::COOKIE;
use serde_json::Value;
use anyhow::Ok;
use anyhow::Result as AnyResult;

use crate::get_lyrics::song_struct::{Song, SongList, SongLyrics};

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36";
const COOKIE_STRING: &str = "NMTID=1";
const SEARCH_URL: &str = "http://music.163.com/api/search/pc?type=1&limit=5&offset=0&s=";
const LYRIC_URL: &str = "http://music.163.com/api/song/lyric?lv=1&kv=1&tv=-1&id=";

pub fn get_song_list(key_word: &str) -> AnyResult<SongList> {

    let requrl = SEARCH_URL.to_string() + key_word;

    let client = reqwest::blocking::Client::new();

    let resp = client.post(requrl)
        .header(COOKIE, COOKIE_STRING)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send()?;

    //println!("{:?}", res.text());

    let json: Value = serde_json::from_str(resp.text().unwrap().as_str())?;

    //println!("{:?}", json["result"]["songs"][0]["id"].as_i64());

    if json["code"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_song_list error"));
    }

    let mut song_list = SongList::new();

    for song in json["result"]["songs"].as_array().unwrap() {
        let song = Song::new(song);
        song_list.push(song);
    }
    
    Ok(song_list)
}

pub fn get_song_lyric(song: &Song) -> AnyResult<SongLyrics> {
    
    let requrl = LYRIC_URL.to_string() + &song.id.to_string();

    let client = reqwest::blocking::Client::new();

    let res = client.post(requrl)
        .header(COOKIE, COOKIE_STRING)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send()?;

    let json: Value = serde_json::from_str(res.text().unwrap().as_str())?;

    if json["code"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_song_lyric error"));
    }

    let mut lrc = SongLyrics::new();

    if let Some(lyc) = json.get("lrc") {
        if lyc.get("lyric").unwrap().is_null() {
            lrc.lyric = "[00:00.000] This Music No Lyric".to_string();
        } else {
            lrc.lyric = lyc.get("lyric").unwrap().to_string();
        };
    } else {
        lrc.lyric = "[00:00.000] Get lyric Error".to_string();
    }

    if let Some(tlyric) = json.get("tlyric") {
        if tlyric.get("lyric").unwrap().is_null() {
            lrc.tlyric = "".to_string();
        } else {
            lrc.tlyric = tlyric.get("lyric").unwrap().to_string();
        };
    } else {
        lrc.tlyric = "".to_string();
    }

    if let Some(klyric) = json.get("klyric") {
        if klyric.get("lyric").unwrap().is_null() {
            lrc.klyric = "".to_string();
        } else {
            lrc.klyric = klyric.get("lyric").unwrap().to_string();
        };
    } else {
        lrc.klyric = "".to_string();
    }

    Ok(lrc)

}

pub fn get_default_song(song_name: &str) -> Song {
    let song_list = get_song_list(song_name).unwrap();
    song_list[0].to_owned()
}

