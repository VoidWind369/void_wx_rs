use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
            .send().await.unwrap();
        response.json::<ListTimes>().await.expect("List Time Failed")
    }

    pub async fn get_time(time_id: u64) -> Self {
        let response = Client::new()
            .get(format!("http://get.cocsnipe.top/getTime/{}", time_id))
            .send().await.unwrap();
        response.json::<Self>().await.expect("Get Time Failed")
    }

    pub async fn set_time(time_id: u64, time: &str) -> String {
        let json = json!({
            "id": time_id,
            "time": time
        });
        let response = Client::new()
            .post("http://get.cocsnipe.top/setTime")
            .json(&json)
            .send().await.unwrap();
        response.json::<String>().await.expect("Set Time Failed")
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