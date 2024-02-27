use std::{collections::HashMap, env, fs, path::PathBuf};

use log::debug;
use serde_json::{json, Value};

use crate::library::controllers::{
    fastmirror::{download_fastmirror_core, get_fastmirror_builds_value, get_fastmirror_value},
    input,
    java::{detect_java, get_java_version, load_java_lists, save_java_lists},
    server::{load_servers_lists, save_servers_lists},
};

fn name() -> String {
    let servers = load_servers_lists();
    loop {
        print!("请输入该服务器的名称: ");
        let name = input();
        if let Some(_) = servers.get(&name) {
            println!("输入错误,服务器已存在,请重新输入!");
            continue;
        }
        return name;
    }
}

pub fn java() -> Value {
    loop {
        let mut java_info = load_java_lists();
        let mut java_lens = 0;

        if let Value::Array(array) = &java_info {
            for (index, item) in array.iter().enumerate() {
                if let Value::Object(java) = item {
                    debug!("{}", json!(java));
                    if let Some(version) = java["version"].as_str() {
                        if let Some(path) = java["path"].as_str() {
                            println!("{index}: {version}({path})");
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
        print!("请选择使用的Java环境: ");
        let input_value = input();
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
                    print!("请输入java.exe的路径: ");
                    let java_path = PathBuf::from(input());
                    if let Ok(metadata) = fs::metadata(&java_path) {
                        if metadata.is_file() {
                            let java_ver = get_java_version(&java_path.display().to_string());
                            if java_ver == "unknown".to_string() {
                                println!("Java无效!");
                                continue;
                            }
                            let mut java = json!({"path": java_path.display().to_string(),"version": java_ver});
                            if let Value::Array(ref mut arr) = java_info.take() {
                                arr.push(java.take());
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
                println!("{java_info}, {choice}");
                return java_info[choice].take();
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
            }
        }
    }
}

pub fn encoding() -> String {
    loop {
        println!("0: UTF-8");
        println!("1: GBK");
        println!("2: ANSI");
        println!("3: ASCII");
        print!("请选择需要使用的编码格式: ");
        let input_value = input();
        if input_value == "0" {
            return "utf-8".to_string();
        }
        if input_value == "1" {
            return "gbk".to_string();
        }
        if input_value == "2" {
            return "ansi".to_string();
        }
        if input_value == "3" {
            return "ascii".to_string();
        }
        println!("输入错误,请重新输入!");
    }
}

fn to_bytes(byte: &str) -> u64 {
    let mut num_part = String::new();
    let mut unit_part = String::new();

    // 分离数字部分和单位部分
    for character in byte.chars() {
        if character.is_ascii_digit() {
            num_part.push(character);
        } else if character.is_ascii_alphabetic() {
            unit_part.push(character.to_ascii_uppercase());
        } else {
            return 0; // 如果有除了数字和单位以外的字符, 则返回0
        }
    }
    let mut unit_json = HashMap::new();
    unit_json.insert("G".to_string(), 1000 * 1000 * 1000);
    unit_json.insert("GB".to_string(), 1000 * 1000 * 1000);
    unit_json.insert("M".to_string(), 1000 * 1000);
    unit_json.insert("MB".to_string(), 1000 * 1000);
    unit_json.insert("K".to_string(), 1000);
    unit_json.insert("KB".to_string(), 1000);
    unit_json.insert("B".to_string(), 1);
    unit_json.insert("".to_string(), 1);

    // Get the conversion factor based on the unit
    let conversion_factor = *unit_json.get(&unit_part).unwrap_or(&0);

    // Convert the numeric part and multiply by the factor
    match num_part.parse::<u64>() {
        Ok(num) => num * conversion_factor,
        Err(_) => 0,
    }
}

pub fn xms(xmx: Option<u64>) -> u64 {
    loop {
        print!("请输入Xms(JVM虚拟机初始堆内存)的大小: ");
        let input_value = input();
        let bytes = to_bytes(&input_value);
        if bytes == 0 {
            println!("输入错误,请重新输入!");
            continue;
        }
        if bytes < to_bytes("1M") {
            println!("输入错误,Xms不能小于1M,请重新输入!");
            continue;
        }
        match xmx {
            Some(xmx) => {
                if bytes > xmx {
                    println!("输入错误,Xms不能大于Xmx,请重新输入!");
                    continue;
                }
                return bytes;
            }
            None => return bytes,
        }
    }
}

pub fn xmx(xms: u64) -> u64 {
    loop {
        print!("请输入Xmx(JVM虚拟机最大堆内存)的大小: ");
        let input_value = input();
        let bytes = to_bytes(&input_value);
        if bytes == 0 {
            println!("输入错误,请重新输入!");
            continue;
        }
        if bytes < to_bytes("1M") {
            println!("输入错误,Xmx不能小于1M,请重新输入!");
            continue;
        }
        if bytes < xms {
            println!("输入错误,Xmx不能小于Xms,请重新输入!");
            continue;
        }
        return bytes;
    }
}

pub fn jvm_args(jvm_args: Option<Value>) -> Value {
    let mut args = Vec::<Value>::new();
    if let Some(jvm_args) = jvm_args {
        for arg in jvm_args.as_array().unwrap() {
            args.push(arg.clone());
        }
    } else {
        args.push(json!("-Dlog4j2.formatMsgNoLookups=true"));
    }
    loop {
        let mut index = 0;
        for arg in args.clone() {
            println!("{index}: {}", arg.as_str().unwrap());
            index += 1;
        }
        println!("{index}: 新参数");
        println!("{}: 确认", index + 1);
        print!("请选择一个选项或要更改的参数(如果为空即为移除参数): ");
        let input_value = input();
        match input_value.parse::<usize>() {
            Ok(input_index) => {
                if input_index == index {
                    print!("请输入参数: ");
                    let input_arg = input();
                    args.push(json!(input_arg));
                    continue;
                }
                if input_index == index + 1 {
                    return json!(args);
                }
                if input_index > index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                println!("请输入参数: ");
                let input_arg = input();
                if input_arg.is_empty() {
                    args.remove(input_index);
                } else {
                    args[input_index] = json!(input_arg);
                }
                continue;
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}

async fn core() -> String {
    let fastmirror = get_fastmirror_value().await;
    loop {
        let mut index = 0;
        let mut cores = Vec::<&String>::new();
        if let Some(obj) = fastmirror.as_object() {
            for (core, value) in obj {
                println!("{index}: {core}(标签: {})", value["tag"]);
                cores.push(core);
                index += 1;
            }
        }
        print!("请选择一个使用的核心: ");
        let input_value = input();
        match input_value.parse::<usize>() {
            Ok(value) => {
                if value > index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                return cores[value].clone();
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}

async fn mc_version(core: &str) -> String {
    let fastmirror = get_fastmirror_value().await;
    loop {
        let mut index = 0;
        let mut mc_versions = Vec::<&str>::new();
        debug!("{core}, {fastmirror}");
        if let Some(arr) = fastmirror[&core]["mc_versions"].as_array() {
            for version in arr {
                if let Some(version) = version.as_str() {
                    println!("{index}: {version}");
                    mc_versions.push(version);
                    index += 1;
                }
            }
        }
        print!("请选择一个使用的minecraft版本: ");
        let input_value = input();
        match input_value.parse::<usize>() {
            Ok(value) => {
                if value > index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                return mc_versions[value].to_string();
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}

async fn build_version(core: &str, mc_version: &str) -> String {
    let fastmirror = get_fastmirror_builds_value(&core, &mc_version).await;
    loop {
        let mut index = 0;
        let mut builds = Vec::<&String>::new();
        if let Some(obj) = fastmirror.as_object() {
            for (build, value) in obj {
                println!("{index}: {build}(更新时间: {})", value["update_time"]);
                builds.push(build);
                index += 1;
            }
        }
        print!("请选择一个使用的构建版本: ");
        let input_value = input();
        match input_value.parse::<usize>() {
            Ok(value) => {
                if value > index {
                    println!("输入错误,请重新输入!");
                    continue;
                }
                return builds[value].clone();
            }
            Err(_) => {
                println!("输入错误,请重新输入!");
                continue;
            }
        }
    }
}

pub async fn main() {
    let mut configs = json!({});

    // 服务器的名称
    let name = name();
    configs["name"] = json!(name);

    // 设置使用的Java
    configs["java"] = java();

    // JVM参数
    configs["jvm_args"] = jvm_args(None);

    // 设置编码
    configs["encoding"] = json!(encoding());

    // 设置Xmx和Xms
    println!("1GB = 1000MB");
    println!("1MB = 1000KB");
    println!("1KB = 1000Bytes");
    let xms = xms(None);
    let xmx = xmx(xms);
    configs["Xms"] = json!(xms);
    configs["Xmx"] = json!(xmx);

    // 下载核心
    let core = core().await;
    let mc_version = mc_version(&core).await;
    let build_version = build_version(&core, &mc_version).await;
    match download_fastmirror_core(&core, &mc_version, &build_version).await {
        Ok(file_path) => {
            let current_dir = env::current_dir()
                .unwrap()
                .join("MCSCS")
                .join("servers")
                .join(&name);
            if let Err(e) = fs::create_dir_all(&current_dir) {
                println!("创建目录失败: {e}");
            }
            if let Err(e) = fs::copy(PathBuf::from(file_path), &current_dir.join("server.jar")) {
                println!("复制核心失败: {e}");
            }
        }
        Err(e) => {
            println!("下载核心失败: {e}");
        }
    }
    configs["info"] = json!(
        {
            "core": core,
            "mc_version": mc_version,
            "build_version": build_version
        }
    );

    save_servers_lists(&name, Some(&configs));
    println!("{}", serde_json::to_string_pretty(&configs).unwrap());
}
