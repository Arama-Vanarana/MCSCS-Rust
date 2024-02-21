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
            Err(err) => Err(err), // Forward the error if initialization fails
        }
    } else {
        Ok(())
    }
}

#[tokio::test]
#[doc = "测试输入"]
async fn test_input() {
    if let Err(err) = init().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    println!("你输入了: {}", library::pages::input("请输入任意内容"));
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
    );
}
