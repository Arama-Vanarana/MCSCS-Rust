/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use std::error::Error;

use mcscs::pages::{config, create, delete, import, init, start};
use mcscs::select::select_option;
use mcscs::utils::{clear_console, pause};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init::main().await?;
    let options = vec![
        "启动服务器",
        "创建服务器",
        "配置服务器",
        "删除服务器",
        // "导入服务器",
        "退出",
    ];
    loop {
        let selection = select_option("请选择一个选项(请按上下键切换, Enter确认)", &options)?;
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
