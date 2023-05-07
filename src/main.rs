use axum::routing::get;
use axum::{extract::WebSocketUpgrade, response::Html, Router};
use rust_embed::RustEmbed;
use transmission::client::{GetSessionRequest, GetSessionResponse, Request, Response};

pub mod app;
pub mod transmission;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

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
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws")),
                    style = std::str::from_utf8(tailwind_css.data.as_ref()).unwrap()
                ))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view
                        .launch(dioxus_liveview::axum_socket(socket), app::root)
                        .await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
