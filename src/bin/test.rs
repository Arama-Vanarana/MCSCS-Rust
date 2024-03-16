/*
 * Copyright (c) 2024 Arama.
 */

use std::error::Error;

use mcscs::select::select_file;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", select_file("请选择任意一个文件")?.display());
    Ok(())
}
