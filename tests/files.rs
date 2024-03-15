/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use std::error::Error;

use mcscs::pages::choose_file;

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", choose_file("请选择任意一个文件")?.display());
    Ok(())
}
