use reqwest::Error;
use serde_json::{Map, Value};

async fn get_api_value(url: &str) -> Result<Value, Error> {
    let response = reqwest::get(url).await?;
    let json = response.json::<Value>().await?;
    return Ok(json);
}

pub async fn get_fastmirror_value() -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
    let data = get_api_value("https://download.fastmirror.net/api/v3").await?;
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
    return Ok(name_map);
}

pub async fn get_fastmirror_builds_value(
    core: &str,
    version: &str,
) -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
    let data = get_api_value(
        format!(
            "https://download.fastmirror.net/api/v3/{}/{}?offset=0&limit=25",
            core, version
        )
        .as_str(),
    )
    .await?;

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

    return Ok(name_map);
}
