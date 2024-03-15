/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use std::error::Error;

use mcscs::pages::{choose, clear_console, config, create, delete, import, init, pause, start};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init::main().await.expect("main()");
    let options = vec![
        "启动服务器",
        "创建服务器",
        "配置服务器",
        "删除服务器",
        "导入服务器",
        "退出",
    ];
    loop {
        let selection = choose("请选择一个选项(请按上下键切换, Enter确认)", &options)?;
        if selection == options.len() - 1 {
            return Ok(());
        }
        clear_console();
        if selection == 0 {
            start::main()?;
        } else if selection == 1 {
            create::main().await?;
        } else if selection == 2 {
            config::main()?;
        } else if selection == 3 {
            delete::main()?;
        } else if selection == 4 {
            import::main();
        }
        pause();
        clear_console();
    }
}
