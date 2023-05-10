use std::time::Duration;

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::transmission::client::TorrentSummary;
use pages::Home;
use pages::Torrent;

pub mod mini_torrent;
pub mod pages;
pub mod stats_bar;
pub mod ui;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    message: String,
    status: String,
}

pub struct RootProps {
    pub initial_route: String,
}

pub fn root(cx: Scope<RootProps>) -> Element {
    let torrents = use_state::<Vec<TorrentSummary>>(cx, || Vec::new());
    let _torrents: &Coroutine<()> = use_coroutine(cx, |_rx| {
        let torrents = torrents.to_owned();
        let transmission = crate::transmission::client::ClientBuilder::new()
            .transmission_url("http://localhost:9091/transmission/rpc".to_string())
            .build()
            .unwrap();
        async move {
            loop {
                let response = transmission.torrent_summary().await.unwrap();
                torrents.set(response.arguments.torrents);
                sleep(Duration::from_secs(2)).await;
            }
        }
    });

    cx.render(rsx!(
        Router {
            initial_url: format!("http://10.0.0.171:3030{}", cx.props.initial_route.clone()),
            Route { to: "/", Home {} },
            Route { to: "/torrent", Torrent {} }
        }
    ))
}
