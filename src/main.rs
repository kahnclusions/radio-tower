use std::net::SocketAddr;

use crate::app::RootProps;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::Query;
use axum::routing::get;
use axum::{extract::WebSocketUpgrade, response::Html, Router};
use dioxus_interpreter_js::INTERPRETER_JS;
use rust_embed::RustEmbed;
use serde::Deserialize;

pub mod app;
pub mod transmission;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

static MAIN_JS: &str = include_str!("./main.js");

pub fn interpreter_glue(url: &str) -> String {
    format!(
        r#"
<script>
    var WS_ADDR = "{url}";
    {INTERPRETER_JS}
    {MAIN_JS}
    main();
</script>
    "#
    )
}

#[derive(Deserialize)]
struct WebSocketQuery {
    initial_route: Option<String>,
}

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([10, 0, 0, 171], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();
    let tailwind_css = Asset::get("tailwind.css").unwrap();
    let html = format!(
        r#"
                <!DOCTYPE html>
                <html class="bg-beige-800 dark:bg-black h-full">
                <head> 
                  <title>Dioxus LiveView with Axum</title>  
                  <meta name="viewport" content="width=device-width, initial-scale=1" />
                  <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Silkscreen">
                  <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Noto+Sans">
                  <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Noto+Serif">
                  <style>
                  {style}
                  </style>
                </head>
                <body class="bg-beige-800 dark:bg-black dark:text-white h-full"> <div id="main" class="h-full"></div> </body>
                {glue}
                </html>
                "#,
        // Create the glue code to connect to the WebSocket on the "/ws" route
        glue = interpreter_glue(&format!("ws://{addr}/ws")),
        style = std::str::from_utf8(tailwind_css.data.as_ref()).unwrap()
    );
    let html_root = html.clone();
    let html_path = html.clone();

    let router = Router::new()
        .route(
            "/ws",
            get(
                move |ws: WebSocketUpgrade,
                      query: Query<WebSocketQuery>,
                      ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
                    let initial_route = query.initial_route.clone().unwrap_or("/".to_string());
                    ws.on_upgrade(move |socket| async move {
                        println!("WebSocket [{:#?}]: Accepted connection", addr);
                        _ = view
                            .launch_with_props(
                                dioxus_liveview::axum_socket(socket),
                                app::root,
                                app::rootProps { initial_route },
                            )
                            .await;
                        println!("WebSocket [{:#?}]: Connection dropped", addr);
                    })
                },
            ),
        )
        .route("/*rest", get(move || async move { Html(html_path) }))
        .route("/", get(move || async move { Html(html_root) }));

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
