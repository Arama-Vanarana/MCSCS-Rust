use std::{error::Error, thread::sleep, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use jsonrpc::Client;
use log::{info, warn};
use serde_json::json;

/// 将Bytes单位转换为对应的单位, 例如: 1000000000 -> 1G
fn format_size(size: u64) -> String {
    let units = ["Bytes", "KiB", "MiB", "GiB", "TiB"];
    let mut index = 0;
    let mut size = size as f64;
    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }
    format!("{size:.2}{}", units[index])
}

/// 使用aria2c下载文件
///
/// # 使用
/// ```
/// use mcscs::aria2c::download;
/// if let Ok(file_path) = download("https://example.com/file.zip") {
///     // 处理文件路径
/// }
/// ```
pub fn download(url: &str) -> Result<String, Box<dyn Error>> {
    // 调用 aria2.addUri 来添加下载任务，并获取 GID
    let client =
        Client::simple_http("http://127.0.0.1:6800/jsonrpc", None, None).expect("download()");
    let args = jsonrpc::arg(json!(["token:MCSCS", [url]]));
    let request = client.build_request("aria2.addUri", Some(&args));
    let response = Client::send_request(&client, request);
    let gid_json = json!(response.expect("download()").result);
    let gid = gid_json.as_str().unwrap_or_default();
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:.green}] {msg}")
            .unwrap()
            .progress_chars("=> "),
    );
    loop {
        let args = jsonrpc::arg(json!([
            "token:MCSCS",
            gid,
            [
                "completedLength",
                "totalLength",
                "downloadSpeed",
                "connections",
                "status",
            ]
        ]));
        let request = client.build_request("aria2.tellStatus", Some(&args));
        let response = Client::send_request(&client, request).expect("download()");
        let status = json!(response.result.unwrap_or_default());
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
            let args = jsonrpc::arg(json!(["token:MCSCS", gid, ["files"]]));
            let request = client.build_request("aria2.tellStatus", Some(&args));
            let response = Client::send_request(&client, request).expect("download()");
            let file_path = json!(response.result.unwrap_or_default())["files"][0]["path"].take();
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
            let args = jsonrpc::arg(json!(["token:MCSCS", gid]));
            let request = client.build_request("aria2.unpause", Some(&args));
            let response = Client::send_request(&client, request).expect("download()");
            if json!(response.result.unwrap_or_default()) == gid_json {
                info!("下载任务已重新启动");
            }
        }
        sleep(Duration::from_millis(175));
    }
}
