use serde_json::json;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir().unwrap();
    match crate::library::controllers::aria2::call_aria2_rpc("aria2.getVersion", json!([]), "check")
        .await
    {
        Ok(version) => {
            println!(
                "Aria2已启动: {}",
                version["version"].as_str().unwrap_or("unknown")
            );
        }
        Err(e) => {
            if e.is_timeout() {
                let aria2_current_dir = current_dir.join("aria2");
                std::fs::create_dir_all(aria2_current_dir.join("logs"))
                    .expect("创建logs文件夹失败");
                std::fs::create_dir_all(aria2_current_dir.join("downloads"))
                    .expect("创建downloads文件夹失败");
                let execute = aria2_current_dir.join("aria2c.exe");
                std::process::Command::new(execute)
                    .args([
                        &format!(
                            "--dir={}",
                            aria2_current_dir.join("downloads").to_str().unwrap()
                        ),
                        &format!(
                            "--log={}",
                            aria2_current_dir
                                .join("logs")
                                .join(format!("{}.log", chrono::Local::now().format("%Y%m%d%H%M")))
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
                    ])
                    .spawn()
                    .expect("Aria2c启动失败!");
                println!("Aria2c启动成功!");
            } else {
                return Err(e.into());
            }
        }
    }
    let servers_current_dir = current_dir.join("servers");
    std::fs::create_dir_all(&servers_current_dir).expect("创建servers文件夹失败");
    match std::fs::metadata(servers_current_dir.join("java.json")) {
        Ok(_) => {}
        Err(_) => crate::library::controllers::java::save_java_lists(
            crate::library::controllers::java::detect_java(),
        ),
    }
    match std::fs::metadata(servers_current_dir.join("config.json")) {
        Ok(_) => {}
        Err(_) => {
            let file = std::fs::File::create(servers_current_dir.join("config.json"))
                .expect("创建config.json错误");
            serde_json::to_writer_pretty(file, &json!({"data": {}})).expect("config.json错误");
        }
    }
    Ok(())
}
