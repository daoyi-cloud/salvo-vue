use salvo::prelude::*;
use serde::Serialize;
mod routers;
use salvo_common::app;

#[tokio::main]
async fn main() {
    app::run(routers::root()).await
}
