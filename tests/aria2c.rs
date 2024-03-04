use mcscs::{aria2c::call_aria2c_rpc, pages::init};
use serde_json::json;

#[tokio::test]
async fn test_get_aria2c_version() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    match call_aria2c_rpc("aria2.addUri", json!([["https://speedtest.zju.edu.cn/1000M"]]), "test").await {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
