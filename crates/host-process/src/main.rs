use std::env;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use env_logger::{Builder, Target};
use log::{info, LevelFilter};

fn main() {
    // 输出日志
    Builder::new()
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    info!("主进程启动");

    let cwd = env::current_dir().expect("无法获取当前工作目录");
    info!("当前工作目录: {:?}", cwd);

    let log_level = "info";

    let mut child = Command::new("./target/debug/child-process.exe")
        .env("RUST_LOG", log_level)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("进程未找到！");
    info!("进程ID: {}", child.id());
    loop {
        let status = child.try_wait().unwrap();
        if let Some(_) = status {
            break;
        }
        sleep(Duration::from_secs(1));
    }
    // child.wait().unwrap();
}
