use crate::library::{
    controllers::{
        aria2c::{self},
        fastmirror::{
            download_fastmirror_core, get_fastmirror_builds_value, get_fastmirror_value,
            get_file_sha1,
        },
    },
    pages::init,
};

async fn get_new_fastmirror_info(core: &str) -> (String, String) {
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

#[tokio::test]
#[doc = "测试下载核心"]
async fn test_download_fastmirror_core() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let (mc_version, build_version) = get_new_fastmirror_info("Mohist").await;
    println!(
        "{}",
        download_fastmirror_core("Mohist", &mc_version, &build_version)
            .await
            .unwrap()
    );
}

#[tokio::test]
#[doc = "测试计算核心SHA1"]
async fn test_check_sha1() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let (mc_version, build_version) = get_new_fastmirror_info("Mohist").await;
    let fastmirror = get_fastmirror_builds_value("Mohist", &mc_version).await;
    let fastmirror_sha1 = fastmirror.clone()[&build_version]["sha1"].take();
    let fastmirror_sha1_str = fastmirror_sha1.as_str().unwrap();
    let file_path = download_fastmirror_core("Mohist", &mc_version, &build_version)
        .await
        .unwrap();
    let file_sha1 = get_file_sha1(&file_path);
    println!("文件路径 = {file_path}");
    println!("FastMirror SHA1 = {fastmirror_sha1_str}");
    println!("File SHA1 = {file_sha1}");
    println!("是否一致: {}", { file_sha1 == fastmirror_sha1_str });
}

#[tokio::test]
#[doc = "测试下载文件"]
async fn test_download_file() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let downloads = aria2c::download("https://speedtest.zju.edu.cn/1000M".to_string()).await;
    let file_path = match downloads {
        Ok(file_path) => {
            file_path
        }
        Err(err) => {
            eprintln!("下载文件失败: {err}");
            "unknown".to_string()
        }
    };
    println!("文件路径 = {file_path}");
}
