use crate::library;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let mut initialized = INITIALIZED.lock().await;
    if !*initialized {
        match library::pages::init::main().await {
            Ok(_) => {
                *initialized = true;
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else {
        Ok(())
    }
}

#[tokio::test]
#[doc = "测试创建服务器页面"]
async fn test_create_page() {
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    library::pages::create::main().await;
}

#[tokio::test]
#[doc = "测试输入"]
async fn test_input() {
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    println!(
        "你输入了: {}",
        library::controllers::input("请输入任意内容")
    );
}

#[tokio::test]
#[doc = "测试日志"]
async fn test_log() {
    use log::{debug, error, info, trace, warn};
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warn message");
    error!("This is an error message");
}

#[tokio::test]
#[doc = "测试寻找Java环境"]
async fn test_detect_java() {
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&library::controllers::java::detect_java()).unwrap()
    );
}

#[tokio::test]
#[doc = "测试下载核心"]
async fn test_download_fastmirror_core() {
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    println!(
        "{}",
        library::controllers::fastmirror::download_fastmirror_core("Mohist", "1.20.1", "build580")
            .await
            .unwrap()
    );
}

#[tokio::test]
#[doc = "测试计算核心SHA1"]
async fn test_check_sha1() {
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let fastmirror =
        &(library::controllers::fastmirror::get_fastmirror_builds_value("Mohist", "1.20.1").await)
            ["build580"];
    let fastmirror_sha1 = (&fastmirror["sha1"]).as_str().unwrap();
    let file_path =
        library::controllers::fastmirror::download_fastmirror_core("Mohist", "1.20.1", "build580")
            .await
            .unwrap();
    let file_sha1 = library::controllers::fastmirror::get_file_sha1(&file_path);
    println!("文件路径 = {file_path}");
    println!("FastMirror SHA1 = {fastmirror_sha1}");
    println!("File SHA1 = {file_sha1}");
    println!("是否一致: {}", { file_sha1 == fastmirror_sha1 });
}

#[tokio::test]
#[doc = "测试下载文件"]
async fn test_download_file() {
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    let file_path = match library::controllers::aria2c::download(
        "http://speedtest.zju.edu.cn/1000M".to_string(),
    )
    .await
    {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("下载文件失败: {err}");
            "Unknown".to_string()
        }
    };
    println!("文件路径 = {file_path}");
}
