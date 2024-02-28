use std::{error::Error, fs, io::Read};

use log::{debug, error};
use serde_json::{json, Map, Value};
use sha1::{Digest, Sha1};

use super::aria2c;

async fn get_api_value(url: &str) -> Value {
    let response = reqwest::get(url).await.expect("FastMirror请求失败");
    let json = response.json::<Value>().await.expect("无法解析JSON");
    debug!("{url} -> {json}");
    json
}

#[doc = "获取FastMirrorAPI的返回值"]
pub async fn get_fastmirror_value() -> Value {
    let data = get_api_value("https://download.fastmirror.net/api/v3").await;

    let mut name_map = Map::new();
    if let Some(builds) = data["data"].as_array() {
        for entry in builds {
            // 获取每个对象内的 "name" 字段值
            if let Some(name) = entry["name"].as_str() {
                // 将 "name" 字段值作为键，对象本身作为值插入到 Map 中
                name_map.insert(name.to_string(), entry.clone());
            }
        }
    }
    json!(name_map)
}

#[doc = "获取FastMirrorAPI的build版本返回值"]
pub async fn get_fastmirror_builds_value(core: &str, version: &str) -> Value {
    let data = get_api_value(&format!(
        "https://download.fastmirror.net/api/v3/{core}/{version}?offset=0&limit=25"
    ))
    .await;

    let mut name_map = Map::new();
    if let Some(builds) = data["data"]["builds"].as_array() {
        for entry in builds {
            // 获取每个对象内的 "name" 字段值
            if let Some(name) = entry["core_version"].as_str() {
                // 将 "name" 字段值作为键，对象本身作为值插入到 Map 中
                name_map.insert(name.to_string(), entry.clone());
            }
        }
    }
    json!(name_map)
}

pub fn get_file_sha1(file_path: &str) -> String {
    let mut buffer = [0u8; 1024];
    let mut file = fs::File::open(file_path).expect("无法打开文件");
    let mut hasher = Sha1::new();

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    hex::encode(hasher.finalize())
}

#[doc = "下载FastMirrorAPI返回的服务器核心"]
pub async fn download_fastmirror_core(
    core: &str,
    mc_version: &str,
    build_version: &str,
) -> Result<String, Box<dyn Error>> {
    let file_path = aria2c::download(format!(
        "https://download.fastmirror.net/download/{core}/{mc_version}/{build_version}"
    ))
    .await
    .expect("下载失败");
    let fastmirror_sha1 = get_fastmirror_builds_value(core, mc_version).await[build_version]
        ["sha1"]
        .as_str()
        .unwrap()
        .to_owned();
    let file_sha1 = get_file_sha1(&file_path);
    if file_sha1 != fastmirror_sha1 {
        error!("SHA1比对失败!FastMirror返回: {fastmirror_sha1}, 但此文件的SHA1为{file_sha1}");
        return Err("SHA1比对失败!".into());
    }
    Ok(file_path)
}
