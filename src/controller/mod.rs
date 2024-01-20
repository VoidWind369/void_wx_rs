use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tower_http::cors::CorsLayer;
use crate::*;

mod wx;

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}

pub async fn run() {
    let Some(port) = app::get_config().await.server_port else { panic!("config port not fount") };
    let address = format!("0.0.0.0:{}", port);
    log_info!("启动参数: {address}");

    let mut app = Router::new()
        .fallback(handler_404);

    app = wx::router(app).await;
    app = app.layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}