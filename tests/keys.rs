use mcscs::pages::{clear_console, pause};

#[test]
#[doc = "测试按任意键继续"]
fn test_pause() {
    pause()
}

#[test]
#[doc = "测试清空控制台"]
fn test_clear_console() {
    print!("Test strings.");
    pause();
    clear_console();
}