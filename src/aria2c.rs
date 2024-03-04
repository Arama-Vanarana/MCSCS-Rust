use std::{error::Error as StdError, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use log::{error, trace, warn};
use reqwest::{Client, Error as ReqwestError};
use serde_json::{json, Value};
use tokio::time::sleep;

#[doc = r#"# 使用
```
// 获取GID
call_aria2c_rpc("aria2.addUri", json!([["https://example.com/file.torrent"]]), "1").await;
```
"#]
pub async fn call_aria2c_rpc(method: &str, params: Value, id: &str) -> Result<Value, ReqwestError> {
    // 合并参数
    let merged_params = {
        let mut merged_params = json!(["token:MCSCS"]).as_array().unwrap_or(&vec![]).clone();
        merged_params.extend(params.as_array().unwrap_or(&vec![]).iter().cloned());
        json!(merged_params)
    };
    let args = json!({
        "jsonrpc": "2.0",
        "method": method,
        "id": id,
        "params": merged_params,
    });
    trace!("aria2c <- {}", args);
    // 发送请求
    match Client::new()
        .post("http://localhost:6800/jsonrpc")
        .json(&args)
        .timeout(Duration::from_secs(1))
        .send()
        .await
    {
        Ok(response) => {
            // 获取响应中的 "result" 字段
            let result = response.json::<Value>().await?;
            trace!("aria2c -> {result}");
            Ok(result["result"].clone())
        }
        Err(e) => {
            if !e.is_timeout() {
                error!("aria2c -> {e}");
            }
            Err(e)
        }
    }
}

fn format_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut index = 0;
    let mut size = size as f64;
    while size >= 1000.0 && index < units.len() - 1 {
        size /= 1000.0;
        index += 1;
    }
    format!("{:.2}{}", size, units[index])
}

#[doc = "使用Aria2c下载文件"]
pub async fn download(url: &str) -> Result<String, Box<dyn StdError>> {
    // 调用 aria2.addUri 来添加下载任务，并获取 GID
    let gid_json = call_aria2c_rpc("aria2.addUri", json!([[url]]), "add").await?;
    let gid = gid_json.as_str().unwrap();
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:.green}] {msg}")
            .unwrap()
            .progress_chars("=> "),
    );
    loop {
        let status = call_aria2c_rpc(
            "aria2.tellStatus",
            json!([
                gid,
                [
                    "completedLength",
                    "totalLength",
                    "downloadSpeed",
                    "connections",
                    "status",
                ]
            ]),
            "status",
        )
        .await?;
        // 获取已完成的大小，总大小，下载速度，剩余时间等信息
        let completed = status["completedLength"]
            .as_str()
            .unwrap_or("0")
            .parse::<u64>()?;
        let total = status["totalLength"]
            .as_str()
            .unwrap_or("0")
            .parse::<u64>()?;
        let speed = status["downloadSpeed"]
            .as_str()
            .unwrap_or("0")
            .parse::<u64>()?;

        pb.set_length(total);
        pb.set_position(completed);

        let mut eta = String::new();
        if speed != 0 {
            let remaining_time_secs = (total - completed) / speed;
            if remaining_time_secs != 0 {
                let remaining_hours = remaining_time_secs / 3600;
                let remaining_minutes = (remaining_time_secs % 3600) / 60;
                let remaining_seconds = remaining_time_secs % 60;
                if remaining_hours > 0 {
                    eta =
                        format!("ETA:{remaining_hours}h {remaining_minutes}m {remaining_seconds}s");
                } else if remaining_minutes > 0 {
                    eta = format!("ETA:{remaining_minutes}m {remaining_seconds}s");
                } else if remaining_seconds > 0 {
                    eta = format!("ETA:{remaining_seconds}s");
                }
            }
        }

        pb.set_message(format!(
            "{}/s {}/{} CN:{} {eta}",
            format_size(speed),
            format_size(completed),
            format_size(total),
            status["connections"]
                .as_str()
                .unwrap_or("0")
                .parse::<u64>()?
        ));
        let download_status = status["status"].as_str().unwrap_or("error");
        if download_status == "complete" {
            let file_path =
                call_aria2c_rpc("aria2.tellStatus", json!([gid, ["files"]]), "file_path").await?
                    ["files"][0]["path"]
                    .take();
            if let Some(file_path) = file_path.as_str() {
                pb.finish_with_message(format!("下载完成: {file_path}"));
                return Ok(file_path.to_string());
            }
            return Err("下载错误".into());
        }
        if download_status == "error" || download_status == "removed" {
            return Err("下载错误".into());
        }
        if download_status == "paused" {
            warn!("下载任务被暂停, 正在重新启动...");
            call_aria2c_rpc("aria2.unpause", json!([gid]), "unpause").await?;
        }
        sleep(Duration::from_millis(175)).await;
    }
}
