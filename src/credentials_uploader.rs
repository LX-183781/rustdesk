use crate::ipc;
use hbb_common::{log, password_security};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::thread;
use std::time::Duration;

pub fn start_task() {
    log::info!("uploader_start");
    thread::spawn(move || loop {
        upload();
        thread::sleep(Duration::from_secs(6));
    });
}

async fn upload() {
    let client = reqwest::client::new();
    log::info!("id=======>{}", ipc::get_id());
    log::info!(
        "passwd==========>{}",
        password_security::temporary_password()
    );
    let mut headers = HeaderMap::new();
    headers.insert("tenant-id", HeaderValue::from_static("1"));

    let mut json_data = HashMap::new();
    json_data.insert("clientId", ipc::get_id());
    json_data.insert("clientPasswd", password_security::temporary_password());

    let post_form_response = client
        .post("http://localhost:48080/app-api/rdm/rustdesk-client/upload-client-info")
        .headers(headers)
        .json(&form_data)
        .send()
        .await?;
}
