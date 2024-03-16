/*
 * Copyright (c) 2024 Arama.
 */

use std::error::Error;
use std::{env, fs};

use dialoguer::Confirm;

use crate::pages::choose_server;

/// 删除服务器页面
pub fn main() -> Result<(), Box<dyn Error>> {
    let mut server = choose_server();
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
