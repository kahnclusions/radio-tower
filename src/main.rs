use std::net::SocketAddr;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::WebSocket;
use axum::routing::get;
use axum::{extract::WebSocketUpgrade, response::Html, Router};
use dioxus_interpreter_js::INTERPRETER_JS;
use rust_embed::RustEmbed;
use transmission::client::{GetSessionRequest, GetSessionResponse, Request, Response};

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

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([10, 0, 0, 171], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();
    let tailwind_css = Asset::get("tailwind.css").unwrap();
    let router = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move || async move {
                Html(format!(
                    r#"
                <!DOCTYPE html>
                <html class="bg-white dark:bg-black">
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
                <body class="bg-white dark:bg-black dark:text-white"> <div id="main"></div> </body>
                {glue}
                </html>
                "#,
                    // Create the glue code to connect to the WebSocket on the "/ws" route
                    glue = interpreter_glue(&format!("ws://{addr}/ws")),
                    style = std::str::from_utf8(tailwind_css.data.as_ref()).unwrap()
                ))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(
                move |ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<SocketAddr>| async move {
                    ws.on_upgrade(move |socket| async move {
                        println!("WebSocket [{:#?}]: Accepted connection", addr);
                        _ = view
                            .launch(dioxus_liveview::axum_socket(socket), app::root)
                            .await;
                        println!("WebSocket [{:#?}]: Connection dropped", addr);
                    })
                },
            ),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
