use serde_json::json;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match crate::library::controllers::aria2::call_aria2_rpc(
        "aria2.getVersion",
        json!([]),
        "test_aria2",
    )
    .await
    {
        Ok(version) => {
            println!("Aria2已启动: {}", version["version"].as_str().unwrap_or("unknown"));
            Ok(())
        }
        Err(e) => {
            if e.is_timeout() {
                let aria2_current_dir = std::env::current_dir().unwrap().join("aria2");
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
                    .expect("启动Aria2c失败!");
                println!("Aria2c启动成功!");
                return Ok(());
            }
            Err(e.into())
        }
    }
}
