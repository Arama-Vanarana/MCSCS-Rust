use crate::library::pages::{create, init, start};

#[tokio::test]
#[doc = "测试启动服务器页面"]
async fn test_start_page() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    start::main();
    ()
}

#[tokio::test]
#[doc = "测试创建服务器页面"]
async fn test_create_page() {
    if let Err(err) = init::main().await {
        eprintln!("初始化失败: {err}");
        return;
    }
    create::main().await;
    ()
}

pub mod controllers {
    use crate::library::{controllers::input, pages::init};

    #[tokio::test]
    #[doc = "测试输入"]
    async fn test_input() {
        if let Err(err) = init::main().await {
            eprintln!("初始化失败: {err}");
            return;
        }
        print!("请输入任意内容: ");
        println!("你输入了: {}", input());
        ()
    }

    #[tokio::test]
    #[doc = "测试日志"]
    async fn test_log() {
        use log::{debug, error, info, trace, warn};
        if let Err(err) = init::main().await {
            eprintln!("初始化失败: {err}");
            return;
        }
        trace!("This is a trace message");
        debug!("This is a debug message");
        info!("This is an info message");
        warn!("This is a warn message");
        error!("This is an error message");
        ()
    }
}
