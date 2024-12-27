use axum::response::IntoResponse;
use axum::routing::*;
use axum::{Json, Router};
use axum_xml_up::Xml;

use crate::app::{send_create_menu, Config, WxResponse, WxSendText};
use crate::controller::sign;
use crate::log_info;

pub async fn wx(Xml(res): Xml<WxResponse>) -> impl IntoResponse {
    let api = Config::get().await.api.unwrap_or_default();
    log_info!("{:?}", res);
    let mut wx_send_text = WxSendText::new();
    if let Some(msg) = res.content {
        wx_send_text = WxSendText {
            to_user_name: res.from_user_name,
            from_user_name: res.to_user_name,
            create_time: res.create_time,
            msg_type: Some("text".to_string()),
            content: Some("一个瑶瑶，两个瑶瑶。。。".to_string()),
        };
        if msg.eq("时间") {
            let times = api.get_list_time().await;
            let mut times_str = String::from("【时间集】");
            for time in times {
                let str = time.format_time().await;
                times_str.push_str("\n");
                times_str.push_str(&str)
            }
            wx_send_text.content = Some(times_str);
        }
    }
    log_info!("{wx_send_text:?}");
    let xml = serde_xml_rs::to_string(&wx_send_text).unwrap();
    xml.trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>").replace("WxSendText", "xml")
    // Xml(wx_send_text)
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