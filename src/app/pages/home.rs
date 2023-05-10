use std::time::Duration;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::app::mini_torrent::MiniTorrent;
use crate::app::stats_bar::StatsBar;
use crate::transmission::client::TorrentSummary;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    message: String,
    status: String,
}

pub fn Home(cx: Scope) -> Element {
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
                sleep(Duration::from_secs(1)).await;
            }
        }
    });

    cx.render(rsx!(
        header {
            class: "fixed top-0 left-0 right-0 h-[40px] bg-beige-800 text-center font-display flex flex-row items-center justify-center text-2xl dark:bg-grey-200",
            "radio-tower"
        }
        main {
            class: "flex flex-col gap-1 m-1 mt-[44px] mb-[44px]",
            torrents.iter().map(|torrent| cx.render(rsx!(
                MiniTorrent {
                    torrent: &torrent
                }
            )))
        }
        footer {
            class: "fixed bottom-0 left-0 right-0 h-[40px] bg-beige-800 dark:bg-grey-200",
            StatsBar {}
        }
    ))
}
