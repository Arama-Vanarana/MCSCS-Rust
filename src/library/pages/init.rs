use log::{debug, error, info, warn, LevelFilter};
use log4rs::{self, append::file::FileAppender, config::{Appender, Logger, Root}, encode::pattern::PatternEncoder, Config};
use serde_json::json;
use std::{env, error::Error, fs, path::PathBuf, process};

use crate::library::controllers::{aria2c, java::{detect_java, save_java_lists}};

pub async fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir().unwrap().join("MCSCS");
    let log_path = current_dir
        .join("logs")
        .join(chrono::Local::now().format("%Y%m%d%H%M").to_string());
    fs::create_dir_all(&log_path).expect("创建logs文件夹失败");
    init_log(&current_dir, &log_path);
    init_aria2(&current_dir, &log_path).await;
    init_servers(&current_dir);
    Ok(())
}

fn init_log(current_dir: &PathBuf, log_path: &PathBuf) {
    fs::create_dir_all(current_dir.join("logs")).expect("创建logs文件夹失败");
    // 文件输出
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.6f)} [{l}] [{f}:{L}] {m}{n}",
        )))
        .build(log_path.join("client.log"))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(
            Logger::builder()
                .appender("file")
                .additive(false)
                .build("app", LevelFilter::Trace),
        )
        .build(
            Root::builder()
                .appender("file")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config).expect("log4rs初始化失败");
}

async fn init_aria2(current_dir: &PathBuf, log_path: &PathBuf) {
    match aria2c::call_aria2c_rpc(
        "aria2.getVersion",
        json!([]),
        "check",
    )
    .await
    {
        Ok(version) => {
            info!(
                "aria2c已启动: {}",
                version["version"].as_str().unwrap_or("unknown")
            );
        }
        Err(e) => {
            if e.is_timeout() {
                warn!("检测到aria2c似乎未开启,正在开启aria2c中...");
                fs::create_dir_all(current_dir.join("downloads"))
                    .expect("创建MCSCS/downloads文件夹失败");
                let execute = current_dir.join("aria2c").join("aria2c.exe");
                let args = [
                    &format!("--dir={}", current_dir.join("downloads").display()),
                    &format!("--log={}", log_path.join("aria2c.log").display()),
                    "--enable-rpc=true",
                    "--rpc-listen-port=6800",
                    "--rpc-max-request-size=10M",
                    "--rpc-secret=MCSCS",
                    &format!(
                        "--conf-path={}",
                        current_dir.join("aria2c").join("aria2c.conf").display()
                    ),
                ];
                debug!("aria2c参数: {}", json!(args));
                process::Command::new(execute)
                    .args(args)
                    .spawn()
                    .expect("aria2c启动失败!");
                info!("aria2c启动成功!");
            } else {
                error!("aria2c开启失败: {e}");
            }
        }
    }
}

fn init_servers(current_dir: &PathBuf) {
    let servers_current_dir = current_dir.join("servers");
    fs::create_dir_all(&servers_current_dir).expect("创建MCSCS/servers文件夹失败");
    match fs::metadata(servers_current_dir.join("java.json")) {
        Ok(_) => info!("MCSCS/servers/java.json存在"),
        Err(_) => save_java_lists(
            &detect_java(),
        ),
    }
    match fs::metadata(servers_current_dir.join("config.json")) {
        Ok(_) => info!("MCSCS/servers/config.json存在"),
        Err(_) => {
            let file = fs::File::create(servers_current_dir.join("config.json"))
                .expect("创建servers/config.json错误");
            serde_json::to_writer_pretty(file, &json!({"data": {}}))
                .expect("MCSCS/servers/config.json错误");
        }
    }
}
