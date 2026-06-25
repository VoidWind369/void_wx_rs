use crate::agent::ollama;
use crate::app::{send_create_menu, WxResponse, WxSendText};
use crate::controller::AppState;
use axum::extract::State;
use axum::{response::IntoResponse, Json};
use axum_serde::Xml;
use void_log::log_info;

impl WxResponse {
    async fn wx_start(self, app_state: &mut AppState) -> WxSendText {
        let from_user_name = self.clone().from_user_name.unwrap_or("none".to_string());
        let mut wx_send_text = WxSendText::new();
        if let Some(msg) = self.content {
            // ai模型回复
            let vec = app_state.messages.get_mut(&from_user_name).unwrap();
            let content = ollama::agent_run(&msg, vec.clone()).await;
            wx_send_text = WxSendText {
                to_user_name: self.from_user_name,
                from_user_name: self.to_user_name,
                create_time: self.create_time,
                msg_type: Some("text".to_string()),
                content: Some(content.unwrap_or_default()),
            };
            // let api = Config::get().await.api.unwrap_or_default();
            // if msg.eq("时间") {
            //     let times = api.get_list_time().await;
            //     let mut times_str = String::from("【时间集】");
            //     for time in times {
            //         let str = time.format_time().await;
            //         times_str.push_str("\n");
            //         times_str.push_str(&str)
            //     }
            //     wx_send_text.content = Some(times_str);
            // }
        }
        wx_send_text
    }
}

pub async fn wx(State(mut app_state): State<AppState>, res: String) -> impl IntoResponse {
    log_info!("{}", res);
    let res = serde_xml_rs::from_str::<WxResponse>(&res).unwrap();
    let wx_send_text = res.wx_start(&mut app_state).await;
    log_info!("{wx_send_text}");
    Xml(wx_send_text)
}

pub async fn create_menu() -> impl IntoResponse {
    let res = send_create_menu().await;
    Json(res)
}
