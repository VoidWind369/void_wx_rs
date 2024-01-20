use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::*;
use axum_xml_up::Xml;
use crate::app::{snipe, WxResponse, WxSendText, WxSign};
use crate::log_info;

pub async fn wx(Xml(res): Xml<WxResponse>) -> impl IntoResponse {
    log_info!("{:?}", res);
    let mut wx_send_text = WxSendText::new();
    if let Some(msg) = res.content {
        wx_send_text = WxSendText {
            to_user_name: res.from_user_name,
            from_user_name: res.to_user_name,
            create_time: res.create_time,
            msg_type: Some("text".to_string()),
            content: None,
        };
        if msg.eq("时间") {
            let times = snipe::ListTime::get().await;
            let mut times_str = String::from("【时间集】");
            for time in times {
                let str = time.format_time().await;
                times_str.push_str("\n");
                times_str.push_str(&str)
            }
            wx_send_text.content = Some(times_str);
        }

        if msg.eq("爱玩") || msg.eq("启动码") {
            let qdm = snipe::get_aw_qdm().await;
            let str = format!("启动码: {}\n下次刷新: {}", qdm[0], qdm[1]);
            wx_send_text.content = Some(str);
        }
    }
    log_info!("{wx_send_text:?}");
    Xml(wx_send_text)
}

async fn sign(Query(res): Query<WxSign>) -> impl IntoResponse {
    log_info!("{:?}", res);
    let mut str = String::new();
    if let Some(echo) = res.sign() {
        str = echo
    }
    str
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/wx", get(sign).post(wx))
}