use crate::app::{Config, WxSign};
use crate::controller::{cn::*, wx::*};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use dashmap::DashMap;
use rig::message::Message;
use tower_http::cors::CorsLayer;
use void_log::log_info;

mod cn;
mod wx;

#[derive(Clone, Default)]
pub struct AppState {
    pub messages: DashMap<String, Vec<Message>>,
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

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}

pub async fn run() {
    let app_state = AppState::default();

    let server = Config::get().await.server.unwrap_or_default();
    let port = server.port.unwrap_or(9000);
    let address = format!("0.0.0.0:{}", port);
    log_info!("启动参数: {address}");

    let app = Router::new()
        .fallback(handler_404)
        .route("/cn", get(sign).post(cn))
        .route("/cn_new", get(sign).post(cn_new))
        .route("/cn_test", post(cn_test))
        .route("/wx", get(sign).post(wx))
        .route("/create_menu", get(create_menu))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
