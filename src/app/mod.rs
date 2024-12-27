pub mod account;
mod redis_util;
pub mod snipe;
mod wx_response;

use redis::{Connection, RedisResult};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
pub use wx_response::*;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Option<ConfigServer>,
    pub redis: Option<ConfigRedis>,
    pub wx: Option<ConfigApi>,
    pub api: Option<ConfigApi>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigServer {
    pub path: Option<String>,
    pub port: Option<u16>,
    pub url: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigDatabase {
    pub url: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigRedis {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub expire: Option<i64>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigApi {
    pub ws: Option<String>,
    pub url: Option<String>,
    pub id: Option<String>,
    pub account: Option<String>,
    pub token: Option<String>,
    pub secret: Option<String>,
}

impl Config {
    pub async fn get() -> Self {
        let mut yaml_file = tokio::fs::File::open("config.yaml")
            .await
            .expect("read file error");
        let mut yaml_str = String::new();
        yaml_file
            .read_to_string(&mut yaml_str)
            .await
            .expect("read str error");
        serde_yml::from_str(yaml_str.as_str()).expect("config error")
    }

    pub async fn get_redis_conn(self) -> RedisResult<Connection> {
        let config = self.redis.unwrap_or_default();
        let password = urlencoding::encode("Z#2nTt98ve!Q#*RY");
        let client = redis::Client::open(format!(
            "redis://{}:{}@{}",
            &config.username.unwrap_or_default(),
            password,
            &config.url.unwrap_or_default()
        ))?;
        let con = client.get_connection()?;
        Ok(con)
    }
}
