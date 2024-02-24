use log::{error, trace};
use serde_json::{json, Value};

#[doc = r#"# 使用
```
// 获取GID
call_aria2c_rpc("aria2.addUri", json!([["http://example.com/file.torrent"]]), "1").await;
```
"#]
pub async fn call_aria2c_rpc(
    method: &str,
    params: Value,
    id: &str,
) -> Result<Value, reqwest::Error> {
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
    match reqwest::Client::new()
        .post("http://localhost:6800/jsonrpc")
        .json(&args)
        .timeout(std::time::Duration::from_secs(1))
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
    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }
    format!("{:.2}{}", size, units[index])
}

#[doc = "使用Aria2c下载文件"]
pub async fn download(url: String) -> Result<String, Box<dyn std::error::Error>> {
    // 调用 aria2.addUri 来添加下载任务，并获取 GID
    let gid_json = call_aria2c_rpc("aria2.addUri", json!([[url]]), "add").await?;
    let gid = gid_json.as_str().unwrap();
    let pb = indicatif::ProgressBar::new(0);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{bar:50.green}] {msg}")
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
                    "files"
                ]
            ]),
            "status",
        )
        .await?;
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
            let file_path = status["files"][0]["path"]
                .as_str()
                .unwrap()
                .replace("/", "\\");
            pb.finish_with_message(format!("下载完成: {file_path}"));
            return Ok(file_path);
        }
        if download_status == "error" || download_status == "removed" {
            return Err("下载错误".into());
        }
        if download_status == "paused" {
            call_aria2c_rpc("aria2.unpause", json!([gid]), "unpause").await?;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
