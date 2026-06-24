use redis::{Commands, RedisResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use void_log::*;

use crate::app::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ResAccessToken {
    access_token: Option<String>,
    expires_in: Option<i64>,
    err_code: Option<String>,
    err_msg: Option<String>,
}

impl ResAccessToken {
    fn get_token(&self) -> String {
        self.access_token.clone().unwrap_or_default()
    }

    async fn from_server() -> reqwest::Result<Self> {
        let config = Config::get().await.wx.unwrap_or_default();
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
            &config.id.unwrap_or_default(),
            &config.secret.unwrap_or_default()
        );
        let response = Client::new().get(url).send().await.expect("链接错误");
        response.json().await
    }

    pub async fn get() -> String {
        let in_redis = redis_get_access_token().await.unwrap_or_default();
        match in_redis {
            Some(access_token) => {
                log_info!("redis access_token {}", &access_token);
                access_token
            }
            None => {
                let token = Self::from_server().await.unwrap().get_token();
                log_info!("get_access_token {}", &token);
                redis_set_access_token(token.clone()).await;
                token
            }
        }
    }
}

pub async fn redis_set_access_token(access_token: String) {
    let config = Config::get().await.redis.unwrap_or_default();
    let mut con = config.get_redis_conn().await.unwrap();
    let set = con
        .set::<&str, String, usize>("snipe_wx_access_token", access_token)
        .unwrap();
    let expire = con
        .expire::<&str, usize>("snipe_wx_access_token", config.expire.unwrap_or_default())
        .unwrap();
    log_info!("SET {set} | EXPIRE {expire}");
}

pub async fn redis_get_access_token() -> RedisResult<Option<String>> {
    let mut con = Config::get()
        .await
        .get_redis_conn()
        .await
        .expect("Redis链接失败");
    con.get::<_, Option<String>>("snipe_wx_access_token")
}
