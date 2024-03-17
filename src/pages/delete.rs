/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use std::error::Error;
use std::{env, fs};

use dialoguer::Confirm;

use crate::select::select_server;

/// 删除服务器页面
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut server = select_server();
    if server.is_null() {
        println!("你还没有创建任何一个服务器!");
        return Ok(());
    }
    if Confirm::new()
        .with_prompt("你是否真的要删除此服务器?")
        .interact()
        .unwrap()
    {
        fs::remove_dir_all(
            env::current_dir()?
                .join("MCSCS")
                .join("servers")
                .join(server["name"].take().as_str().unwrap()),
        )?;
    }

    Ok(())
}
