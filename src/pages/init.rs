use std::{env, error::Error, fs, path::Path, process::Command};

use chrono::Local;
use jsonrpc::Client;
use lazy_static::lazy_static;
use log::{error, info, trace, warn, LevelFilter};
use log4rs::{
    self,
    append::file::FileAppender,
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use serde_json::json;
use tokio::sync::Mutex;

use crate::java::{detect_java, save_java_lists};

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

/// 初始化页面
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut initialized = INITIALIZED.lock().await;
    if !*initialized {
        let res = {
            let current_dir = env::current_dir().unwrap().join("MCSCS");
            let log_path = current_dir
                .join("logs")
                .join(Local::now().format("%Y%m%d%H%M").to_string());
            fs::create_dir_all(&log_path).expect("创建logs文件夹失败");
            init_log(&log_path);
            init_aria2(&current_dir, &log_path).await;
            init_servers(&current_dir);
            Ok(())
        };
        match res {
            Ok(_) => {
                *initialized = true;
                Ok(())
            }
            Err(err) => {
                error!("{err}");
                Err(err)
            }
        }
    } else {
        Ok(())
    }
}

/// 初始化日志
fn init_log(log_path: &Path) {
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
        .build(Root::builder().appender("file").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).expect("log4rs初始化失败");
}

// 初始化aria2c
async fn init_aria2(current_dir: &Path, log_path: &Path) {
    let client =
        Client::simple_http("http://127.0.0.1:6800/jsonrpc", None, None).expect("init_aria2()");
    let args = jsonrpc::arg(json!([]));
    let request = client.build_request("aria2.getVersion", Some(&args));
    match Client::send_request(&client, request) {
        Ok(version) => {
            info!(
                "aria2c已启动: {}",
                json!(version.result)["version"]
                    .as_str()
                    .unwrap_or("unknown")
            );
        }
        Err(e) => {
            warn!("检测到aria2c似乎未开启,正在开启aria2c中...");
            fs::create_dir_all(current_dir.join("downloads"))
                .expect("创建MCSCS/downloads文件夹失败");
            let execute = current_dir.join("aria2c").join("aria2c.exe");
            let mut aria2c = Command::new(execute);
            aria2c.arg(format!("--dir={}", current_dir.join("downloads").display()));
            aria2c.arg(format!("--log={}", log_path.join("aria2c.log").display()));
            aria2c.arg("--enable-rpc=true");
            aria2c.arg("--rpc-listen-port=6800");
            aria2c.arg("--rpc-max-request-size=10M");
            aria2c.arg("--rpc-secret=MCSCS");
            aria2c.arg(format!(
                "--conf-path={}",
                current_dir.join("aria2c").join("aria2c.conf").display()
            ));
            aria2c.arg("--quiet=true");
            trace!("shell <- {}", format!("{:?}", aria2c));
            if aria2c.spawn().is_ok() {
                info!("aria2c启动成功!");
                return;
            }
            error!("aria2c开启失败: {e}");
        }
    }
}

/// 初始化服务器页面相关文件夹和文件
fn init_servers(current_dir: &Path) {
    let configs_current_dir = current_dir.join("configs");
    fs::create_dir_all(current_dir.join("servers")).expect("创建MCSCS/servers文件夹失败");
    fs::create_dir_all(&configs_current_dir).expect("创建MCSCS/configs文件夹失败");
    match fs::metadata(configs_current_dir.join("java.json")) {
        Ok(_) => info!("MCSCS/servers/java.json存在"),
        Err(_) => save_java_lists(&detect_java()),
    }
    match fs::metadata(configs_current_dir.join("servers.json")) {
        Ok(_) => info!("MCSCS/configs/servers.json存在"),
        Err(_) => {
            let file = fs::File::create(configs_current_dir.join("servers.json"))
                .expect("创建MCSCS/configs/servers.json错误");
            serde_json::to_writer_pretty(file, &json!({}))
                .expect("写入MCSCS/configs/servers.json错误");
        }
    }
}
