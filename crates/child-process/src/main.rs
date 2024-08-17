use std::thread::sleep;
use std::time::Duration;

use log::info;

fn main() {
    env_logger::init();
    info!("子进程启动！");
    sleep(Duration::from_secs(5));
    info!("子进程结束！");
}
