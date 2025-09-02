use crate::ipc;
use hbb_common::{log, tokio};

pub fn start_timer() {
    log::info!("uploader_start");
    std::thread::spawn(move || loop {
        log::info!("id=======>{}", ipc::get_id());
        log::info!("passwd==========>{}", ipc::get_permanent_password());
        tokio::time::sleep(std::time::Duration::from_secs(6));
    });
}