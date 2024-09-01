use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::sync::Mutex;

pub type DBPool = Arc<Mutex<State>>;
pub struct State {
    pub db_pool: SqlitePool,
    pub domain: String,
    pub port: String,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct MyRequest {
    pub url: String,
    pub wireguard: WireGuard,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct WireGuard {
    pub name: String,
    pub server_ip: String,
    pub server_port: String,
    pub client_ip: String,
    pub public_key: String,
    pub private_key: String,
    pub allowed_ips: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    #[serde(rename = "socks-port")]
    pub socks_port: u16,
    #[serde(rename = "allow-lan")]
    pub allow_lan: bool,
    pub mode: String,
    #[serde(rename = "log-level")]
    pub log_level: String,
    #[serde(rename = "external-controller")]
    pub external_controller: String,
    pub proxies: Vec<Proxy>,
    #[serde(rename = "proxy-groups")]
    pub proxy_groups: Vec<ProxyGroup>,
    pub rules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    pub server: String,
    pub port: u16,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub uuid: Option<String>,
    #[serde(rename = "alterId")]
    pub alterid: Option<u32>,
    pub cipher: Option<String>,
    pub tls: Option<bool>,
    pub network: Option<String>,
    #[serde(rename = "ws-opts")]
    pub ws_opts: Option<WsOpts>,
    pub password: Option<String>,
    pub sni: Option<String>,
    #[serde(rename = "skip-cert-verify")]
    pub skip_cert_verify: Option<bool>,
    #[serde(rename = "private-key")]
    pub private_key: Option<String>,
    #[serde(rename = "public-key")]
    pub public_key: Option<String>,
    pub ip: Option<String>,
    #[serde(rename = "allowed-ips")]
    pub allowed_ips: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsOpts {
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyGroup {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub proxies: Vec<String>,
    pub url: Option<String>,
    pub interval: Option<u32>,
    pub tolerance: Option<u32>,
    pub strategy: Option<String>,
}