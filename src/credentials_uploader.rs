use hbb_common::log;
use crate::ipc;

pub fn start_timer(){
    log::info!("uploader_start");
    log::info!("id=======>{}",ipc::get_id());
    log::info!("passwd==========>{}",ipc::get_permanent_password())
}