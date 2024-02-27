use std::{env, fs};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use log::debug;
use rayon::prelude::*;
use regex::Regex;
use serde_json::{json, Value};

fn search_file(path: &Path, java_paths: &Arc<Mutex<Vec<Value>>>) {
    if let Ok(entries) = fs::read_dir(path) {
        entries
            .filter_map(|entry| entry.ok())
            .par_bridge()
            .for_each(|entry| {
                let file_path = entry.path();
                let file_name = match file_path.file_name() {
                    Some(name) => name.to_string_lossy().into_owned(),
                    None => return,
                };

                if file_name.contains('$')
                    || file_name.contains('{')
                    || file_name.contains('}')
                    || file_name.contains('_')
                {
                    return;
                }

                if let Ok(metadata) = entry.metadata() {
                    if metadata.file_type().is_symlink() {
                        return;
                    }
                } else {
                    return;
                }

                if file_path.is_dir() && !["Windows", "AppData"].contains(&file_name.as_str()) {
                    search_file(&file_path, java_paths);
                } else if file_name == "java.exe" {
                    if let Some(java_path) = file_path.to_str() {
                        let version = get_java_version(java_path);
                        if version != *"unknown" {
                            let mut java_paths = java_paths.lock().unwrap();
                            java_paths.push(json!({"version": version, "path": java_path}));
                        }
                    }
                }
            });
    }
}

pub fn get_java_version(java_path: &str) -> String {
    let output = Command::new(java_path)
        .args(["-version", "2>&1"])
        .output()
        .unwrap();
    let output_str = String::from_utf8_lossy(&output.stderr);
    let re = Regex::new(r"(\d+)(?:\.(\d+))?(?:\.(\d+))?(?:[._](\d+))?(?:-(.+))?")
        .expect("正则表达式不正确");
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

pub fn detect_java() -> Value {
    let java_paths = Arc::new(Mutex::new(Vec::new()));

    let drives: Vec<_> = (b'A'..=b'Z')
        .map(|drive| format!("{}:\\", drive as char))
        .collect();

    drives.into_par_iter().for_each(|drive| {
        let root_path = PathBuf::from(drive);
        search_file(&root_path, &java_paths);
    });

    let java = json!(*java_paths.lock().unwrap());
    debug!("成功遍历到的Java环境: {java}");
    java
}

pub fn save_java_lists(java: &Value) {
    let file = fs::File::create(
        env::current_dir()
            .unwrap()
            .join("MCSCS")
            .join("servers")
            .clone()
            .join("java.json"),
    )
        .expect("创建servers/java.json错误");
    debug!("已保存到MCSCS/servers/java.json: {java}");
    serde_json::to_writer_pretty(file, &json!({"data": java})).expect("写入servers/java.json错误");
}

pub fn load_java_lists() -> Value {
    let mut file = fs::File::open(
        env::current_dir()
            .unwrap()
            .join("MCSCS")
            .join("servers")
            .join("java.json"),
    )
        .expect("读取MCSCS/servers/java.json失败");

    // 读取文件内容到字符串中
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("读取MCSCS/servers/java.json失败");

    let java = serde_json::from_str::<Value>(&json_data).expect("无法解析JSON");
    debug!("从MCSCS/servers/java.json加载到的Java环境: {java}");
    java["data"].clone()
}
