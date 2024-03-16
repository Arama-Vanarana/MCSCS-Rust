/*
 * Copyright (c) 2024 Arama.
 */

use std::error::Error;
use std::{collections::HashMap, env, fs};

use log::error;
use serde_json::{json, Value};

use crate::select::{select_file, select_option};
use crate::utils::input;
use crate::{
    fastmirror::{download_server_core, get_fastmirror_builds_value, get_fastmirror_value},
    java::{detect_java, get_java_version, load_java_lists, save_java_lists},
    server::{load_servers_lists, save_servers_lists},
};

/// 返回用户输入的服务器名称
pub fn name() -> String {
    let servers = load_servers_lists(None);
    loop {
        let name = input("请输入该服务器的名称");
        if servers.get(&name).is_some() {
            println!("输入错误,服务器已存在,请重新输入!");
            continue;
        }
        return name;
    }
}

/// 返回用户选择/手动输入的Java环境
pub fn java() -> Result<Value, Box<dyn Error>> {
    loop {
        let javas = load_java_lists();
        let mut options: Vec<String> = Vec::new();
        for java in javas.as_array().unwrap() {
            let info = format!(
                "{}: {}",
                java["version"].as_str().unwrap(),
                java["path"].as_str().unwrap()
            );
            options.push(info);
        }
        options.push("重新检测Java环境".to_string());
        options.push("手动输入Java可执行程序路径".to_string());
        let selection = select_option("请选择一个Java环境或选项", &options)?;
        if selection == options.len() - 2 {
            save_java_lists(&detect_java());
            println!("刷新成功!");
            continue;
        }
        if selection == options.len() - 1 {
            #[cfg(target_os = "windows")]
            let java_path = choose_file("java.exe").unwrap();
            #[cfg(not(target_os = "windows"))]
            let java_path = select_file("请选择一个Java可执行程序").unwrap();
            if let Ok(metadata) = fs::metadata(&java_path) {
                if metadata.is_file() {
                    let java_ver = get_java_version(&java_path);
                    if java_ver.is_err() {
                        println!("Java无效!");
                        continue;
                    }
                    return Ok(json!({"path": java_path, "version": java_ver.unwrap()}));
                }
                println!("Java不存在!");
                continue;
            }
        }
        return Ok(javas[selection].clone());
    }
}

/// 返回用户选择的编码格式
pub fn encoding() -> String {
    let options = vec!["UTF-8", "GBK", "ANSI", "ASCII"];
    let selection = select_option("请选择需要使用的编码格式", &options).unwrap();
    options[selection].to_lowercase()
    // println!("0: UTF-8");
    // println!("1: GBK");
    // println!("2: ANSI");
    // println!("3: ASCII");
    // print!("请选择需要使用的编码格式: ");
    // let input_value = input();
    // if input_value == "0" {
    //     return "utf-8".to_string();
    // }
    // if input_value == "1" {
    //     return "gbk".to_string();
    // }
    // if input_value == "2" {
    //     return "ansi".to_string();
    // }
    // if input_value == "3" {
    //     return "ascii".to_string();
    // }
}

/// 将类似1G,1M等等的字节单位转换为Bytes
///
/// # 使用
/// ```
/// // 1G = 1000000000B
/// use mcscs::pages::create::to_bytes;
/// let bytes = to_bytes("1GB");
/// ```
pub fn to_bytes(byte: &str) -> u64 {
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
    unit_json.insert("T".to_string(), 1024 * 1024 * 1024 * 1024);
    unit_json.insert("TB".to_string(), 1000 * 1000 * 1000 * 1000);
    unit_json.insert("TIB".to_string(), 1024 * 1024 * 1024 * 1024);

    unit_json.insert("G".to_string(), 1024 * 1024 * 1024);
    unit_json.insert("GB".to_string(), 1000 * 1000 * 1000);
    unit_json.insert("GIB".to_string(), 1024 * 1024 * 1024);

    unit_json.insert("M".to_string(), 1024 * 1024);
    unit_json.insert("MB".to_string(), 1000 * 1000);
    unit_json.insert("MIB".to_string(), 1024 * 1024);

    unit_json.insert("K".to_string(), 1024);
    unit_json.insert("KB".to_string(), 1000);
    unit_json.insert("KIB".to_string(), 1024);

    unit_json.insert("B".to_string(), 1);
    unit_json.insert("Bytes".to_string(), 1);
    unit_json.insert("".to_string(), 1);

    match num_part.parse::<u64>() {
        Ok(num) => num * (*unit_json.get(&unit_part).unwrap_or(&0)),
        Err(e) => {
            error!("{e}");
            0
        }
    }
}

/// 返回用户输入的XMS(JVM虚拟机初始堆内存)
///
/// # 使用
/// * 使用场景: 创建服务器
/// ```
/// use mcscs::pages::create::xms;
/// let xms = xms(None);
/// ```
/// * 使用场景: 配置服务器, 服务器的XMX为1GB
/// ```
/// use mcscs::pages::create::{to_bytes, xmx};
/// let xms = xmx(to_bytes("1GB"));
/// ```
pub fn xms(xmx: Option<u64>) -> u64 {
    loop {
        let input_value = input("请输入Xms(JVM虚拟机初始堆内存)的大小");
        let bytes = to_bytes(&input_value);
        if bytes == 0 {
            println!("输入错误,请重新输入!");
            continue;
        }
        if bytes < to_bytes("1MiB") {
            println!("输入错误,Xms不能小于1MiB,请重新输入!");
            continue;
        }
        if let Ok(mem) = sys_info::mem_info() {
            if bytes > (mem.total * 1024) {
                println!("输入错误,Xms不能大于系统内存,请重新输入!");
                continue;
            }
        }
        return match xmx {
            Some(xmx) => {
                if bytes > xmx {
                    println!("输入错误,Xms不能大于Xmx,请重新输入!");
                    continue;
                }
                bytes
            }
            None => bytes,
        };
    }
}

/// 返回用户输入的XMX(JVM虚拟机最大堆内存)
pub fn xmx(xms: u64) -> u64 {
    loop {
        let input_value = input("请输入Xmx(JVM虚拟机最大堆内存)的大小");
        let bytes = to_bytes(&input_value);
        if bytes == 0 {
            println!("输入错误,请重新输入!");
            continue;
        }
        if bytes < to_bytes("1MiB") {
            println!("输入错误,Xmx不能小于1MiB,请重新输入!");
            continue;
        }
        if let Ok(mem) = sys_info::mem_info() {
            if bytes > (mem.total * 1024) {
                println!("输入错误,Xms不能大于系统内存,请重新输入!");
                continue;
            }
        }
        if bytes < xms {
            println!("输入错误,Xmx不能小于Xms,请重新输入!");
            continue;
        }
        return bytes;
    }
}

/// 返回用户输入的JVM虚拟机参数
///
/// # 使用
/// * 使用场景: 创建服务器
/// ```
/// // 如果是None配置默认会是json!(["-Dlog4j2.formatMsgNoLookups=true"])
/// use mcscs::pages::create::jvm_args;
/// let jvm_args = jvm_args(None);
/// ```
/// * 使用场景: 配置服务器
/// ```
/// use serde_json::json;
/// use mcscs::pages::create::jvm_args;
/// let config = json!(["JVM虚拟机参数", "..."]);
/// let jvm_args = jvm_args(Some(&config));
/// ```
pub fn jvm_args(jvm_args: Option<&Value>) -> Value {
    let mut args = Vec::<Value>::new();
    if let Some(jvm_args) = jvm_args {
        for arg in jvm_args.as_array().unwrap() {
            args.push(arg.clone());
        }
    } else {
        args.push(json!("-Dlog4j2.formatMsgNoLookups=true"));
    }
    loop {
        let mut display_args = args
            .iter()
            .filter_map(|value| value.as_str())
            .collect::<Vec<&str>>();
        display_args.push("新参数");
        display_args.push("确认");
        let selection = select_option(
            "请选择一个选项或要更改的JVM虚拟机参数(如果为空即为移除参数)",
            &display_args,
        )
        .unwrap();
        if selection == display_args.len() - 2 {
            let input_arg = input("请输入参数");
            args.push(json!(input_arg));
            continue;
        }
        if selection == display_args.len() - 1 {
            return json!(args);
        }
        let input_arg = input("请输入参数");
        if input_arg.is_empty() {
            args.remove(selection);
        } else {
            args[selection] = json!(input_arg);
        }
        continue;
    }
}

/// 返回用户输入的服务器参数
///
/// # 使用
/// * 使用场景: 创建服务器
/// ```
/// // 如果是None配置默认会是json!(["--nogui"])
/// use mcscs::pages::create::server_args;
/// let server_args = server_args(None);
/// ```
/// * 使用场景: 配置服务器
/// ```
/// use serde_json::json;
/// use mcscs::pages::create::server_args;
/// let config = json!(["服务器参数", "..."]);
/// let server_args = server_args(Some(&config));
/// ```
pub fn server_args(server_args: Option<&Value>) -> Value {
    let mut args = Vec::<Value>::new();
    if let Some(server_args) = server_args {
        for arg in server_args.as_array().unwrap() {
            args.push(arg.clone());
        }
    } else {
        args.push(json!("--nogui"));
    }
    loop {
        let mut display_args = args
            .iter()
            .filter_map(|value| value.as_str())
            .collect::<Vec<&str>>();
        display_args.push("新参数");
        display_args.push("确认");
        let selection = select_option(
            "请选择一个选项或要更改的服务器参数(如果为空即为移除参数)",
            &display_args,
        )
        .unwrap();
        if selection == display_args.len() - 2 {
            let input_arg = input("请输入参数");
            args.push(json!(input_arg));
            continue;
        }
        if selection == display_args.len() - 1 {
            return json!(args);
        }
        let input_arg = input("请输入参数");
        if input_arg.is_empty() {
            args.remove(selection);
        } else {
            args[selection] = json!(input_arg);
        }
        continue;
    }
}

/// 返回用户选择的服务器核心
pub async fn core() -> String {
    let fastmirror = get_fastmirror_value().await;
    let mut options = Vec::<String>::new();
    let mut cores = Vec::<&str>::new();
    if let Some(obj) = fastmirror.as_object() {
        for (core, value) in obj {
            options.push(format!("{core}(标签: {})", value["tag"].as_str().unwrap()));
            cores.push(core);
        }
    }
    let selection = select_option("请选择一个使用的核心", &options).unwrap();
    cores[selection].to_string()
}

/// 返回用户选择的服务器核心支持的Minecraft版本
pub async fn mc_version(core: &str) -> String {
    let fastmirror = get_fastmirror_value().await;
    let mut options = Vec::<&str>::new();
    if let Some(arr) = fastmirror[&core]["mc_versions"].as_array() {
        for version in arr {
            if let Some(version) = version.as_str() {
                options.push(version);
            }
        }
    }
    let selection = select_option("请选择一个使用的minecraft版本", &options).unwrap();
    options[selection].to_string()
}

/// 返回用户选择的构建版本
pub async fn build_version(core: &str, mc_version: &str) -> String {
    let fastmirror = get_fastmirror_builds_value(core, mc_version).await;
    let mut options = Vec::<String>::new();
    let mut builds = Vec::<&str>::new();
    if let Some(obj) = fastmirror.as_object() {
        for (build, value) in obj {
            options.push(format!("{build}(更新时间: {})", value["update_time"]));
            builds.push(build);
        }
    }
    let selection = select_option("请选择一个使用的构建版本", &options).unwrap();
    builds[selection].to_string()
}

/// 创建服务器页面
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut configs = json!({});

    // 服务器的名称
    let name = name();
    configs["name"] = json!(name);

    // 设置使用的Java
    configs["java"] = java()?;

    // JVM参数
    configs["jvm_args"] = jvm_args(None);

    // 设置编码
    configs["encoding"] = json!(encoding());

    // 设置Xmx和Xms
    println!("1GiB = 1024MB, 1GB = 1000MB");
    println!("1MiB = 1024KB, 1MB = 1000KB");
    println!("1KiB = 1024Bytes, 1KB = 1000Bytes");
    let xms = xms(None);
    let xmx = xmx(xms);
    configs["Xms"] = json!(xms);
    configs["Xmx"] = json!(xmx);

    // 下载核心
    let core = core().await;
    let mc_version = mc_version(&core).await;
    let build_version = build_version(&core, &mc_version).await;

    let file_path = download_server_core(&core, &mc_version, &build_version).await?;
    let current_dir = env::current_dir()?
        .join("MCSCS")
        .join("servers")
        .join(&name);
    fs::create_dir_all(&current_dir)?;
    fs::copy(file_path, current_dir.join("server.jar"))?;

    configs["info"] = json!(
        {
            "core": core,
            "mc_version": mc_version,
            "build_version": build_version
        }
    );

    // 服务器参数
    configs["server_args"] = server_args(None);

    save_servers_lists(&name, &configs);
    Ok(())
}
