use rayon::prelude::*;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn is_junction(path: &Path) -> bool {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        metadata.file_type().is_symlink()
    } else {
        false
    }
}

fn search_file(path: &Path, java_paths: &Arc<Mutex<Vec<Value>>>) {
    if let Ok(entries) = fs::read_dir(path) {
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
                            java_paths.push(json!({"version": get_java_version(java_path).unwrap(), "path": java_path}));
                        }
                    }
                }
            });
    }
}

pub fn get_java_version(java_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new(java_path)
        .args(["-version", "2> &1"])
        .output()
        .unwrap();
    let output_str = String::from_utf8_lossy(&output.stderr);
    let re = regex::Regex::new(r"(\d+)(?:\.(\d+))?(?:\.(\d+))?(?:[._](\d+))?(?:-(.+))?")
        .expect("Invalid regex");
    // 在输出中查找第一个匹配项
    if let Some(captured) = re.captures(&output_str) {
        if let Some(first_match) = captured.get(0) {
            Ok(first_match.as_str().to_string())
        } else {
            Err("No match found".into())
        }
    } else {
        Err("No match found".into())
    }
}

// 似乎四秒即可寻找完毕
pub fn detect_java() -> Value {
    let java_paths = Arc::new(Mutex::new(Vec::new()));

    (b'A'..=b'Z').into_par_iter().for_each(|drive| {
        let root_path = PathBuf::from(format!("{}:\\", drive as char));
        search_file(&root_path, &java_paths);
    });
    let java_paths = json!(*java_paths.lock().unwrap());
    java_paths
}
