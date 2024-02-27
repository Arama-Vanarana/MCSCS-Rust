use serde_json::json;

use crate::library::controllers::aria2c::call_aria2c_rpc;

pub mod downloads;
pub mod java;

pub async fn stop_aria2c() {
    match call_aria2c_rpc("aria2.shutdown", json!([]), "shutdown").await {
        Ok(code) => {
            if code.as_str().unwrap() != "OK" {
                eprintln!("关闭aria2c失败: {code}");
            }
        }
        Err(..) => {
            eprintln!("关闭aria2c失败");
        }
    }
}