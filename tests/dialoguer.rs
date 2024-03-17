/*
 * Copyright (c) 2024 Arama. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 * Morbi non lorem porttitor neque feugiat blandit. Ut vitae ipsum eget quam lacinia accumsan.
 * Etiam sed turpis ac ipsum condimentum fringilla. Maecenas magna.
 * Proin dapibus sapien vel ante. Aliquam erat volutpat. Pellentesque sagittis ligula eget metus.
 * Vestibulum commodo. Ut rhoncus gravida arcu.
 */

use dialoguer::Editor;

#[test]
fn main() {
    if let Some(rv) = Editor::new()
        .executable("vim")
        .edit("Enter a commit message")
        .unwrap()
    {
        println!("Your message:");
        println!("{}", rv);
    } else {
        println!("Abort!");
    }
}
