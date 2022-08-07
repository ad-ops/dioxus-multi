use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;
use std::net::SocketAddr;
use app::app;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(app_endpoint));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let start_result = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;
    match start_result {
        Ok(_) => println!("listening on http://{}", addr),
        Err(e) => eprintln!("error starting webserver: {}", e),
    }
}

#[allow(clippy::unused_async)]
async fn app_endpoint() -> Html<String> {
    let mut app = VirtualDom::new(app);
    let _mutations = app.rebuild();
    
    Html(dioxus::ssr::render_vdom(&app))
}