use std::any;
use std::fs::File;
use std::io::{Write, Read};

use regex::Regex;
use reqwest::header::{COOKIE, CONTENT_TYPE, USER_AGENT};
use serde_json::Value;
use anyhow::Ok;
use anyhow::Result as AnyResult;
use strsim::levenshtein;
use log::info;

use crate::get_lyrics::lyric_file::{lyric_file_path, lyric_file_exists};
use crate::get_lyrics::netease::model::{NeteaseSong, NeteaseSongList, NeteaseSongLyrics};
use crate::api::model::{Lrcx, IDTag, LyricTimeLine};
use crate::parse_lyric::utils::time_tag_to_time_f64;
use crate::get_lyrics::song::{RemoteSongTrait};

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36";
const COOKIE_STRING: &str = "NMTID=1";
const SEARCH_URL: &str = "http://music.163.com/api/search/pc?type=1&offset=0&s=";
const LYRIC_URL: &str = "http://music.163.com/api/song/lyric?lv=1&kv=1&tv=-1&id=";


async fn get_song_list(key_word: &str, number: i32) -> AnyResult<NeteaseSongList> {

    let requrl = SEARCH_URL.to_string() + key_word + "&limit=" + &number.to_string();

    let client = reqwest::Client::builder().proxy(reqwest::Proxy::http("http://127.0.0.1:7890")?)
    .proxy(reqwest::Proxy::https("https://127.0.0.1:7890")?)
    .build().unwrap();

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
            let timestamp = lrc_timeline_regex.captures(line).unwrap()
                             .get(0).unwrap().as_str().to_string();
            let mut lyric_line: LyricTimeLine = Default::default();
            let verse = line[timestamp.to_string().len()..].trim().to_string();
            lyric_line.line.text = verse.clone();
            lyric_line.line.length = verse.len() as i64;
            lyric_line.start = time_tag_to_time_f64(timestamp.trim_start_matches('[').
                                                    to_string().trim_end_matches(']'));
            lrcx.lyric_body.push(lyric_line);
            continue;
        }
    }
    lrcx.cal_duration_from_start();
    Ok(lrcx)
}

pub async fn save_lyric_file(song: &NeteaseSong) -> AnyResult<()> {
    info!("getting default song");

    let mut search_song = NeteaseSong::new_empty();
    search_song.name = song.name.clone().replace('&', "");
    search_song.artist = song.artist.clone().replace('&', "");
    //remove & in the keyword

    let default_song = get_best_match_song(&search_song).await;
    info!("default song {} \n getting lyric", default_song.name);

    let song_lyrics = get_song_lyric(&default_song).await?;
    info!("song_lyrics {:?}", song_lyrics);

    let mut lrcx = parse_netease_lyric(song_lyrics.get_original_lyric().unwrap(), "\n")?;
    info!("writing lyric file of length {}", lrcx.lyric_body.len());

    let mut file = File::create(lyric_file_path(&song.artist, &song.name))?;

    let mut default_timeline: LyricTimeLine = Default::default();
    if lrcx.lyric_body.is_empty() {
        default_timeline.line.set_text("No Lyric for this song".to_string());
    }
    lrcx.lyric_body.insert(0, default_timeline);
    let serial = serde_json::to_string(&lrcx)?;
    write!(file, "{}", serial)?;

    Ok(())
}

