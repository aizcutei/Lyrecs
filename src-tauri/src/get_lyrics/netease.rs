use regex::Regex;
use reqwest::header::{COOKIE, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;
use serde_json::Value;
use anyhow::Ok;
use anyhow::Result as AnyResult;
use strsim::levenshtein;
use log::info;

use crate::api::lyric_line::LyricTimeLine;
use crate::get_lyrics::song_struct::{NeteaseSong, NeteaseSongList, NeteaseSongLyrics};
use crate::api::lyric_line::{Lrcx, IDTag};
use crate::parse_lyric::utils;

use super::song_struct::Song;

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36";
const COOKIE_STRING: &str = "NMTID=1";
const SEARCH_URL: &str = "http://music.163.com/api/search/pc?type=1&offset=0&s=";
const LYRIC_URL: &str = "http://music.163.com/api/song/lyric?lv=1&kv=1&tv=-1&id=";

async fn get_song_list(key_word: &str, number: i32) -> AnyResult<NeteaseSongList> {

    let requrl = SEARCH_URL.to_string() + key_word + "&limit=" + &number.to_string();

    let client = reqwest::Client::new();

    let resp = client.post(requrl)
        .header(COOKIE, COOKIE_STRING)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send().await?;

    //println!("{:?}", res.text());

    let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str())?;

    //println!("{:?}", json["result"]["songs"][0]["id"].as_i64());

    if json["code"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_song_list error"));
    }

    let mut song_list = NeteaseSongList::new();

    info!("reveived song list");
    for song in json["result"]["songs"].as_array().unwrap() {
        info!("{:?}", song["name"]);
        let song = NeteaseSong::new(song);
        song_list.push(song);
    }
    
    Ok(song_list)
}

pub async fn get_song_lyric(song: &NeteaseSong) -> AnyResult<NeteaseSongLyrics> {

    if song.is_empty() {
        return Err(anyhow::anyhow!("No search result"));
    }
    
    let requrl = LYRIC_URL.to_string() + &song.id.to_string();

    let client = reqwest::Client::new();
    let res = client.post(requrl)
        .header(COOKIE, COOKIE_STRING)
        .header(USER_AGENT, USER_AGENT_STRING)
        .header(CONTENT_TYPE, "application/json")
        .send().await?;

    let re = res.text().await.unwrap();
    let json: Value = serde_json::from_str(re.as_str())?;

    if json["code"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_song_lyric error"));
    }

    let mut lrc = NeteaseSongLyrics::new();

    if let Some(lyc) = json.get("lrc") {
        if lyc.get("lyric").unwrap().is_null() {
            lrc.lyric = "[00:00.000] This Music Has No Lyric".to_string();
        } else {
            lrc.lyric = lyc.get("lyric").unwrap().to_string().replace("\\n", "\n"); // replace \\n to \n
        };
    } else {
        lrc.lyric = "[00:00.000] Get lyric Error".to_string();
    }

    if let Some(tlyric) = json.get("tlyric") {
        if tlyric.get("lyric").unwrap().is_null() {
            lrc.tlyric = "".to_string();
        } else {
            lrc.tlyric = tlyric.get("lyric").unwrap().to_string().replace("\\n", "\n");
        };
    } else {
        lrc.tlyric = "".to_string();
    }

    if let Some(klyric) = json.get("klyric") {
        if klyric.get("lyric").unwrap().is_null() {
            lrc.klyric = "".to_string();
        } else {
            lrc.klyric = klyric.get("lyric").unwrap().to_string().replace("\\n", "\n");
        };
    } else {
        lrc.klyric = "".to_string();
    }

    Ok(lrc)

}

pub async fn get_default_song(song_name: &NeteaseSong) -> NeteaseSong {
    let song_list = get_song_list(&song_name.name, 1).await.unwrap();
    if song_list.len() == 0 {
        return NeteaseSong::new_empty();
    }
    song_list[0].to_owned()
}

pub async fn get_best_match_song(song_name: &NeteaseSong) -> NeteaseSong {
    let song_list = get_song_list(&song_name.name, 5).await.unwrap();
    if song_list.len() == 0 {
        return NeteaseSong::new_empty();
    }
    
    let mut best_match_song = song_list[0].to_owned();
    let mut best_match_distance = usize::MAX;

    for song in song_list {
        let listed_song_name = format!("{} {}", song.artist, song.name);
        let distance = levenshtein(&listed_song_name, &song_name.name);
        //println!("{} {} {}", listed_song_name, song_name.name, distance);
        if distance < best_match_distance {
            best_match_song = song.to_owned();
            best_match_distance = distance;
        }
    }

    //if distance > 6 , search artist and song name
    if best_match_distance > 6 {
        let search_key_word = format!("{} {}", song_name.artist, song_name.name);
        let song_list = get_song_list(&search_key_word, 5).await.unwrap();

        if song_list.len() == 0 {
            return NeteaseSong::new_empty();
        }

        for song in song_list {
            let listed_song_name = format!("{} {}", song.artist, song.name);
            let distance = levenshtein(&listed_song_name, &song_name.name);
            //println!("{} {} {}", listed_song_name, song_name.name, distance);
            if distance < best_match_distance {
                best_match_song = song.to_owned();
                best_match_distance = distance;
            }
        }
    }

    best_match_song
}

pub fn parse_netease_lyric(s: String, splitter: &str) -> AnyResult<Lrcx> {
    let mut lrcx: Lrcx = Default::default();
    let mut lrc_string: &str = &s;

    if s.starts_with('"') {
        lrc_string = &s[1..s.len() - 1];
    }

    let lines: Vec<&str> = lrc_string.split(splitter).collect();
    info!("{} lines", lines.len());
    let lrc_metatag_regex = Regex::new(r#"\[[a-z]+\]"#).unwrap();
    let lrc_timeline_regex = Regex::new(r#"\[[0-9]+:[0-9]+.[0-9]+\]"#).unwrap();

    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if lrc_metatag_regex.captures(line).is_some() {
            let re = lrc_metatag_regex.captures(line).unwrap();
            let tag_name = re.get(0).unwrap().as_str();
            let tag_value = line[tag_name.len() + 1..].trim().to_string();
            lrcx.metadata.insert(IDTag::new(tag_name.to_string(), tag_value));
            continue;
        }
        if lrc_timeline_regex.captures(line).is_some() {
            // let timestamp = {
            //     let t = lrc_timeline_regex.captures(line).unwrap()
            //                 .get(0).unwrap().as_str();
            //     utils::time_tag_to_time_f64(t[1..t.len() - 1].trim())
            // };
            let timestamp = lrc_timeline_regex.captures(line).unwrap()
                             .get(0).unwrap().as_str().to_string();
            let lyric_line: LyricTimeLine = Default::default();
            let verse = line[timestamp.to_string().len()..].trim().to_string();
            lyric_line.line.text = verse;
            lyric_line.line.length = verse.len() as i64;
            lyric_line.time = timestamp;
            lrcx.lyric_body.push(lyric_line);
            continue;
        }
    }
    Ok(lrcx)
}