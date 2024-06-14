pub mod model;

use anyhow::{anyhow, Result};
use std::{collections::HashSet, time::Duration};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    time::sleep,
};

async fn build_blocklist(
    blocklist_response: &model::blocklist_response::Root,
) -> Result<Vec<model::blocklist::Item>> {
    let list = blocklist_response
        .data
        .as_ref()
        .and_then(|x| x.list.as_ref())
        .ok_or(anyhow!("blocklist_response has no list"))?;

    let result = list
        .iter()
        .map(|x| {
            let mid = x.mid.unwrap_or(-1).to_string();
            let uname = x.uname.clone().unwrap_or("".to_string());
            let face = x.face.clone().unwrap_or("".to_string());
            let space_url = format!("https://space.bilibili.com/{}", mid);
            model::blocklist::Item {
                mid,
                uname,
                face,
                space_url,
            }
        })
        .collect::<Vec<model::blocklist::Item>>();

    Ok(result)
}

async fn build_headers(cookie: &model::config::Cookie) -> Result<reqwest::header::HeaderMap> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authority", "api.bilibili.com".parse().unwrap());
    headers.insert("accept", "application/json, text/plain, */*".parse()?);
    headers.insert("accept-language", "zh-CN,zh;q=0.9,en;q=0.8".parse()?);
    headers.insert(
        reqwest::header::COOKIE,
        format!(
            "SESSDATA={}; bili_jct={};",
            cookie.sessdata, cookie.bili_jct
        )
        .parse()?,
    );
    headers.insert("origin", "https://account.bilibili.com".parse()?);
    headers.insert("referer", "https://account.bilibili.com/".parse()?);

    Ok(headers)
}

async fn get_blocklist_response(
    client: &reqwest::Client,
    headers: &reqwest::header::HeaderMap,
    pn: u32,
) -> Result<model::blocklist_response::Root> {
    let url = format!("https://api.bilibili.com/x/relation/blacks?pn={}&ps=50", pn);
    tracing::info!("url {}", url);

    let root = client
        .get(url)
        .headers(headers.clone())
        .send()
        .await?
        .json::<model::blocklist_response::Root>()
        .await?;

    Ok(root)
}

async fn get_blocklist(
    client: &reqwest::Client,
    cookie: &model::config::Cookie,
    pn: u32,
) -> Result<Vec<model::blocklist::Item>> {
    let headers = build_headers(cookie).await?;
    let blocklist_response = get_blocklist_response(client, &headers, pn).await?;
    build_blocklist(&blocklist_response).await
}

async fn write_blocklist_to_json_file(blocklist: &HashSet<model::blocklist::Item>) -> Result<()> {
    let mut blocklist: Vec<&model::blocklist::Item> = Vec::from_iter(blocklist);
    blocklist.sort_by(|a, b| a.mid.cmp(&b.mid));

    let file = File::create("blocklist.json").await?;
    let mut writer = BufWriter::new(file);
    writer
        .write_all(serde_json::to_string_pretty(&blocklist)?.as_bytes())
        .await?;
    writer.flush().await?;
    Ok(())
}

async fn load_config() -> Result<model::config::Config> {
    let file = File::open("config.json").await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).await?;
    let config: model::config::Config = serde_json::from_str(&buffer)?;
    Ok(config)
}

async fn load_blocklist_set() -> Result<Vec<model::blocklist::Item>> {
    let file = File::open("blocklist.json").await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).await?;
    let config: Vec<model::blocklist::Item> = serde_json::from_str(&buffer)?;
    Ok(config)
}

async fn send_block_request(
    client: &reqwest::Client,
    cookie: &model::config::Cookie,
    mid: &str,
) -> Result<()> {
    let mut headers = build_headers(cookie).await?;
    headers.insert("content-type", "application/x-www-form-urlencoded".parse()?);

    let res = client
        .post("https://api.bilibili.com/x/relation/modify")
        .headers(headers)
        .body(format!(
            "fid={}&act=5&re_src=11&csrf={}",
            mid, cookie.bili_jct
        ))
        .send()
        .await?
        .text()
        .await?;
    tracing::info!("{:#?}", res);

    Ok(())
}

pub async fn pull() -> Result<()> {
    let config = load_config().await?;

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()?;

    let mut blocklist_set = HashSet::new();
    for cookie in &config.cookie_list {
        let mut pn = 1;
        loop {
            match get_blocklist(&client, cookie, pn).await {
                Ok(blocklist) => {
                    if !blocklist.is_empty() {
                        blocklist_set.extend(blocklist);
                    } else {
                        break;
                    }
                }
                Err(_) => break,
            }
            pn += 1;
        }
    }

    write_blocklist_to_json_file(&blocklist_set).await?;

    Ok(())
}

pub async fn push() -> Result<()> {
    let config = load_config().await?;

    let client = reqwest::Client::builder()
        .user_agent("Chrome/96.0.4664.110")
        .build()?;

    let blocklist_set = load_blocklist_set().await?;

    for cookie in config.cookie_list {
        for blocklist_item in &blocklist_set {
            send_block_request(&client, &cookie, &blocklist_item.mid).await?;
            sleep(Duration::from_millis(config.sleep_ms)).await;
        }
    }

    Ok(())
}
