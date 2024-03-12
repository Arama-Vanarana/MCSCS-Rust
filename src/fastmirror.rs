use std::{
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use log::error;
use reqwest::{Client, Url};
use serde_json::{json, Map, Value};
use sha1::{Digest, Sha1};

use crate::aria2c::download;

/// 获取FastMirror的返回值
///
/// # 使用
/// ```
/// use mcscs::fastmirror::get_fastmirror_value;
///
/// #[tokio::main]
/// async fn main() {
///     let fastmirror = get_fastmirror_value().await;
///     // ...
/// }
/// ```
///
/// # 返回
/// ```JSON
/// // 类似:
/// {
///     "Mohist": {
///         "name": "Mohist",
///         "tag": "mod",
///         "homepage": "https://mohistmc.com",
///         "recommend": false,
///         "mc_versions": [
///             "1.20.1",
///             "1.19.4",
///             "1.19.3",
///             "1.19.2",
///             "1.18.2",
///             "1.16.5",
///             "1.12.2"
///         ]
///     }
/// }
/// ```
pub async fn get_fastmirror_value() -> Value {
    let url = Url::parse("https://download.fastmirror.net/api/v3").expect("get_fastmirror_value()");
    let response = reqwest::get(url).await.expect("get_fastmirror_value()");
    let data = response
        .json::<Value>()
        .await
        .expect("get_fastmirror_value()");

    let mut name_map = Map::new();
    if let Some(builds) = data["data"].as_array() {
        for entry in builds {
            if let Some(name) = entry["name"].as_str() {
                name_map.insert(name.to_string(), entry.clone());
            }
        }
    }
    json!(name_map)
}

/// 获取FastMirror返回的指定版本的构建版本
///
/// # 使用
/// ```
/// use mcscs::fastmirror::get_fastmirror_builds_value;
///
/// #[tokio::main]
/// async fn main() {
///     let fastmirror = get_fastmirror_builds_value("Mohist", "1.20.1").await;
///     // ...
/// }
/// ```
///
/// # 返回
/// ```Json
/// // 类似:
/// {
///     "build593": {
///         "name": "Mohist",
///         "mc_version": "1.20.1",
///         "core_version": "build593",
///         "update_time": "2024-03-04T06:38:48",
///         "sha1": "bab89293e4aad011852e152d7a7838197fb46bca"
///     },
///     "build592": {
///         "name": "Mohist",
///         "mc_version": "1.20.1",
///         "core_version": "build592",
///         "update_time": "2024-03-04T04:17:06",
///         "sha1": "a00807503741f3035bee70ee24f0ad53452f2d6e"
///     }
/// }
/// ```
pub async fn get_fastmirror_builds_value(core: &str, version: &str) -> Value {
    let mut url = Url::parse(&format!(
        "https://download.fastmirror.net/api/v3/{core}/{version}"
    ))
    .expect("get_fastmirror_builds_value()");
    url.query_pairs_mut()
        .append_pair("offset", "0")
        .append_pair("limit", "25");
    let response = Client::new()
        .get(url)
        .send()
        .await
        .expect("get_fastmirror_builds_value()");
    let data = response
        .json::<Value>()
        .await
        .expect("get_fastmirror_builds_value()");

    let mut name_map = Map::new();
    if let Some(builds) = data["data"]["builds"].as_array() {
        for entry in builds {
            if let Some(name) = entry["core_version"].as_str() {
                name_map.insert(name.to_string(), entry.clone());
            }
        }
    }
    json!(name_map)
}

/// 获取文件的SHA1值
pub fn get_file_sha1(file_path: &Path) -> String {
    let mut buffer = [0u8; 1024];
    let mut file = fs::File::open(file_path).expect("get_file_sha1()");
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

/// 下载服务器核心
///
/// # 使用
/// ```
/// use mcscs::fastmirror::download_server_core;
///
/// #[tokio::main]
/// async fn main() {
///     let path = download_server_core("Mohist", "1.20.1", "build593").await.expect("下载失败");
///     // ...
/// }
/// ```
pub async fn download_server_core(
    core: &str,
    mc_version: &str,
    build_version: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let file_path = download(&format!(
        "https://download.fastmirror.net/download/{core}/{mc_version}/{build_version}"
    ))
    .expect("download_server_core()");
    let fastmirror_sha1 = get_fastmirror_builds_value(core, mc_version).await[build_version]
        ["sha1"]
        .as_str()
        .unwrap()
        .to_string();
    let file_sha1 = get_file_sha1(&PathBuf::from(&file_path));
    if file_sha1 != fastmirror_sha1 {
        error!("Fastmirror: {fastmirror_sha1} != File: {file_sha1}");
        return Err("SHA1".into());
    }
    Ok(PathBuf::from(file_path))
}
