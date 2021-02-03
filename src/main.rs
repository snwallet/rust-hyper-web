mod app;
#[tokio::main]
async fn main() {
    app::run(8888).await;
}