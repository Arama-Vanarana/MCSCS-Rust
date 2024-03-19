/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use mcscs::{pages::init, select::select_server, server::load_servers_lists};

#[tokio::test]
async fn test_get_server_config() {
    init::main().await.expect("main()");
    let data = load_servers_lists(Some(
        select_server()["name"]
            .as_str()
            .expect("test_get_server_config()"),
    ));
    println!(
        "data: {}",
        serde_json::to_string_pretty(&data).expect("test_get_server_config()")
    );
}
