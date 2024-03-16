/*
 * Copyright (c) 2024 Arama.
 */

use mcscs::{
    aria2c,
    fastmirror::{
        download_server_core, get_fastmirror_builds_value, get_fastmirror_value, get_file_sha1,
    },
    pages::init,
};

async fn get_new_fastmirror_info(core: &str) -> (String, String) {
    let fastmirror = get_fastmirror_value().await;
    println!("{fastmirror}");
    let mc_version = if let Some(fastmirror) = fastmirror[core]["mc_versions"]
        .as_array()
        .and_then(|arr| arr.first())
    {
        fastmirror.as_str().unwrap().to_string()
    } else {
        "unknown".to_string()
    };
    let fastmirror = get_fastmirror_builds_value(core, &mc_version).await;
    println!("{fastmirror}");
    let build_version = if let Some(fastmirror) = fastmirror
        .as_object()
        .and_then(|obj| obj.iter().next_back())
    {
        fastmirror.0.to_string()
    } else {
        "unknown".to_string()
    };
    (mc_version, build_version)
}

/// 测试下载核心
#[tokio::test]
async fn test_download_fastmirror_core() {
    init::main().await.expect("main()");
    let (mc_version, build_version) = get_new_fastmirror_info("Mohist").await;
    println!(
        "{}",
        download_server_core("Mohist", &mc_version, &build_version)
            .await
            .unwrap()
            .display()
    );
}

/// 测试计算SHA1
#[tokio::test]
async fn test_check_sha1() {
    init::main().await.expect("main()");
    let (mc_version, build_version) = get_new_fastmirror_info("Mohist").await;
    let mut fastmirror = get_fastmirror_builds_value("Mohist", &mc_version).await;
    let fastmirror_sha1 = fastmirror[&build_version]["sha1"].take();
    let fastmirror_sha1_str = fastmirror_sha1.as_str().unwrap();
    let file_path = download_server_core("Mohist", &mc_version, &build_version)
        .await
        .unwrap();
    let file_sha1 = get_file_sha1(&file_path);
    println!("文件路径 = {}", file_path.display());
    println!("FastMirror SHA1 = {fastmirror_sha1_str}");
    println!("File SHA1 = {file_sha1}");
    println!("是否一致: {}", { file_sha1 == fastmirror_sha1_str });
}

/// 测试下载文件
#[tokio::test]
async fn test_download_file() {
    init::main().await.expect("main()");
    let downloads =
        aria2c::download("https://speed.cloudflare.com/__down?during=download&bytes=104857600");
    let file_path = downloads.unwrap_or_else(|err| {
        eprintln!("下载文件失败: {err}");
        "unknown".to_string()
    });
    println!("文件路径 = {file_path}");
}
