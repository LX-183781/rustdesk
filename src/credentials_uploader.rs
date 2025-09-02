use crate::ipc;
use hbb_common::{log,password_security};
use std::time::Duration;
use std::thread;

pub fn start_task() {
    log::info!("uploader_start");
    thread::spawn( move || {
        loop {
            upload();
            thread::sleep(Duration::from_secs(6));
        }
    });
}

fn upload(){
    log::info!("id=======>{}", ipc::get_id());
    log::info!("passwd==========>{}", password_security::temporary_password());
}