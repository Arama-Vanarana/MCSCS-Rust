/*
 * Copyright (c) 2024 Arama.
 */

use mcscs::{
    pages::{choose_server, init},
    server::load_servers_lists,
};

#[tokio::test]
async fn test_get_server_config() {
    init::main().await.expect("main()");
    let data = load_servers_lists(Some(
        choose_server()["name"]
            .as_str()
            .expect("test_get_server_config()"),
    ));
    println!(
        "data: {}",
        serde_json::to_string_pretty(&data).expect("test_get_server_config()")
    );
}
