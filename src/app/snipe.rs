use crate::app::ConfigApi;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

type ListTimes = Vec<ListTime>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTime {
    pub union: String,
    pub day: String,
    pub time: String,
}

impl ConfigApi {
    pub async fn get_list_time(&self) -> ListTimes {
        let response = Client::new()
            .get(format!(
                "{}/get/listTime",
                self.url.clone().unwrap_or_default()
            ))
            .send()
            .await
            .unwrap();
        response
            .json::<ListTimes>()
            .await
            .expect("List Time Failed")
    }

    pub async fn get_time(&self, time_id: u64) -> ListTime {
        let response = Client::new()
            .get(format!(
                "{}/get/getTime/{}",
                self.url.clone().unwrap_or_default(),
                time_id
            ))
            .send()
            .await
            .unwrap();
        response.json().await.expect("Get Time Failed")
    }

    pub async fn set_time(&self, time_id: u64, time: &str) -> String {
        let json = json!({
            "id": time_id,
            "time": time
        });
        let response = Client::new()
            .post(format!(
                "{}/get/setTime",
                self.url.clone().unwrap_or_default()
            ))
            .json(&json)
            .send()
            .await
            .unwrap();
        response.json::<String>().await.expect("Set Time Failed")
    }
}

impl ListTime {
    pub async fn format_time(&self) -> String {
        format!("{}: {} {}", self.union, self.day, self.time)
    }
}
