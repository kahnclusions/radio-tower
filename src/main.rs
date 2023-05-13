use std::net::SocketAddr;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::Query;
use axum::routing::get;
use axum::{extract::WebSocketUpgrade, response::Html, Router};
use clap::Parser;
use color_eyre::Report;
use dioxus_interpreter_js::INTERPRETER_JS;
use serde::Deserialize;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;
use user_config::load_config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file to load
    #[arg(short, long, default_value_t = String::from("$HOME/.config/radio-tower.toml"))]
    config: String,
}

pub mod app;
pub mod transmission;
pub mod user_config;

static TAILWIND_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/tailwind.css"));
static MAIN_JS: &'static str = include_str!("./main.js");

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

pub fn tracing_setup() -> Result<(), Report> {
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("radio-tower")
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .with(telemetry)
        .init();

    Ok(())
}

pub fn tracing_teardown() {
    opentelemetry::global::shutdown_tracer_provider();
}

#[derive(Deserialize)]
struct WebSocketQuery {
    initial_route: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    tracing_setup()?;

    info!("Starting radio-tower");

    let args = Args::parse();
    let config = load_config(&args.config)?;

    debug!("Loaded config: {:?}", &config);

    let addr: std::net::SocketAddr = ([10, 0, 0, 171], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();
    // let tailwind_css = Asset::get("tailwind.css").unwrap();
    let html = format!(
        r#"
                <!DOCTYPE html>
                <html class="bg-beige-800 dark:bg-black h-full">
                <head> 
                  <title>radio-tower</title>  
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
        style = TAILWIND_CSS
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
                        info!("WebSocket [{:#?}]: Accepted connection", addr);
                        _ = view
                            .launch_with_props(
                                dioxus_liveview::axum_socket(socket),
                                app::root,
                                app::rootProps { initial_route },
                            )
                            .await;
                        info!("WebSocket [{:#?}]: Connection dropped", addr);
                    })
                },
            ),
        )
        .route("/*rest", get(move || async move { Html(html_path) }))
        .route("/", get(move || async move { Html(html_root) }));

    info!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    tracing_teardown();
    Ok(())
}
