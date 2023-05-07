use std::time::Duration;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::transmission::client::{SessionStats, Torrent, TorrentSummary};

use mini_torrent::MiniTorrent;
use stats_bar::StatsBar;

pub mod mini_torrent;
pub mod stats_bar;
pub mod ui;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    message: String,
    status: String,
}

pub fn root(cx: Scope) -> Element {
    let stats = use_state::<Option<SessionStats>>(cx, || None);
    let torrents = use_state::<Vec<TorrentSummary>>(cx, || Vec::new());
    let _ws: &Coroutine<()> = use_coroutine(cx, |_rx| {
        let stats = stats.to_owned();
        let transmission = crate::transmission::client::ClientBuilder::new()
            .transmission_url("http://localhost:9091/transmission/rpc".to_string())
            .build()
            .unwrap();
        async move {
            loop {
                let response = transmission.session_stats().await.unwrap();
                stats.set(Some(response.arguments));
                sleep(Duration::from_secs(1)).await;
            }
        }
    });

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
            class: "sticky top-0 left-0 right-0 h-[40px] bg-beige-800 text-center font-display flex flex-row items-center justify-center text-2xl dark:bg-grey-200",
            "radio-tower"
        },
        div {
            class: "flex flex-col gap-2",
            torrents.iter().map(|torrent| cx.render(rsx!(
                MiniTorrent {
                    torrent: &torrent
                }
            )))
        },
        footer {
            class: "fixed bottom-0 left-0 right-0 h-[40px] bg-beige-800 dark:bg-grey-200",
            StatsBar { stats: stats }
        }
    ))
}
