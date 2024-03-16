/*
 * Copyright (c) 2024 Arama.
 */

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{env, error::Error, fs, path::Path, process::Command, time::Duration};

use chrono::Local;
use lazy_static::lazy_static;
use log::{info, trace, warn, LevelFilter};
use log4rs::{
    self,
    append::file::FileAppender,
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::aria2c::{call_aria2c_rpc, get_aria2c_execute};
use crate::java::load_java_lists;
use crate::{
    aria2c::install_aria2c,
    java::{detect_java, save_java_lists},
};

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

/// 初始化页面
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut initialized = INITIALIZED.lock().await;
    if !*initialized {
        let res = {
            let current_dir = env::current_dir()?.join("MCSCS");
            let log_path = current_dir
                .join("logs")
                .join(Local::now().format("%Y%m%d%H%M").to_string());
            fs::create_dir_all(&log_path)?;
            init_log(&log_path)?;
            init_aria2(&current_dir, &log_path).await?;
            init_servers(&current_dir)?;
            Ok(())
        };
        match res {
            Ok(_) => {
                *initialized = true;
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else {
        Ok(())
    }
}

/// 初始化日志
fn init_log(log_path: &Path) -> Result<(), Box<dyn Error>> {
    // 文件输出
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.6f)} [{l}] [{f}:{L}] {m}{n}",
        )))
        .build(log_path.join("client.log"))?;

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(
            Logger::builder()
                .appender("file")
                .additive(false)
                .build("app", LevelFilter::Trace),
        )
        .build(Root::builder().appender("file").build(LevelFilter::Trace))?;

    log4rs::init_config(config)?;
    Ok(())
}

// 初始化aria2c
async fn init_aria2(current_dir: &Path, log_path: &Path) -> Result<(), Box<dyn Error>> {
    let aria2c_current_dir = current_dir.join("aria2c");
    install_aria2c().await;
    match call_aria2c_rpc("aria2.getVersion", json!([])) {
        Ok(version) => {
            info!(
                "aria2c已启动: {}",
                json!(version)["version"].as_str().unwrap_or("unknown")
            );
        }
        Err(_) => {
            warn!("检测到aria2c似乎未开启,正在开启aria2c中...");
            fs::create_dir_all(current_dir.join("downloads"))?;
            fs::create_dir_all(&aria2c_current_dir)?;
            if !aria2c_current_dir.join("aria2c.conf").exists() {
                let data = reqwest::get("https://raw.githubusercontent.com/Arama-Vanarana/MCSCS-Rust/main/MCSCS/aria2c/aria2c.conf").await.unwrap().text().await.unwrap();
                fs::write(aria2c_current_dir.join("aria2c.conf"), data)?;
            }
            let mut aria2c = Command::new(get_aria2c_execute()?);
            aria2c.arg(format!("--dir={}", current_dir.join("downloads").display()));
            aria2c.arg(format!("--log={}", log_path.join("aria2c.log").display()));
            aria2c.arg(format!(
                "--conf-path={}",
                current_dir.join("aria2c").join("aria2c.conf").display()
            ));
            aria2c.arg("--enable-rpc=true");
            aria2c.arg("--rpc-listen-port=6800");
            aria2c.arg("--rpc-secret=MCSCS");
            aria2c.arg("--quiet=true");
            trace!("shell <- {}", format!("{:?}", aria2c));
            if aria2c.spawn().is_err() {
                panic!(
                    "aria2c未安装, 请安装后再次运行本程序:
Ubuntu/Debian: 
sudo apt update
sudo apt install aria2"
                );
            }
            sleep(Duration::from_millis(100)).await;
        }
    }
    Ok(())
}

/// 初始化服务器页面相关文件夹和文件
fn init_servers(current_dir: &Path) -> Result<(), Box<dyn Error>> {
    let configs_current_dir = current_dir.join("configs");
    fs::create_dir_all(current_dir.join("servers"))?;
    fs::create_dir_all(&configs_current_dir)?;
    match fs::metadata(configs_current_dir.join("java.json")) {
        Ok(_) => {
            trace!(
                "find -> {}",
                configs_current_dir.join("java.json").display()
            );
            let mut javas = load_java_lists();
            for java in javas.as_array_mut().unwrap() {
                if !PathBuf::from(java["path"].as_str().unwrap()).exists() {
                    save_java_lists(&detect_java());
                    break;
                }
            }
        }
        Err(_) => save_java_lists(&detect_java()),
    }
    match fs::metadata(configs_current_dir.join("servers.json")) {
        Ok(_) => trace!(
            "find -> {}",
            configs_current_dir.join("servers.json").display()
        ),
        Err(_) => {
            let file = fs::File::create(configs_current_dir.join("servers.json"))?;
            serde_json::to_writer_pretty(file, &json!({}))?;
        }
    }
    Ok(())
}
