use crate::app::{Config, WxSign};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tower_http::cors::CorsLayer;
use void_log::log_info;

mod cn;
mod wx;

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
    let server = Config::get().await.server.unwrap_or_default();
    let port = server.port.unwrap_or(9000);
    let address = format!("0.0.0.0:{}", port);
    log_info!("启动参数: {address}");

    let mut app = Router::new().fallback(handler_404);

    app = wx::router(app).await;
    app = cn::router(app).await;
    app = app.layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
