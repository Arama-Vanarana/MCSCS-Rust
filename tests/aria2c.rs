/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use std::error::Error;

use log::{error, info};
use serde_json::json;

use mcscs::aria2c::{call_aria2c_rpc, get_aria2c_execute};
use mcscs::{aria2c::install_aria2c, pages::init};

#[tokio::test]
async fn test_get_aria2c_version() -> Result<(), Box<dyn Error>> {
    init::main().await.expect("main()");
    let result = call_aria2c_rpc("aria2.getVersion", json!([]))?;
    println!("{}", serde_json::to_string_pretty(&json!(result)).unwrap());
    Ok(())
}

#[tokio::test]
async fn test_install_aria2c() {
    install_aria2c().await;
}

#[tokio::test]
async fn test_get_aria2c_execute() {
    init::main().await.expect("main()");
    let execute = get_aria2c_execute().expect("main()");
    info!("{}", execute.display());
}
