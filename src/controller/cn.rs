use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use axum_xml_up::Xml;

use crate::app::{account, snipe, WxResponse, WxSendText};
use crate::controller::sign;
use crate::log_info;

pub async fn cn(Xml(res): Xml<WxResponse>) -> impl IntoResponse {
    // a211f6ccb1d1339f3bf89506ddf90f90
    log_info!("{:?}" ,&res);
    let from_user_name = res.clone().from_user_name.unwrap_or("none".to_string());
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
            let mut base_vec = vec!["加盟#标签", "fwa/gfl", "查部落#标签", "对战日志#标签"];
            let vec = match acc.r#type {
                Some(1) => {
                    base_vec.append(&mut vec!["更新#标签", "信息", "待审核", "审核#标签", "fwa/gfl#新时间"]);
                    base_vec
                }
                Some(2) => {
                    base_vec.append(&mut vec!["更新#标签", "信息", "fwa"]);
                    base_vec
                }
                _ => base_vec
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
        if msg.contains("更新#") {
            let tag = msg.split("#").collect::<Vec<&str>>()[1];
            let accounts = match tag.len() {
                0..=3 => { "标签错误".to_string() }
                _ => { account::up_acc(&from_user_name, tag).await }
            };
            wx_send_text.content = Some(accounts);
        }
        if msg.eq("信息") {
            let mut strs = String::from("【信息】");
            let acc = account::search_acc(&from_user_name, 1).await;
            strs.push_str("\n");
            strs.push_str(&format!("微信: {}", acc.name.unwrap_or_default()));
            strs.push_str("\n");
            strs.push_str(&format!("标签: {}", acc.tag.unwrap_or_default()));
            strs.push_str("\n");
            strs.push_str(&format!("部落: {}", acc.clan_name.unwrap_or_default()));
            wx_send_text.content = Some(strs);
        }
        if msg.eq("待审核") {
            let accounts = account::list_wait_acc(&from_user_name).await;
            let mut strs = String::from("【待审核】");
            for acc in accounts {
                let str = format!("【{}】 | {}", acc.id.unwrap(), acc.tag.unwrap());
                strs.push_str("\n");
                strs.push_str("---------------");
                strs.push_str("\n");
                strs.push_str(&str);
                strs.push_str("\n");
                strs.push_str(&acc.clan_name.unwrap_or_default());
                strs.push_str("\n");
                strs.push_str(&acc.name.unwrap_or_default());
                strs.push_str("\r\n");
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
        if msg.eq("fwa") || msg.eq("fwl") || msg.eq("gfl") {
            let acc = account::search_acc(&from_user_name, 1).await;
            let time_id = match msg.as_str() {
                "fwa" => 81,
                "fwl" => 82,
                "gfl" => 83,
                _ => 0
            };
            let str = match acc.r#type {
                Some(1..=2) => {
                    let time = snipe::ListTime::get_time(time_id).await;
                    time.format_time().await
                }
                _ => { "未加盟".to_string() }
            };
            wx_send_text.content = Some(str);
        }
        if msg.starts_with("fwa#") || msg.starts_with("fwl#") || msg.starts_with("gfl#") {
            let acc = account::search_acc(&from_user_name, 1).await;
            let time_str = msg.split("#").collect::<Vec<&str>>();
            let time_id = match time_str[0] {
                "fwa" => 81,
                "fwl" => 82,
                "gfl" => 83,
                _ => 0
            };
            let text = match acc.r#type {
                Some(1) => {
                    let fmt_time = time_str[1].replace("：", ":");
                    snipe::ListTime::set_time(time_id, &fmt_time).await
                }
                _ => "无权限".to_string()
            };
            wx_send_text.content = Some(text);
        }
        if msg.starts_with("查部落#") {
            let tag = msg.split("#").collect::<Vec<&str>>();
            let clan_info = account::coc_clan_info(tag[1]).await;
            wx_send_text.content = Some(clan_info);
        }
        if msg.starts_with("对战日志#") {
            let tag = msg.split("#").collect::<Vec<&str>>();
            let clan_info = account::coc_war_log(tag[1]).await;
            wx_send_text.content = Some(clan_info);
        }
    }
    log_info!("{wx_send_text:?}");
    let xml = serde_xml_rs::to_string(&wx_send_text).unwrap();
    xml.trim_start_matches("<?xml version=\"1.0\" encoding=\"UTF-8\"?>").replace("WxSendText", "xml")
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/cn", get(sign).post(cn))
}