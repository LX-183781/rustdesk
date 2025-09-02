use crate::ipc;
use hbb_common::{log, password_security};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn start_task() {
    log::info!("uploader_start");
    thread::spawn(move || loop {
        upload();
        thread::sleep(Duration::from_secs(6));
    });
}

fn upload() {
    let client = reqwest::Client::new();
    log::info!("id=======>{}", ipc::get_id());
    log::info!(
        "passwd==========>{}",
        password_security::temporary_password()
    );
    let mut headers = HeaderMap::new();
    headers.insert("tenant-id", HeaderValue::from_static("1"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let mut json_data = HashMap::new();
    json_data.insert("clientId", ipc::get_id());
    json_data.insert("clientPasswd", password_security::temporary_password());

    let response = match client
        .post("http://localhost:48080/app-api/rdm/rustdesk-client/upload-client-info")
        .headers(headers)
        .json(&json_data)
        .send()
    {
        Ok(res) => {
            log::info!("Upload successful, status: {}", res.status());
            res
        }
        Err(e) => {
            log::error!("Upload failed: {}", e);
            return;
        }
    };

    // 处理响应内容
    if let Ok(body) = response.text() {
        log::info!("Response body: {}", body);
    } else {
        log::warn!("Failed to read response body");
    }
}
