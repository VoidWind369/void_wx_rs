mod controller;
mod app;

#[tokio::main]
async fn main() {
    controller::run().await;
}
