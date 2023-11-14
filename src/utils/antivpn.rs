use serde::{Deserialize, Serialize};

use std::error::Error;
use gloo_net::http::Request;
use log::{info};

#[derive(Serialize, Deserialize, Debug)]
pub struct KauriResponse {
    accuracyRadius: i32,
    method: String,
    queriesLeft: i32,
    city: String,
    ip: String,
    isp: String,
    latitude: f64,
    longitude: f64,
    timeZone: String,
    lastAccess: i64,
    network: String,
    proxy: bool,
    countryCode: String,
    success: bool,
    organization: String,
    cached: bool,
    countryName: String,
    asn: String
}

pub async fn get_ip_info(ip: String) -> Result<KauriResponse, Box<dyn Error>> {

    let params = [("ip", ip)];

    let response = Request::get("https://funkemunky.cc/vpn?").query(params).send().await?;

    let status = response.status();
    info!("Status: {status}");

    let json: KauriResponse = response.json().await?;

    return Ok(json);
}

