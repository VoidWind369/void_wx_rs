mod wx_response;
mod redis_util;
pub mod snipe;
pub mod account;

use redis::{Connection, RedisResult};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
pub use wx_response::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: Option<i64>,
    pub server_url: Option<String>,
    pub redis_url: String,
    pub redis_username: String,
    pub redis_password: String,
    pub redis_expire: i64,
    pub wx_appid: String,
    pub wx_secret: String,
}

pub async fn get_config() -> Config {
    let mut yaml_file = tokio::fs::File::open("config.yaml").await.expect("read file error");
    let mut yaml_str = String::new();
    yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
    serde_yaml::from_str::<Config>(yaml_str.as_str()).expect("config error")
}

pub async fn get_redis_conn() -> RedisResult<Connection> {
    let config = get_config().await;
    let password = urlencoding::encode("Z#2nTt98ve!Q#*RY");
    let client = redis::Client::open(format!("redis://{}:{}@{}", config.redis_username, password, config.redis_url))?;
    let con = client.get_connection()?;
    Ok(con)
}