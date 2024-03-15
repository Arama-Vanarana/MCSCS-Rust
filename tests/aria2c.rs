/*
 * Copyright (c) 2024 Minecraft Server Config Script for Rust.
 */

use log::{error, info};
use serde_json::json;

use mcscs::aria2c::{call_aria2c_rpc, get_aria2c_execute};
use mcscs::{aria2c::install_aria2c, pages::init};

#[tokio::test]
async fn test_get_aria2c_version() {
    init::main().await.expect("main()");
    match call_aria2c_rpc("aria2.getVersion", json!([])) {
        Ok(result) => {
            info!("{}", serde_json::to_string_pretty(&json!(result)).unwrap());
        }
        Err(e) => {
            error!("{e}");
        }
    }
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
