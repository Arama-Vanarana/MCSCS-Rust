/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use std::error::Error;

use mcscs::pages::{config, delete, init, start};

#[doc = "测试创建服务器"]
mod test_create {
    use std::error::Error;

    use serde_json::json;

    use mcscs::pages::{
        create::{self, java, jvm_args, name, xms, xmx},
        init,
    };

    #[tokio::test]
    #[doc = "测试创建服务器页面"]
    async fn test_create_pages() -> Result<(), Box<dyn Error>> {
        create::main().await
    }

    #[test]
    #[doc = "测试用户输入服务器昵称"]
    fn test_server_name() {
        println!("{}", name())
    }

    #[test]
    #[doc = "测试用户选择Java环境"]
    fn test_java() {
        println!(
            "{}",
            serde_json::to_string_pretty(&java().unwrap()).unwrap_or("unknown".to_string())
        )
    }

    #[test]
    #[doc = "测试用户输入JVM初始堆内存和最大堆内存"]
    fn test_ram() {
        println!("1GB = 1000MB");
        println!("1MB = 1000KB");
        println!("1KB = 1000Bytes");
        let xms = xms(None);
        let xmx = xmx(xms);
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({"Xms": xms, "Xmx": xmx}))
                .unwrap_or("unknown".to_string())
        )
    }

    #[test]
    #[doc = "测试用户输入JVM参数"]
    fn test_jvm_args() {
        println!(
            "{}",
            serde_json::to_string_pretty(&jvm_args(None)).unwrap_or("unknown".to_string())
        )
    }

    #[doc = "测试用户选择核心"]
    mod test_download {
        use mcscs::{
            fastmirror::download_server_core,
            pages::create::{build_version, core, mc_version},
        };

        use super::*;

        #[tokio::test]
        #[doc = "测试下载核心"]
        async fn test_download() {
            if let Err(err) = init::main().await {
                eprintln!("初始化失败: {err}");
                return;
            }
            let core = core().await;
            let mc_version = mc_version(&core).await;
            let build_version = build_version(&core, &mc_version).await;
            match download_server_core(&core, &mc_version, &build_version).await {
                Ok(file_path) => {
                    println!("下载成功: {}", file_path.display());
                }
                Err(e) => {
                    println!("下载核心失败: {e}");
                }
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&json!(
                    {
                        "core": core,
                        "mc_version": mc_version,
                        "build_version": build_version
                    }
                ))
                .unwrap_or("unknown".to_string())
            )
        }
    }
}

#[test]
#[doc = "测试启动服务器页面"]
fn test_start_pages() -> Result<(), Box<dyn Error>> {
    start::main()
}

#[test]
#[doc = "测试删除服务器页面"]
fn test_delete_pages() -> Result<(), Box<dyn Error>> {
    delete::main()
}

#[test]
#[doc = "测试配置服务器页面"]
fn test_config_pages() -> Result<(), Box<dyn Error>> {
    config::main()
}

#[doc = "测试初始化页面"]
mod test_init {
    use log::{debug, error, info, trace, warn};

    use super::*;

    #[tokio::test]
    async fn test_log() {
        init::main().await.expect("main()");
        trace!("this is a trace msg.");
        debug!("this is a debug msg.");
        info!("this is an info msg.");
        warn!("this is a warn msg.");
        error!("this is an error msg.");
    }

    #[tokio::test]
    async fn test_init() {
        init::main().await.expect("main()");
    }
}
