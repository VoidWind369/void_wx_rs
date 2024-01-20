use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::app::redis_util::redis_get_access_token;
use crate::{log_error, log_info};
use crate::app::get_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WxResponse {
    pub to_user_name: Option<String>,
    pub from_user_name: Option<String>,
    pub create_time: Option<i64>,
    pub msg_type: Option<String>,
    pub content: Option<String>,
    pub msg_id: Option<i64>,
    pub msg_data_id: Option<String>,
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WxSendText {
    pub to_user_name: Option<String>,
    pub from_user_name: Option<String>,
    pub create_time: Option<i64>,
    pub msg_type: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ResAccessToken {
    access_token: Option<String>,
    expires_in: Option<i64>,
    err_code: Option<String>,
    err_msg: Option<String>,
}

impl WxSendText {
    pub fn new() -> Self {
        Self {
            to_user_name: None,
            from_user_name: None,
            create_time: None,
            msg_type: None,
            content: None,
        }
    }
}

async fn get_access_token_from_server() -> Option<String> {
    let config = get_config().await;
    let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}", config.wx_appid, config.wx_secret);
    let response = Client::new().get(url).send().await;
    match response {
        Ok(r) => {
            let res_str = r.json::<ResAccessToken>().await.unwrap();
            log_info!("{res_str:?}");
            res_str.access_token
        }
        Err(e) => {
            log_error!("Send Error {e}");
            None
        }
    }
}

pub async fn get_access_token() -> String {
    let in_redis = redis_get_access_token().await.unwrap_or(None);
    match in_redis {
        Some(access_token) => {
            access_token
        }
        None => {
            get_access_token_from_server().await.unwrap_or("".to_string())
        }
    }
}

pub async fn send_text(wx_send_text: WxSendText) {
    let str = serde_xml_rs::to_string(&wx_send_text).expect("xml to string error");
    let response = Client::new().post("api.weixin.qq.com")
        .body(str).send().await;
    match response {
        Ok(r) => {
            let res_str = r.text().await.unwrap_or("No Search".to_string());
            log_info!("{res_str}")
        }
        Err(e) => {
            log_error!("Send Error {e}")
        }
    };
}