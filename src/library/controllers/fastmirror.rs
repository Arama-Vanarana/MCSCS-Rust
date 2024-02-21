use serde_json::{json, Value};

async fn get_api_value(url: &str) -> Value {
    let response = reqwest::get(url).await.expect("FastMirror请求失败");
    let json = response.json::<Value>().await.expect("无法解析JSON");
    json
}

#[doc = "获取FastMirrorAPI的返回值"]
pub async fn get_fastmirror_value() -> Value {
    let data = get_api_value("https://download.fastmirror.net/api/v3").await;
    let mut name_map = serde_json::Map::new();

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
    let data = get_api_value(
        format!("https://download.fastmirror.net/api/v3/{core}/{version}?offset=0&limit=25")
            .as_str(),
    )
    .await;

    let mut name_map = serde_json::Map::new();

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

#[doc = "下载FastMirrorAPI返回的服务器核心"]
pub async fn download_fastmirror_core(core: &str, mc_version: &str, build_version: &str) -> String {
    return crate::library::controllers::aria2c::download(format!(
        "https://download.fastmirror.net/download/{core}/{mc_version}/{build_version}"
    ))
    .await
    .expect("下载失败");
}
