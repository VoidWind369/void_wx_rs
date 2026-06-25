mod agent;
mod app;
mod controller;
mod wx_api;

#[tokio::main]
async fn main() {
    controller::run().await;
}
