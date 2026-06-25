use crate::agent::ollama;
use crate::app::{send_create_menu, Config, WxResponse, WxSendText};
use crate::controller::sign;
use axum::{response::IntoResponse, routing::*, Json, Router};
use axum_serde::Xml;
use void_log::log_info;

impl WxResponse {
    async fn wx_start(self) -> WxSendText {
        let mut wx_send_text = WxSendText::new();
        if let Some(msg) = self.content {
            // ai模型回复
            let content = ollama::agent_run(&msg).await;
            wx_send_text = WxSendText {
                to_user_name: self.from_user_name,
                from_user_name: self.to_user_name,
                create_time: self.create_time,
                msg_type: Some("text".to_string()),
                content: Some(content),
            };

            let content = ollama::agent_run(&msg).await;
            wx_send_text.content = Some(content);
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

async fn wx(res: String) -> impl IntoResponse {
    log_info!("{}", res);
    let res = serde_xml_rs::from_str::<WxResponse>(&res).unwrap();
    let wx_send_text = res.wx_start().await;
    log_info!("{wx_send_text}");
    Xml(wx_send_text)
}

async fn create_menu() -> impl IntoResponse {
    let res = send_create_menu().await;
    Json(res)
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/wx", get(sign).post(wx))
        .route("/create_menu", get(create_menu))
}
