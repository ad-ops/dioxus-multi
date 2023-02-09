// use axum::{response::Html, routing::get, Router, http::StatusCode};
// use dioxus::prelude::*;
// use std::net::SocketAddr;
// use app::app;

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .nest("/test", get(app_endpoint))
//         .fallback(
//             get(app_endpoint).handle_error(|_| async move {
//                 (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
//             }),
//         );
//         //.route("/*path", get(app_endpoint));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("listening on http://{}", addr);
//     let start_result = axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await;
//     match start_result {
//         Ok(_) => (),
//         Err(e) => eprintln!("error starting webserver: {}", e),
//     }
// }

// #[allow(clippy::unused_async)]
// async fn app_endpoint() -> Html<String> {
//     // let mut app = VirtualDom::new(app);
//     // let _mutations = app.rebuild();

//     let mut dom = VirtualDom::new(app);
//     let _mutations = dom.rebuild();
//     let text = dioxus::ssr::render_vdom_cfg(&dom, |cfg| cfg.pre_render(true));
//     Html(text)

//     // Html(dioxus::ssr::render_vdom(&app))
// }

use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use app::app;

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        .route(
            "/",
            get(move || async move {
                Html(format!(
                    r#"
            <!DOCTYPE html>
            <html>
                <head> <title>Dioxus LiveView with Warp</title>  </head>
                <body> <div id="main"></div> </body>
                {glue}
            </html>
            "#,
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    _ = view.launch(dioxus_liveview::axum_socket(socket), app).await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}