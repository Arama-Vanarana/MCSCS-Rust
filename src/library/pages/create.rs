use std::{fs, path::PathBuf};

use crate::library::controllers::{
    input,
    java::{detect_java, get_java_version, load_java_lists, save_java_lists},
};
use log::debug;
use serde_json::{json, Value};

fn java() -> Value {
    loop {
        let mut java_info = load_java_lists();
        let mut java_lens = 0;
        if let Value::Array(array) = &java_info {
            for (index, item) in array.iter().enumerate() {
                if let Value::Object(java) = item {
                    debug!("{}", json!(java));
                    if let Some(version) = java["version"].as_str() {
                        if let Some(path) = java["path"].as_str() {
                            println!("{}: {}({})", index, version, path);
                        } else {
                            println!("Path不存在！");
                        }
                    } else {
                        println!("Version不存在！");
                    }
                    java_lens += 1;
                }
            }
        }
        println!("{}: 重新检测Java环境", java_lens);
        println!("{}: 手动输入java.exe路径", java_lens + 1);
        let input_value = input("请选择使用的Java环境");
        debug!("{input_value}");
        match input_value.parse::<usize>() {
            Ok(choice) => {
                debug!("用户选择了{choice}");
                if choice == java_lens {
                    save_java_lists(&detect_java());
                    println!("刷新成功!");
                    continue;
                }
                if choice == java_lens + 1 {
                    let java_path = PathBuf::from(input("请输入java.exe的路径"));
                    if let Ok(metadata) = fs::metadata(&java_path) {
                        if metadata.is_file() {
                            let java_ver = get_java_version(&java_path.display().to_string());
                            if java_ver == "unknown".to_string() {
                                println!("Java无效!");
                                continue;
                            }
                            let java = json!({
                                "path": java_path.display().to_string(),
                                "version": java_ver                            });
                            if let Value::Array(ref mut arr) = java_info {
                                arr.push(java.clone());
                            }
                            save_java_lists(&java_info);
                            return java;
                        }
                        println!("Java不存在!");
                        continue;
                    }
                }
                if choice > java_lens {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                return java_info[choice].clone();
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
            }
        }
    }
}

pub async fn main() {
    println!("{}", java())
}
