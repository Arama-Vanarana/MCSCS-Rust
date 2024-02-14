use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

async fn call_aria2_rpc(method: &str, params: Value, id: &str) -> Result<Value, reqwest::Error> {
    // 合并参数
    let mut merged_params = json!(["token:MCSCS"]).as_array().unwrap_or(&vec![]).clone();
    merged_params.extend(params.as_array().unwrap_or(&vec![]).iter().cloned());

    let client = Client::new();
    let mut started = false;
    // 发送请求
    loop {
        match client
            .post("http://localhost:6800/jsonrpc") // Change URL accordingly
            .json(&json!({
                "jsonrpc": "2.0",
                "method": method,
                "id": id,
                "params": serde_json::to_value(merged_params.clone()).unwrap(),
            }))
            .send()
            .await
        {
            Ok(response) => {
                // 获取响应中的 "result" 字段
                let result = response.json::<Value>().await?;
                let result_value = result["result"].clone();
                return Ok(result_value);
            }
            Err(e) => {
                if started == false {
                    let aria2_current_dir = std::env::current_dir().unwrap().join("aria2");
                    std::fs::create_dir_all(aria2_current_dir.join("logs"))
                        .expect("Failed to create logs folder");
                    let execute = aria2_current_dir.join("aria2c.exe");
                    std::process::Command::new(execute)
                        .current_dir(aria2_current_dir)
                        .args([
                            "--dir=downloads",
                            "--input-file=aria2.session",
                            "--save-session=aria2.session",
                            "--conf-path=aria2.conf",
                            &format!("--log=logs/{}.log", Local::now().format("%Y%m%d%H%M")),
                        ])
                        .spawn()
                        .expect("Failed to start aria2c");
                    started = true;
                } else {
                    println!("Error: {}", e);
                    return Err(e);
                }
            }
        }
    }
}

fn format_time(hours: u64, minutes: u64, seconds: u64) -> String {
    if hours > 0 {
        format!("ETA:{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("ETA:{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("ETA:{}s", seconds)
    } else {
        String::new()
    }
}

fn time_remaining(speed: u64, completed: u64, total: u64) -> String {
    if speed == 0 {
        return String::new();
    }

    let remaining_time_secs = (total - completed) / speed;

    if remaining_time_secs == 0 {
        return String::new();
    }

    let remaining_hours = remaining_time_secs / 3600;
    let remaining_minutes = (remaining_time_secs % 3600) / 60;
    let remaining_seconds = remaining_time_secs % 60;

    Some(format_time(
        remaining_hours,
        remaining_minutes,
        remaining_seconds,
    ))
    .unwrap_or(String::new())
}

// 定义一个函数来格式化文件大小和下载速度
fn format_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut index = 0;
    let mut size = size as f64;
    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }
    format!("{:.2}{}", size, units[index])
}

// 定义一个函数来下载文件并显示进度条
pub async fn download(url: &str) -> Result<String, reqwest::Error> {
    // 调用 aria2.addUri 来添加下载任务，并获取 GID
    let value = call_aria2_rpc("aria2.addUri", json!([[url]]), "add").await?;
    let gid = value.as_str().unwrap();
    // 创建一个进度条
    let pb = ProgressBar::new(0);
    // 设置进度条的样式
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:50}] {msg}")
            .unwrap()
            .progress_chars("=>"),
    );
    // 循环更新进度条
    loop {
        // 调用 aria2.tellStatus 来获取下载状态
        let status = call_aria2_rpc("aria2.tellStatus", json!([gid]), "status").await?;
        // 获取已完成的大小，总大小，下载速度，剩余时间等信息
        let completed = status["completedLength"]
            .as_str()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap();
        let total = status["totalLength"]
            .as_str()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap();
        let speed = status["downloadSpeed"]
            .as_str()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap();
        // 设置进度条的最大值
        pb.set_length(total);
        // 设置进度条的当前值
        pb.set_position(completed);
        // 设置进度条的下载速度和剩余时间
        pb.set_message(format!(
            "{}/s {}/{} CN:{} {}",
            format_size(speed),
            format_size(completed),
            format_size(total),
            status["connections"]
                .as_str()
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap(),
            time_remaining(speed, completed, total),
        ));
        if status["status"].as_str().unwrap_or("") == "complete" {
            let file_path = if let Some(path) = status["files"][0]["path"].as_str() {
                path.to_string()
            } else {
                "unknown".to_string()
            };
            pb.finish_with_message(format!("Download complete: {}", file_path));
            return Ok(file_path);
        }
        std::thread::sleep(Duration::from_millis(300));
    }
}
