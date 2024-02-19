use rayon::prelude::*;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn is_junction(path: &Path) -> bool {
    if let Ok(metadata) = std::fs::symlink_metadata(path) {
        metadata.file_type().is_symlink()
    } else {
        false
    }
}

fn search_file(path: &Path, java_paths: &Arc<Mutex<Vec<Value>>>) {
    if let Ok(entries) = std::fs::read_dir(path) {
        entries
            .filter_map(|entry| entry.ok())
            .par_bridge()
            .for_each(|entry| {
                let file_path = entry.path();
                if is_junction(&file_path) {
                    return;
                }

                if let Some(file_name) = file_path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.contains("$")
                            || file_name_str.contains("{")
                            || file_name_str.contains("}")
                            || file_name_str.contains("_")
                        {
                            return;
                        }
                    }
                }
                if file_path.is_dir()
                    && !["Windows", "AppData"]
                        .contains(&file_path.file_name().unwrap().to_str().unwrap())
                {
                    search_file(&file_path, java_paths);
                } else if let Some(file_name) = file_path.file_name() {
                    if file_name == "java.exe" {
                        if let Some(java_path) = file_path.to_str() {
                            let mut java_paths = java_paths.lock().unwrap();
                            let version = get_java_version(java_path);
                            if version != "unknown".to_string() {
                                java_paths.push(json!({"version": version, "path": java_path}));
                            }
                        }
                    }
                }
            });
    }
}

pub fn get_java_version(java_path: &str) -> String {
    let output = std::process::Command::new(java_path)
        .args(["-version", "2>&1"])
        .output()
        .unwrap();
    let output_str = String::from_utf8_lossy(&output.stderr);
    let re = regex::Regex::new(r"(\d+)(?:\.(\d+))?(?:\.(\d+))?(?:[._](\d+))?(?:-(.+))?")
        .expect("Invalid regex");
    // 在输出中查找第一个匹配项
    if let Some(captured) = re.captures(&output_str) {
        if let Some(first_match) = captured.get(0) {
            first_match.as_str().to_string()
        } else {
            "unknown".to_string()
        }
    } else {
        "unknown".to_string()
    }
}

// 似乎四秒即可寻找完毕
pub fn detect_java() -> Value {
    let java_paths = Arc::new(Mutex::new(Vec::new()));

    (b'A'..=b'Z').into_par_iter().for_each(|drive| {
        let root_path = PathBuf::from(format!("{}:\\", drive as char));
        search_file(&root_path, &java_paths);
    });
    let java = json!(*java_paths.lock().unwrap());
    java
}

pub fn save_java_lists(java: Value) {
    let current_dir = std::env::current_dir().unwrap().join("servers");
    let file =
        std::fs::File::create(current_dir.clone().join("java.json")).expect("创建java.json错误");
    serde_json::to_writer_pretty(file, &json!({"data": java})).expect("写入java.json错误");
}

pub fn load_java_lists() -> Value {
    use std::io::Read;
    let mut file = std::fs::File::open(
        std::env::current_dir()
            .unwrap()
            .join("servers")
            .join("java.json"),
    )
    .expect("读取java.json失败");

    // 读取文件内容到字符串中
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("读取java.json失败");

    // 将 JSON 字符串反序列化为 MyData 结构体实例
    let data: serde_json::Value = serde_json::from_str(&json_data).expect("无法解析JSON");
    data["data"].clone()
}
