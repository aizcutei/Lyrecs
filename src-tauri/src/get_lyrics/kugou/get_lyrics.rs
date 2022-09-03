use std::fs::File;
use std::io::Read;
use std::io::Write;

use anyhow::Result as AnyResult;
use base64::{decode_config, STANDARD_NO_PAD};
use flate2::read::ZlibDecoder;
use log::info;
use log::warn;
use reqwest::header::USER_AGENT;
use serde_json::Value;
use std::result::Result::Ok;

use crate::get_lyrics::kugou::model::{KugouSong, KugouSongList, KugouSongLyrics};
use crate::get_lyrics::lyric_file::get_client_provider;
use crate::get_lyrics::lyric_file::lyric_file_path;
use crate::get_lyrics::song::RemoteSongTrait;

const USER_AGENT_STRING: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36";
const SEARCH_URL: &str =
    "http://msearchcdn.kugou.com/api/v3/search/song?plat=0&version=9108&keyword=";
const LYRIC_SEARCH_URL: &str = "http://krcs.kugou.com/search?ver=1&man=yes&hash=";
const LYRIC_URL: &str = "http://lyrics.kugou.com/download?ver=1&client=pc&id=";
const KEYS: [u8; 16] = [
    64, 71, 97, 119, 94, 50, 116, 71, 81, 54, 49, 45, 206, 210, 110, 105,
];

async fn get_song_list(key_word: &str, number: i32) -> AnyResult<KugouSongList> {
    let requrl = SEARCH_URL.to_string() + key_word + "&pagesize=" + &number.to_string();
    let client = get_client_provider().get().await;

    info!("requesting song list");
    let resp = client
        .get(requrl)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send()
        .await?;

    info!("received song list");

    let resp_str = resp.text().await.unwrap();
    let json: Value = serde_json::from_str(&resp_str)?;

    if json["status"].as_i64() != serde::__private::Some(1) {
        return Err(anyhow::anyhow!("get_song_list error"));
    }

    let mut song_list = KugouSongList::new();

    info!("paresed song list");

    for song in json["data"]["info"].as_array().unwrap() {
        info!("{:?}", song["songname"]);
        let song = KugouSong::new(song);
        song_list.push(song);
    }

    Ok(song_list)
}

pub async fn get_default_song(song_name: &str) -> KugouSong {
    match get_song_list(song_name, 1).await {
        Ok(song_list) => {
            if song_list.len() == 0 {
                info!("no song found");
                return KugouSong::new_empty();
            }
            song_list[0].clone()
        }
        Err(e) => {
            warn!("error: {}", e.to_string());
            KugouSong::new_empty()
        }
    }
}

pub async fn get_lyrics_list(song: &KugouSong) -> AnyResult<KugouSongList> {
    let requrl = LYRIC_SEARCH_URL.to_string() + &song.hash;

    let client = get_client_provider().get().await;

    info!("requesting lyrics list");
    let resp = client
        .get(requrl)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send()
        .await?;

    let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str())?;

    if json["errcode"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_lyrics_list error"));
    }

    let mut lyrics_list = KugouSongList::new();

    info!("parsed lyrics list");

    for song in json["candidates"].as_array().unwrap() {
        info!("{:?}, {:?}", song["song"], song["product_from"]);
        let song = KugouSong::new(song);
        lyrics_list.push(song);
    }

    Ok(lyrics_list)
}

pub async fn get_default_lyric_item(lyric_list: &KugouSongList) -> KugouSong {
    if lyric_list.len() == 0 {
        info!("no song found");
        return KugouSong::new_empty();
    }
    lyric_list[0].clone()
}

pub async fn get_song_lyric(song: &KugouSong) -> AnyResult<KugouSongLyrics> {
    let requrl = LYRIC_URL.to_string() + &song.id + "&accesskey=" + &song.access_key;

    let client = get_client_provider().get().await;

    let resp = client
        .get(requrl)
        .header(USER_AGENT, USER_AGENT_STRING)
        .send()
        .await?;

    let json: Value = serde_json::from_str(resp.text().await.unwrap().as_str())?;

    if json["status"].as_i64() != serde::__private::Some(200) {
        return Err(anyhow::anyhow!("get_song_lyric error"));
    }

    let mut content = "".to_string();
    if json.get("content").is_some() {
        content = json["content"].as_str().unwrap().to_string();
    }

    let mut lyric = KugouSongLyrics::new(content);

    lyric = decode_lyric(&mut lyric).await?;

    Ok(lyric)
}

pub async fn kugou_get_first_lyric(key_word: &str) -> AnyResult<KugouSongLyrics> {
    let song = get_default_song(key_word).await;
    let lyric_list = get_lyrics_list(&song).await?;
    let lyric = get_default_lyric_item(&lyric_list).await;
    get_song_lyric(&lyric).await
}

pub async fn decode_lyric(lyric: &mut KugouSongLyrics) -> AnyResult<KugouSongLyrics> {
    let mut decoded =
        decode_config(lyric.content.as_bytes(), STANDARD_NO_PAD).expect("decode error");

    if String::from_utf8_lossy(&decoded[..4]) != "krc1" {
        return Err(anyhow::anyhow!("decode error"));
    }

    let (_, input) = decoded.split_at_mut(4);

    for i in 0..input.len() {
        input[i] ^= KEYS[i % 16];
    }

    //println!("{:?}", String::from_utf8_lossy(&input[..]));

    let mut decoder = ZlibDecoder::new(&input[..]);
    let mut result = String::new();
    decoder.read_to_string(&mut result).unwrap();

    lyric.decoded = result;

    Ok(lyric.to_owned())
}

pub async fn save_lyric_file(song: &KugouSong) -> AnyResult<()> {
    info!("getting default song");

    //remove & in the keyword
    let mut search_song = format!(
        "{} {}",
        song.name.clone().replace('&', ""),
        song.artist.clone().replace('&', "")
    );

    let default_song = get_default_song(&search_song).await;
    info!("default song {:?} \n getting lyric", default_song.name);

    let lyrics_list = get_lyrics_list(&default_song).await.unwrap();

    let song_lyrics = get_default_lyric_item(&lyrics_list).await;

    let lyric_str = get_song_lyric(&song_lyrics).await.unwrap();

    //let lrcx = Lrcx::from_str(song_lyrics.get_original_lyric().unwrap(), "\n").unwrap();
    info!("writing lyric file of length");

    let mut file = File::create(lyric_file_path(&song.artist, &song.name))?;
    /*
    if lrcx.is_empty() {
        file.write_all(b"[00:00.000] No Lyric for this song\n[00:10.000] \xE2\x99\xAB ~ ~ ~")?; //add a start line
    } else {
        file.write_all(b"[00:00.000] \n")?; //add a start line
    }


    for line in lrcx.iter() {
        file.write_all(line.to_string().as_bytes())?;
        file.write_all(b"\n")?;
    }
    */
    write!(file, "{}", serde_json::to_string(&lyric_str.to_lrcx())?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::get_lyrics::kugou::model::KugouSongLyrics;

    use super::decode_lyric;

    #[test]
    fn decode_part() {
        let encryted = r"a3JjMTjbDC+VXGmAQOs25fGi\/4xNl7SYeBSufBFgMD\/tVHnNCY5VY2W84EOL\/4BNVUAHI1BXRtXS6voiScYSodDDTTiyiJEpjxmLBbtiZOJCzTLhfdv4mfRBOhLM54aicawPDiu6BrkgbIiuHmjp1S0ohOqPERZ4yG2pgaTzrNgrESE++5D3jKnKK7s4xeMXsG\/SGKGilDVFX09Ai+zv8TznueSylTevUFWJkQfr74eDZFJGjxi5TqmYWBkua09wsj9MtuELTGICjQKxvbPcpiXAq5DNEK6KN9GosavAlHJwYIf6u5oO5CXc4Xg9CM7GcT4ClL5w9ims1Yv76MVt0xshgH44hqYyL9exhnykuia5VB1xJMKUql1rU6VaeFpAhvun9N+sUxaTa3qeuOkIfJc5dZK3wf3FAMJpefwya3086mDm33vY+t1Di7OOTTI7ci6YkVbR246A9Xd+4ws\/osisNXcaWV6Vo1h5Om5PMzYU\/MXq+\/VqrtDWyCooy8+ZBYlp1Uv+8WHfe7R5D1ZpE3frDozRn\/iSn9NkGuJN7vG06AqrgQZhzG2m6KqYpZLWjfnaYUCtrcQFZXJNbxnBH\/EokBZHbR3h+bIj0MbJ8IoJNKxLhlcQDRZntD9pVtQ08\/abSN9QmqKP4dVRbb8cYtHucqE7w7do+\/pTA+gd1vrYxS+Hwm6OarnaN1toXxA5AAxVpH6zZg7Lm+cdaOkZnFPukH7fj0gNF8OA5J62V8u22CCVoHX+v\/Jm+1QGScryLtlJJrxkuahe7FWi6z4U6tcyMdRpx0rfeXUPRGfiSgrelugbLMjA5+nBNQqa8zMq+5ZNU2cIvzQuP32Mkqnt6j\/3aqZZUFwp6TOr+1u0wBqhbqPhYwJomsPx1aE1953zM07b\/duaPSqjCVxJJx+5ipzeYAj8drmj\/CltsFWAc16D+yFKpY6Td9F7cFdr0+Vo60Z9fWdgS5KAK0kNKNdmdCGI7bcV8MGdhXrNCBWqGopm0woSUuyFLbkOU3wDD8bjivzyLs91Fuqt9HQgUEtI2eCRp8uj\/XR7MJGyO3xkNhMCtkj1R0nFb7k7oII1qgYjnDtXUDrj3Z3V8DVvkNg7Khdhyj0KvFZnURmwcjgtZvMu0uxG8WrjOLv13pvK3JJuXrfIE+2PtMJjNjTRo8QXciiY5Sv9lJinojDkMsF1R033tbyHavffBD3yXn18kx4VCbAPYjOZMqCHpQNLNT\/WyTZoSMyRam6InRXyIfCZX8L8NP8F65TKCENOEVt6P4RaOMMholn9jXfaI\/R3wnZTI61gQrk65LxjWjBcl\/MmNVW05k8megEDz6eiZUa\/Xz1No1juvEtk4SJEI1yeptAwKUaIwaDrQFnmtxs5vb\/tykaNtra4f0rib8a2XDpMLvU3Bqixg5Gy4DyBcylzh0jD9f7io2EJgeH33qdBXP8RVRFbgfu3WwApT15ZiZAnowBgiDjOEfDcGtGC2NgqMBH+vlCJzx5jA60xG7pNC\/xWSN1\/ZuxJDHgaCy369G\/eJfRUphTjebHiM7FWQ3CSCkr6CgmI9ijroao4ZrAkKrLjpFJMW+q3HMKcgE9i\/x2nBGVkTtXcMRm2EDhRnwN\/v\/BabocD3q3jStOgEuzOgFFH1nuKRDHW+YHDFp8WXVwAG3iCxI\/biRNNzLxoNcDWO7pYxemV+W7sE\/gf1j5UCHpywUE504g6qkp405Y63JrWmnAYoFgR4WWz3OHlSVwX\/74RN\/WvHmGbBfMk\/wp9HnYwd0Z74\/obXw15NMvIJMUSVv9XHVMrCZUDCpmcWrCDweibM8\/GzLG7g0nLaVaW1NMGfyFZKKoNmQXTqtgOYQ83EPBU8U121zQQQ3a66C9qe2+XuDK\/WgeNAKn1yjcxx87yG6PdcZ74mtgFFTbxamAXHljD3NiJPouvc0W96mAMYD+kDuoexs+tzaf2HxHkPI2ZSGKv2G3K8NccuxhZv\/YKMFeqhnQnXzp\/e22j5w8vNLMQQrA+erWK\/rTw\/rcUVBPslMKoUTNrjvA4tXAT+1lAJTv+rbR8dRyNBWVcCMmRXR5+EQN6+l8SGTri9EWeTLRKlPHVLIxiE9kwKS3FWC1DpoCR5kZpg2oQqzzuCxBhEJpINee1tfsRj5sCTud8tMJuaKpdwycainoUkD8UtTLEvwDJ9IYhQGZmk4vMRK4KE\/2c7cC2A1fpXCLb4igIpG1QA+Pl1cfUBwvj1DCpsWOpBIcAvlJU2olA8uc\/vi5SQ\/IVPUG+miJWpG7w+i0hWAIHEi3GSOK8eIkpqE+jbR6JodW0hZKgVw8Z7Ss6EhR0yxAsQbiyu4SXUGIx0uTJyfXm4q+DolKP618txlcHAQ+VEAl6dYMsqrV+OS4W8fl7jI5j\/haip\/xKOP2\/0rk117d4wKrw+8SIRUWNj8eSpWDE9orl+gdh+Yq0srja9E5KAEpztPBT7mkWNodkpGol2FGuRHrCE6rMoyTK\/LAhaBT85CKy3qucw3lV\/ifE2TdPb7dWIapFqYOnXqzi0cHoiZC4M3uYol5jbZ50FZT8ucsTo9ANfhFJOSo6Iz6pGaBc9s619xv0UEig\/PcFxwIWPHiznk4eeP\/kwnMhWjQEfi\/ISEW+w0Gy5BYNCWbI5lPnsq8wFwjlLVo4ikb7fS9KZh3wdFDM+GX7jH1wy+wdQvOdujNJ3vXNueg4sv75MvOhiNclzbh\/HK1pE9DMKyYaMyJJqg\/w4+T2xFwZv5t3\/cJFFcxxcP4MPL\/K1\/\/xOEkC\/XTxcZrYu0Mww8xurJ7Xgwqcij9lAmpPyVY3VtqOZNSGmzOo1It66L2e7gPOw5dJcPu1bhrZMc+oo9hy+jzVQXkppoT529ZtpQ7llHSxli6j1iWE7syVZbnpokNZMPN+pz5YAncF3XAMIMct27EmNML3nZNJ2Y\/tkLr9bhLI6jdzTX+DC5pk+eZqo34DKWQJq7jyOcHsZ24dmftm2jE7eOauwfn9Mz6EzhpMURxApsUcDoDn2gOROFu9xsMWNVlvHOQY0HWSyjq6sNtG3SSZYca\/r0mvWxoDqkQ2050cpxDK4B2wMdFHtkwN";

        let mut lrc = KugouSongLyrics::new_empty();

        lrc.content = encryted.to_string();

        decode_lyric(&mut lrc);
    }
}
