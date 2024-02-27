use serde_json::json;

use crate::library::{
    controllers::{
        aria2c,
        fastmirror::{
            download_fastmirror_core, get_fastmirror_builds_value, get_fastmirror_value,
            get_file_sha1,
        },
    },
    pages::init,
};
use crate::library::controllers::aria2c::call_aria2c_rpc;

#[tokio::test]
#[doc = "测试下载核心"]
async fn test_download_fastmirror_core() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let fastmirror = get_fastmirror_value().await;
    let mc_version = if let Some(fastmirror) = fastmirror["Mohist"]["mc_versions"]
        .as_array()
        .and_then(|arr| arr.get(0))
    {
        fastmirror.as_str().unwrap()
    } else {
        "unknown"
    };
    let fastmirror = get_fastmirror_builds_value("Mohist", mc_version).await;
    let build_version = if let Some(fastmirror) = fastmirror
        .as_object()
        .and_then(|obj| obj.iter().next_back())
    {
        fastmirror.0
    } else {
        "unknown"
    };
    println!(
        "{}",
        download_fastmirror_core("Mohist", mc_version, build_version)
            .await
            .unwrap()
    );
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
    ()
}

#[tokio::test]
#[doc = "测试计算核心SHA1"]
async fn test_check_sha1() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let fastmirror = get_fastmirror_value().await;
    let mc_version = if let Some(fastmirror) = fastmirror["Mohist"]["mc_versions"]
        .as_array()
        .and_then(|arr| arr.get(0))
    {
        fastmirror.as_str().unwrap()
    } else {
        "unknown"
    };
    let fastmirror = get_fastmirror_builds_value("Mohist", mc_version).await;
    let build_version = if let Some(fastmirror) = fastmirror
        .as_object()
        .and_then(|obj| obj.iter().next_back())
    {
        fastmirror.0
    } else {
        "unknown"
    };
    let fastmirror_sha1 = fastmirror.clone()[build_version]["sha1"].take();
    let fastmirror_sha1_str = fastmirror_sha1.as_str().unwrap();
    let file_path = download_fastmirror_core("Mohist", mc_version, build_version)
        .await
        .unwrap();
    let file_sha1 = get_file_sha1(&file_path);
    println!("文件路径 = {file_path}");
    println!("FastMirror SHA1 = {fastmirror_sha1_str}");
    println!("File SHA1 = {file_sha1}");
    println!("是否一致: {}", { file_sha1 == fastmirror_sha1_str });
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
    ()
}

#[tokio::test]
#[doc = "测试下载文件"]
async fn test_download_file() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let file_path = if let Ok(file_path) = aria2c::download("http://speedtest.zju.edu.cn/1000M".to_string()).await {
        file_path
    } else if let Err(err) = aria2c::download("http://speedtest.zju.edu.cn/1000M".to_string()).await {
        eprintln!("下载文件失败: {err}");
        "unknown".to_string()
    } else {
        unreachable!()
    };
    println!("文件路径 = {file_path}");
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
    ()
}
