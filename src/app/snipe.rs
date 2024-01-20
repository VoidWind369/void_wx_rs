use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::log_info;

type ListTimes = Vec<ListTime>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTime {
    pub union: String,
    pub day: String,
    pub time: String,
}

impl ListTime {
    pub async fn get() -> ListTimes {
        set_xin().await;
        let response = Client::new()
            .get("http://get.cocsnipe.top/listTime".to_string())
            .send()
            .await
            .unwrap();
        response.json::<ListTimes>().await.expect("List Time Failed")
    }

    pub async fn format_time(&self) -> String {
        format!("{}: {} {}", self.union, self.day, self.time)
    }
}

async fn set_xin() {
    let response = Client::new()
        .get("http://get.cocsnipe.top/setXm")
        .send()
        .await
        .unwrap();
    log_info!("鑫盟{}", response.text().await.unwrap_or("没有更新".to_string()))
}

pub async fn get_aw_qdm() -> [String; 2] {
    let response = Client::new()
        .get("http://get.cocsnipe.top/aw")
        .send().await.expect("getAwErr");
    let res = response.json().await.unwrap();
    log_info!("启动码{:?}", res);
    res
}