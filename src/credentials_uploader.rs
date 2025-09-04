use crate::hbbs_http::create_http_client;
use crate::ipc;

use hbb_common::{default_net, fingerprint, log, mac_address, password_security};

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn start_task() {
    log::info!("uploader_start");
    password_security::update_temporary_password();
    thread::spawn(move || loop {
        upload();
        thread::sleep(Duration::from_secs(10));
    });
}

fn get_mac_address() -> String {
    let mut addr = default_net::get_mac().map(|m| m.addr).unwrap_or_default();
    if addr.is_empty() {
        addr = mac_address::get_mac_address()
            .ok()
            .and_then(|mac| mac)
            .map(|mac| mac.to_string())
            .unwrap_or_else(|| "".to_string());
    }
    addr = addr.replace(":", "");
    addr
}

fn upload() {
    log::info!("id=======>{}", ipc::get_id());
    log::info!(
        "passwd==========>{}",
        password_security::temporary_password()
    );
    let info = fingerprint::get_fingerprinting_info();
    log::info!("mac=======>{}", get_mac_address());

    let client = create_http_client();
    let mut headers = HeaderMap::new();
    headers.insert("tenant-id", HeaderValue::from_static("1"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let mut json_data = HashMap::new();
    json_data.insert("clientId", ipc::get_id());
    json_data.insert("clientPasswd", password_security::temporary_password());
    json_data.insert("macAddress", info.addr().to_string());
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
