mod controller;
mod log;
mod app;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    controller::run().await;
}
