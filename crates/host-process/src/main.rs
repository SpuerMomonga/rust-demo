use std::env;
use std::process::{Command, Stdio};

fn main() {
    print!("主进程启动");

    let cwd = env::current_dir().expect("无法获取当前工作目录");
    println!("当前工作目录: {:?}", cwd);

    let mut child = Command::new("./target/debug/child-process.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("进程未找到！");
    println!("进程ID: {}", child.id());
    child.wait().unwrap();
}
