use crate::ipc;
use hbb_common::log;
use std::time::Duration;
use std::thread;

pub fn start_timer() {
    log::info!("uploader_start");
    thread::spawn( move || {
        loop {
            log::info!("id=======>{}", ipc::get_id());
            log::info!("passwd==========>{}", ipc::get_permanent_password());
            thread::sleep(Duration::from_secs(6));
        }
    });
}
