use log::{debug, info};
use log4rs;
use serde_json::json;
use std::path::PathBuf;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir().unwrap();
    let now_time = chrono::Local::now();
    init_log(&current_dir, &now_time);
    init_aria2(&current_dir, &now_time)
        .await
        .expect("Aria2c启动失败!");
    init_servers(&current_dir);
    Ok(())
}

fn init_log(
    current_dir: &std::path::PathBuf,
    time: &chrono::prelude::DateTime<chrono::prelude::Local>,
) {
    std::fs::create_dir_all(current_dir.join("logs")).expect("创建logs文件夹失败");
    // 文件输出
    let file = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.6f)} [{l}] [{f}:{L}] {m}{n}",
        )))
        .build(PathBuf::from(format!(
            "logs/{}.log",
            time.format("%Y%m%d%H%M")
        )))
        .unwrap();

    let config = log4rs::Config::builder()
        .appender(log4rs::config::Appender::builder().build("file", Box::new(file)))
        .logger(
            log4rs::config::Logger::builder()
                .appender("file")
                .additive(false)
                .build("app", log::LevelFilter::Debug),
        )
        .build(
            log4rs::config::Root::builder()
                .appender("file")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).expect("log4rs初始化失败");
}

async fn init_aria2(
    current_dir: &std::path::PathBuf,
    time: &chrono::prelude::DateTime<chrono::prelude::Local>,
) -> Result<(), Box<dyn std::error::Error>> {
    match crate::library::controllers::aria2::call_aria2_rpc("aria2.getVersion", json!([]), "check")
        .await
    {
        Ok(version) => {
            info!(
                "Aria2c已启动: {}",
                version["version"].as_str().unwrap_or("unknown")
            );
            Ok(())
        }
        Err(e) => {
            if e.is_timeout() {
                let aria2_current_dir = current_dir.join("aria2");
                std::fs::create_dir_all(aria2_current_dir.join("logs"))
                    .expect("创建aria2/logs文件夹失败");
                std::fs::create_dir_all(aria2_current_dir.join("downloads"))
                    .expect("创建aria2/downloads文件夹失败");
                let execute = aria2_current_dir.join("aria2c.exe");
                let args = [
                    &format!(
                        "--dir={}",
                        aria2_current_dir.join("downloads").to_str().unwrap()
                    ),
                    &format!(
                        "--log={}",
                        aria2_current_dir
                            .join("logs")
                            .join(format!("{}.log", time.format("%Y%m%d%H%M")))
                            .to_str()
                            .unwrap(),
                    ),
                    "--enable-rpc=true",
                    "--rpc-listen-port=6800",
                    "--rpc-max-request-size=10M",
                    "--rpc-secret=MCSCS",
                    &format!(
                        "--conf-path={}",
                        aria2_current_dir.join("aria2.conf").to_str().unwrap()
                    ),
                ];
                debug!("aria2c参数: {}", json!(args));
                std::process::Command::new(execute)
                    .args(args)
                    .spawn()
                    .expect("Aria2c启动失败!");
                info!("Aria2c启动成功!");
                return Ok(());
            }
            Err(e.into())
        }
    }
}

fn init_servers(current_dir: &std::path::PathBuf) {
    let servers_current_dir = current_dir.join("servers");
    std::fs::create_dir_all(&servers_current_dir).expect("创建servers文件夹失败");
    match std::fs::metadata(servers_current_dir.join("java.json")) {
        Ok(_) => info!("java.json存在"),
        Err(_) => crate::library::controllers::java::save_java_lists(
            crate::library::controllers::java::detect_java(),
        ),
    }
    match std::fs::metadata(servers_current_dir.join("config.json")) {
        Ok(_) => info!("config.json存在"),
        Err(_) => {
            let file = std::fs::File::create(servers_current_dir.join("config.json"))
                .expect("创建servers/config.json错误");
            serde_json::to_writer_pretty(file, &json!({"data": {}})).expect("config.json错误");
        }
    }
}
