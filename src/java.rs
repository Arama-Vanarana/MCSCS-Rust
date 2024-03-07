use std::{
    env,
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
};

use log::trace;
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

                if file_path.is_dir() && !"Windows".contains(file_name.as_str()) {
                    search_file(&file_path, java_paths);
                } else if file_name == "java.exe" || file_name == "java" {
                    let version = get_java_version(&file_path);
                    if let Ok(version) = version {
                        let mut java_paths = java_paths.lock().unwrap();
                        java_paths.push(json!({"version": version, "path": file_path}));
                    }
                }
            });
    }
}

/// 获取Java的版本
///
/// # 使用
/// ```
/// use std::path::PathBuf;
/// use mcscs::java::get_java_version;
/// let version = get_java_version(&PathBuf::from("JavaPath"));
/// ```
pub fn get_java_version(java_path: &Path) -> Result<String, Box<dyn Error>> {
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
            Ok(first_match.as_str().to_string())
        } else {
            Err("正则表达式匹配失败".into())
        }
    } else {
        Err("正则表达式匹配失败".into())
    }
}

/// 获取计算机所有安装的Java
///
/// # 使用
/// ```
/// let java = detect_java();
/// ```
///
/// # 返回
/// ```JSON
/// {
///     "path": "C:\\Users\\Arama\\scoop\\apps\\dragonwell17-jdk\\17.0.9.0.10-17.0.9\\bin\\java.exe",
///     "version": "17.0.9"
/// },
/// {
///     "path": "C:\\Users\\Arama\\scoop\\apps\\zulu8-jdk\\8.76.0.17\\bin\\java.exe",
///     "version": "1.8.0_402"
/// },
/// {
///     "path": "C:\\Users\\Arama\\scoop\\apps\\zulu8-jdk\\8.76.0.17\\jre\\bin\\java.exe",
///     "version": "1.8.0_402"
/// }
/// ```
pub fn detect_java() -> Value {
    let java_paths = Arc::new(Mutex::new(Vec::new()));

    (b'A'..=b'Z')
        .map(|drive| format!("{}:\\", drive as char))
        .collect::<Vec<String>>()
        .into_par_iter()
        .for_each(|drive| {
            search_file(&PathBuf::from(drive), &java_paths);
        });

    let java = json!(*java_paths.lock().unwrap());
    // trace!(target: "detect_java", "{java}");
    java
}

/// 保存Java环境列表到[`.\MCSCS\configs\java.json`]
pub fn save_java_lists(java: &Value) {
    let file = fs::File::create(
        env::current_dir()
            .unwrap()
            .join("MCSCS")
            .join("configs")
            .clone()
            .join("java.json"),
    )
    .expect("创建configs/java.json错误");
    trace!("MCSCS/configs/java.json <- {java}");
    serde_json::to_writer_pretty(file, &json!(java)).expect("写入configs/java.json错误");
}

/// 从[`.\MCSCS\configs\java.json`]读取Java环境列表
pub fn load_java_lists() -> Value {
    let mut file = fs::File::open(
        env::current_dir()
            .unwrap()
            .join("MCSCS")
            .join("configs")
            .join("java.json"),
    )
    .expect("读取MCSCS/configs/java.json失败");

    // 读取文件内容到字符串中
    let mut java = String::new();
    file.read_to_string(&mut java)
        .expect("读取MCSCS/configs/java.json失败");
    let java = serde_json::from_str::<Value>(&java).expect("无法解析JSON");
    trace!("MCSCS/configs/java.json -> {java}");
    java
}
