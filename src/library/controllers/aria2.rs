use serde_json::{json, Value};

async fn call_aria2_rpc(method: &str, params: Value, id: &str) -> Result<Value, reqwest::Error> {
    // 合并参数
    let mut merged_params = json!(["token:MCSCS"]).as_array().unwrap_or(&vec![]).clone();
    merged_params.extend(params.as_array().unwrap_or(&vec![]).iter().cloned());

    let client = reqwest::Client::new();
    // 发送请求
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
            Ok(result_value)
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
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
pub async fn download(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 调用 aria2.addUri 来添加下载任务，并获取 GIDlet mut started = false;
    let value = call_aria2_rpc("aria2.addUri", json!([[url]]), "add").await?;
    let gid = value.as_str().unwrap();
    // 创建一个进度条
    let pb = indicatif::ProgressBar::new(0);
    // 设置进度条的样式
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{bar:50.green}] {msg}")
            .unwrap()
            .progress_chars("=> "),
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

        // 计算剩余时间
        let mut eta = String::new();
        if speed != 0 {
            let remaining_time_secs = (total - completed) / speed;
            if remaining_time_secs != 0 {
                let remaining_hours = remaining_time_secs / 3600;
                let remaining_minutes = (remaining_time_secs % 3600) / 60;
                let remaining_seconds = remaining_time_secs % 60;
                if remaining_hours > 0 {
                    eta = format!(
                        "ETA:{}h {}m {}s",
                        remaining_hours, remaining_minutes, remaining_seconds
                    );
                } else if remaining_minutes > 0 {
                    eta = format!("ETA:{}m {}s", remaining_minutes, remaining_seconds);
                } else if remaining_seconds > 0 {
                    eta = format!("ETA:{}s", remaining_seconds);
                }
            }
        }

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
            eta,
        ));
        let download_status = status["status"].as_str().unwrap_or("error");
        if download_status == "complete" {
            let file_path = status["files"][0]["path"].to_string();
            pb.finish_with_message(format!("Download complete: {}", file_path));
            return Ok(file_path);
        }
        if download_status == "error" || download_status == "removed" {
            return Err("下载错误".into());
        }
        if download_status == "paused" {
            call_aria2_rpc("aria2.unpause", json!([gid]), "unpause").await?;
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}
