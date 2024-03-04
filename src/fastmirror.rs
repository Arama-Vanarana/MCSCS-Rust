use std::{
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use log::{debug, error};
use serde_json::{json, Map, Value};
use sha1::{Digest, Sha1};

use crate::aria2c::download;

async fn get_api_value(url: &str) -> Value {
    let response = reqwest::get(url).await.expect("FastMirror请求失败");
    let json = response.json::<Value>().await.expect("无法解析JSON");
    debug!("{url} -> {json}");
    json
}

/// 获取FastMirror的返回值
///
/// # 使用
/// ```
/// let fastmirror = get_fastmirror_value();
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

/// 获取FastMirror返回的指定版本的构建版本
///
/// # 使用
/// ```
/// let fastmirror = get_fastmirror_builds_value("Mohist", "1.20.1");
/// ```
///
/// # 返回
/// ```Json
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

/// 获取文件的SHA1值
pub fn get_file_sha1(file_path: &Path) -> String {
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

/// 下载服务器核心
///
/// # 使用
/// ```
/// let path = download_server_core("Mohist", "1.20.1", "build593").expect("下载失败");
/// ```
pub async fn download_server_core(
    core: &str,
    mc_version: &str,
    build_version: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let file_path = download(&format!(
        "https://download.fastmirror.net/download/{core}/{mc_version}/{build_version}"
    ))
    .await
    .expect("下载失败");
    let fastmirror_sha1 = get_fastmirror_builds_value(core, mc_version).await[build_version]
        ["sha1"]
        .as_str()
        .unwrap()
        .to_owned();
    let file_sha1 = get_file_sha1(&PathBuf::from(&file_path));
    if file_sha1 != fastmirror_sha1 {
        error!("SHA1比对失败!FastMirror返回: {fastmirror_sha1}, 但此文件的SHA1为{file_sha1}");
        return Err("SHA1比对失败!".into());
    }
    Ok(PathBuf::from(file_path))
}
