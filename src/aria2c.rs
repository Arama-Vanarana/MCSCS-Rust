/*
 * Copyright (c) 2024 Arama.
 */

use std::path::PathBuf;
use std::process::Command;
use std::{env, error::Error, thread::sleep, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use jsonrpc::Client;
use log::{info, trace, warn};
use serde_json::{json, Value};

pub fn call_aria2c_rpc(method: &str, params: Value) -> Result<Value, Box<dyn Error>> {
    let mut merge_params = vec![json!("token:MCSCS")];
    merge_params.append(params.clone().as_array_mut().expect("call_aria2c_rpc()"));
    let client = Client::simple_http("http://127.0.0.1:6800/jsonrpc", None, None)?;
    let args = jsonrpc::arg(json!(merge_params));
    let request = client.build_request(method, Some(&args));
    let response = Client::send_request(&client, request)?;
    Ok(json!(response.result))
}

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
    let gid_json = call_aria2c_rpc("aria2.addUri", json!([[url]]))?;
    let gid = gid_json.as_str().unwrap_or_default();
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
        )?;
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
            let file_path = call_aria2c_rpc("aria2.tellStatus", json!([gid, ["files"]]))?["files"]
                [0]["path"]
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
            if call_aria2c_rpc("aria2.unpause", json!([gid]))? == gid_json {
                info!("下载任务已重新启动");
            } else {
                return Err("下载错误".into());
            }
        }
        sleep(Duration::from_millis(175));
    }
}

#[cfg(target_os = "windows")]
pub async fn install_aria2c() {
    use std::{
        fs::{self, File},
        io,
    };

    use zip::ZipArchive;

    if get_aria2c_execute().is_err() {
        let path = env::current_dir()
            .expect("install_aria2c()")
            .join("MCSCS")
            .join("aria2c");
        println!("开始下载Aria2c");
        let url = {
            let request = reqwest::Client::new()
                .get("https://api.github.com/repos/aria2/aria2/releases")
                .header("User-Agent", "MCSCS/1.0");
            let response = request.send().await.expect("install_aria2c()");
            let response = response.json::<Value>().await.expect("install_aria2c()");
            let mut result = String::new();
            for data in response[0]["assets"].as_array().expect("install_aria2c()") {
                let name = data["name"].as_str().expect("install_aria2c()");
                if name.contains("win") && name.contains("64bit") {
                    result = data["browser_download_url"]
                        .as_str()
                        .expect("install_aria2c()")
                        .to_string();
                }
            }
            result
        };

        let response = reqwest::get(url).await.expect("install_aria2c()");
        if !response.status().is_success() {
            panic!("install_aria2c()");
        }

        let mut file = File::create(path.join("aria2c.zip")).expect("install_aria2c()");
        io::copy(
            &mut response.bytes().await.expect("install_aria2c()").as_ref(),
            &mut file,
        )
        .expect("install_aria2c()");
        println!("Aria2c下载完成");

        let file = File::open(path.join("aria2c.zip")).expect("install_aria2c()");
        let mut archive = ZipArchive::new(file).expect("install_aria2c()");

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).expect("install_aria2c()");
            if file.name().ends_with("aria2c.exe") {
                let mut outfile = File::create(&path.join("aria2c.exe")).expect("install_aria2c()");
                io::copy(&mut file, &mut outfile).expect("install_aria2c()");
            }
        }
        fs::remove_file(path.join("aria2c.zip")).expect("install_aria2c()");
        println!("解压完成");
    }
}

#[cfg(not(target_os = "windows"))]
pub async fn install_aria2c() {
    if get_aria2c_execute().is_err() {
        panic!(
            "aria2c未安装, 请安装后再次运行本程序:
Ubuntu/Debian:
sudo apt update
sudo apt install aria2"
        );
    }
}

pub fn get_aria2c_execute() -> Result<PathBuf, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    let execute = "aria2c.exe";
    #[cfg(not(target_os = "windows"))]
    let execute = "aria2c";

    // 先检测有没有内置的aria2c
    let aria2c_path = env::current_dir()
        .expect("get_aria2c_execute()")
        .join("MCSCS")
        .join("aria2c")
        .join(execute);
    if aria2c_path.exists() {
        let mut process = Command::new(&aria2c_path);
        process.arg("-v");
        if process.output()?.status.success() {
            trace!("find -> {}", aria2c_path.display());
            return Ok(aria2c_path);
        }
    }

    // 检测PATH环境变量里有没有
    if let Ok(path_env) = env::var("PATH") {
        let paths = env::split_paths(&path_env).collect::<Vec<PathBuf>>();
        for path in paths {
            let aria2c_path = path.join(execute);
            if aria2c_path.exists() {
                return Ok(aria2c_path);
            }
        }
    }

    // 没找到
    Err("未找到aria2c可执行文件".into())
}
