use axum::{Json, Router};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::*;
use axum_xml_up::Xml;

use crate::app::{account, send_create_menu, snipe, WxResponse, WxSendText, WxSign};
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
            content: Some("一个瑶瑶，两个瑶瑶。。。".to_string()),
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
    let xml = serde_xml_rs::to_string(&wx_send_text).unwrap();
    xml.trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>").replace("WxSendText", "xml")

    // Xml(wx_send_text)
}

pub async fn cn(Xml(res): Xml<WxResponse>) -> impl IntoResponse {
    // a211f6ccb1d1339f3bf89506ddf90f90
    log_info!("{:?}" ,&res);
    let from_user_name = res.clone().to_user_name.unwrap_or("none".to_string());
    let mut wx_send_text = WxSendText::new();
    if let Some(msg) = res.content {
        wx_send_text = WxSendText {
            to_user_name: res.from_user_name,
            from_user_name: res.to_user_name,
            create_time: res.create_time,
            msg_type: Some("text".to_string()),
            content: Some("啊～不要。。。".to_string()),
        };
        if msg.eq("指令") {
            let mut strs = String::from("【指令】");
            let acc = account::search_acc(&from_user_name, 1).await;
            let vec = match acc.r#type {
                Some(1) => vec!["加盟#标签", "待审核", "审核#标签", "fwa", "fwa#"],
                _ => vec!["加盟#标签", "fwa"]
            };
            for ve in vec {
                strs.push_str("\n");
                strs.push_str(&ve)
            }
            wx_send_text.content = Some(strs);
        }
        if msg.contains("加盟#") {
            let tag = msg.split("#").collect::<Vec<&str>>()[1];
            let accounts = match tag.len() {
                0..=3 => { "标签错误".to_string() }
                _ => { account::to_acc(&from_user_name, tag).await }
            };
            wx_send_text.content = Some(accounts);
        }
        if msg.eq("待审核") {
            let accounts = account::list_wait_acc(&from_user_name).await;
            let mut strs = String::from("【待审核】");
            for acc in accounts {
                let str = format!("{} | {}", acc.id.unwrap(), acc.name.unwrap());
                strs.push_str("\n");
                strs.push_str(&str)
            }
            wx_send_text.content = Some(strs);
        }
        if msg.contains("审核#") {
            let id = msg.split("#").collect::<Vec<&str>>()[1].parse::<i64>();
            let accounts = match id {
                Ok(i) => { account::join_acc(i, &from_user_name).await }
                Err(_) => { "无对象".to_string() }
            };
            wx_send_text.content = Some(accounts);
        }
        if msg.eq("fwa") {
            let acc = account::search_acc(&from_user_name, 1).await;
            let str = match acc.r#type {
                Some(1..=2) => {
                    let time = snipe::ListTime::get_time(81).await;
                    time.format_time().await
                }
                _ => { "未加盟".to_string() }
            };
            wx_send_text.content = Some(str);
        }
        if msg.starts_with("fwa#") {
            let acc = account::search_acc(&from_user_name, 1).await;
            let time_str = msg.split("#").collect::<Vec<&str>>();
            let text = match acc.r#type {
                Some(1) => {
                    let fmt_time = time_str[1].replace("：", ":");
                    snipe::ListTime::set_time(81, &fmt_time).await
                }
                _ => "无权限".to_string()
            };
            wx_send_text.content = Some(text);
        }
    }
    log_info!("{wx_send_text:?}");
    let xml = serde_xml_rs::to_string(&wx_send_text).unwrap();
    xml.trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>").replace("WxSendText", "xml")
}

async fn create_menu() -> impl IntoResponse {
    let res = send_create_menu().await;
    Json(res)
}

async fn sign(Query(res): Query<WxSign>) -> impl IntoResponse {
    log_info!("{:?}", res);
    let mut str = String::new();
    if let Some(echo) = res.sign() {
        str = echo
    }
    log_info!("{str}");
    str
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/wx", get(sign).post(wx))
        .route("/cn", get(sign).post(cn))
        .route("/create_menu", get(create_menu))
}