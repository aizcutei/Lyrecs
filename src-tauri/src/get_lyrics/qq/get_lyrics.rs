use regex::Regex;
use reqwest::header::{COOKIE, CONTENT_TYPE, USER_AGENT, REFERER};
use serde_json::Value;
use anyhow::Ok;
use anyhow::Result as AnyResult;
use strsim::levenshtein;
use log::info;

use crate::get_lyrics::lyric_file::get_client_provider;
use crate::get_lyrics::song::RemoteSongTrait;

use super::model::QQSongLyrics;
use super::model::{QQSong, QQSongList};

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36";
const SEARCH_URL: &str = "https://u.y.qq.com/cgi-bin/musicu.fcg";
const SEARCH_URL_ALT: &str = "https://c.y.qq.com/splcloud/fcgi-bin/smartbox_new.fcg";
const LYRIC_URL: &str = "http://c.y.qq.com/lyric/fcgi-bin/fcg_query_lyric.fcg";

async fn get_song_list(key_word: &str, number: i32) -> AnyResult<QQSongList> {
    let data = r#"{"music.search.SearchCgiService":{"method":"DoSearchForQQMusicDesktop","module":"music.search.SearchCgiService","param":{"num_per_page":""#.to_string() + &number.to_string() + r#"","page_num":"1","query":""# + key_word + r#"","search_type":"0"}}}"#;
    let requrl = SEARCH_URL.to_string();
    let client = get_client_provider().get().await;
    let res = client.post(requrl)
        .header(USER_AGENT, USER_AGENT_STRING)
        .header(CONTENT_TYPE, "application/json")
        .body(data)
        .send().await?;

    let json: Value = serde_json::from_str(res.text().await.unwrap().as_str())?;

    if json["music.search.SearchCgiService"]["data"]["body"]["song"]["list"].as_array().unwrap().is_empty() {
        return Err(anyhow::anyhow!("qq get_song_list error"));
    }

    let mut song_list = QQSongList::new();

    info!("reveived song list");
    for song in json["music.search.SearchCgiService"]["data"]["body"]["song"]["list"].as_array().unwrap() {
        info!("{:?}", song["name"]);
        let song = QQSong::new(song);
        song_list.push(song);
    }

    Ok(song_list)
}

pub async fn get_default_song(song_name: &str) -> QQSong {
    let song_list = get_song_list(song_name, 1).await.unwrap();
    if song_list.len() == 0 {
        info!("qq no song found");
        return QQSong::default();
    }
    song_list[0].clone()
}

pub async fn get_song_lyric(song: &QQSong) -> AnyResult<QQSongLyrics> {

    let refurl = r#"http://y.qq.com/portal/song/"#.to_owned() + &song.mid.to_string() + r#".html"#;
    let requrl = format!("nobase64=1&musicid={}&callback=jsonp1&g_tk=5381&jsonpCallback=jsonp1&loginUin=0&hostUin=0&format=jsonp&inCharset=utf8&outCharset=utf-8&notice=0&platform=yqq&needNewCode=0", song.id);
    let client = get_client_provider().get().await;
    let resp = client.get(LYRIC_URL.to_string() + "?" + requrl.as_str())
        .header(USER_AGENT, USER_AGENT_STRING)
        .header(REFERER, refurl)
        .send().await?;

    let text = resp.text().await.unwrap();
    //println!("{:?}", &text[7..&text.len()-1]);

    let json: Value = serde_json::from_str(&text[7..&text.len()-1])?;

    if json["lyric"].as_str().unwrap().is_empty() {
        return Err(anyhow::anyhow!("qq get_song_lyric error"));
    }

    let lyric = QQSongLyrics::new(&json);

    Ok(lyric)
}

pub async fn qq_save_lyric_file(lyric: &QQSongLyrics) -> AnyResult<()> {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::{get_default_song, get_song_lyric};

    #[tokio::test]
    async fn qq_get() {
        let song = get_default_song("花の塔").await;
        let lyric = get_song_lyric(&song).await;
    }
}
