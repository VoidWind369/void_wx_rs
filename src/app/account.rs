use reqwest::{Client};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app::{Config, get_config};
use crate::{log_error, log_info};

pub type SnipeAccounts = Vec<SnipeAccount>;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct SnipeAccount {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub r#type: Option<i64>,
    pub status: Option<i64>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

pub async fn to_acc(name: &str, tag: &str) -> String {
    let config: Config = get_config().await;
    let tag = format!("#{}", tag.replace("#", ""));
    let url = format!("{}/oa_to", config.server_url.unwrap());
    log_info!("请求 {}", &url);
    let json = json!({
        "name": name,
        "tag": tag,
        "type": 2
    });
    let response = Client::new().post(url)
        .json(&json).send().await;
    match response {
        Ok(re) => {
            let r = re.text().await.unwrap();
            if r.contains("1") {
                "提交成功，等待管理审核".to_string()
            } else {
                "警告，请检查是否重复提交".to_string()
            }
        }
        Err(e) => {
            log_error!("{e}");
            "失败，请检查是否重复提交".to_string()
        }
    }
}

pub async fn list_wait_acc(name: &str) -> SnipeAccounts {
    let config: Config = get_config().await;
    let admin_acc = search_acc(name, 1).await;
    match admin_acc.r#type {
        Some(1) => {
            let url = format!("{}/oa_wait", config.server_url.unwrap());
            log_info!("请求 {}", &url);
            let response = Client::new().get(url).send().await;
            response.unwrap().json::<SnipeAccounts>().await.unwrap_or_default()
        }
        _ => {
            vec![]
        }
    }
}

pub async fn search_acc(name: &str, status: i64) -> SnipeAccount {
    let config: Config = get_config().await;
    let url = format!("{}/oa_search", config.server_url.unwrap());
    log_info!("请求 {}", &url);
    let json = json!({
        "name": name,
        "status": status
    });
    let response = Client::new().post(url).json(&json).send().await;
    match response {
        Ok(re) => {
            re.json::<SnipeAccount>().await.unwrap_or_default()
        }
        Err(e) => {
            log_error!("{e}");
            Default::default()
        }
    }
}

pub async fn join_acc(id: i64, admin_name: &str) -> String {
    let config: Config = get_config().await;
    let admin_acc = search_acc(admin_name, 1).await;
    if Some(1) != admin_acc.r#type {
        return "无权限".to_string();
    };
    let url = format!("{}/oa_set", config.server_url.unwrap());
    log_info!("请求 {}", &url);
    let json = json!({
        "id": id,
        "status": 1
    });
    let response = Client::new().post(url)
        .json(&json).send().await;
    match response {
        Ok(re) => {
            let r = re.text().await.unwrap();
            if r.contains("1") {
                "修改成功".to_string()
            } else {
                "无变化".to_string()
            }
        }
        Err(e) => {
            log_error!("{e}");
            format!("{e}")
        }
    }
}