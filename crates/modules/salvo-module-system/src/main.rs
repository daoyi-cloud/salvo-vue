use salvo::Router;
use salvo_common::app;

#[tokio::main]
async fn main() {
    app::run(Router::new()).await
}
