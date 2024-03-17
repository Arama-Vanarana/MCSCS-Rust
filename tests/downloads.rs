/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use mcscs::{
    aria2c,
    fastmirror::{
        download_server_core, get_fastmirror_builds_value, get_fastmirror_value, get_file_sha1,
    },
    pages::init,
};

async fn get_new_fastmirror_info(core: &str) -> (String, String, String) {
    let fastmirror = get_fastmirror_value().await;
    let mc_version = if let Some(fastmirror) = fastmirror[core]["mc_versions"]
        .as_array()
        .and_then(|arr| arr.first())
    {
        fastmirror.as_str().unwrap().to_string()
    } else {
        "unknown".to_string()
    };
    let fastmirror = get_fastmirror_builds_value(core, &mc_version).await;
    let build_version = fastmirror[0]["core_version"].as_str().unwrap().to_string();
    let sha1 = fastmirror[0]["sha1"].as_str().unwrap().to_string();
    (mc_version, build_version, sha1)
}

/// 测试下载核心
#[tokio::test]
async fn test_download_server_core() {
    init::main().await.expect("main()");
    let (mc_version, build_version, _) = get_new_fastmirror_info("Mohist").await;
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
    let (mc_version, build_version, fastmirror_sha1) = get_new_fastmirror_info("Mohist").await;
    let file_path = download_server_core("Mohist", &mc_version, &build_version)
        .await
        .unwrap();
    let file_sha1 = get_file_sha1(&file_path);
    println!("文件路径 = {}", file_path.display());
    println!("FastMirror SHA1 = {fastmirror_sha1}");
    println!("File SHA1 = {file_sha1}");
    println!("是否一致: {}", { file_sha1 == fastmirror_sha1 });
}

/// 测试下载文件
#[tokio::test]
async fn test_download_file() {
    init::main().await.expect("main()");
    let downloads =
        aria2c::download("https://speed.cloudflare.com/__down?during=download&bytes=104857600");
    let file_path = downloads.unwrap();
    println!("文件路径 = {}", file_path.display());
}
