#![allow(non_snake_case)]
use std::cmp::Ordering;
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::{
    IoCloseCircleOutline, IoCloseOutline, IoFilterOutline, IoFunnelOutline, IoTrendingDownOutline,
    IoTrendingUpOutline,
};
use dioxus_free_icons::Icon;
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

enum TorrentSort {
    BySize,
    ByName,
    ByProgress,
    ByStatus,
}

enum Order {
    Asc,
    Desc,
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
                sleep(Duration::from_secs(2)).await;
            }
        }
    });

    let torrent_filter = use_state(cx, || "".to_string());
    let torrent_sort = use_state(cx, || TorrentSort::ByName);
    let torrent_order = use_state(cx, || Order::Asc);
    let mut torrents: Vec<_> = torrents
        .iter()
        .filter_map(|torrent| {
            if torrent.name.contains(torrent_filter.as_str()) {
                Some(torrent)
            } else {
                None
            }
        })
        .collect();

    torrents.sort_by(move |a, b| match *(torrent_sort.current()) {
        TorrentSort::ByName => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        TorrentSort::BySize => a.size_when_done.total_cmp(&b.size_when_done),
        TorrentSort::ByProgress => a.percent_done.total_cmp(&b.percent_done),
        TorrentSort::ByStatus => a.status.partial_cmp(&b.status).unwrap_or(Ordering::Equal),
    });

    if matches!(*torrent_order.current(), Order::Desc) {
        torrents.reverse();
    }

    render! {
        header { class: "fixed top-0 left-0 right-0 h-[40px] bg-beige-800 text-center font-display flex flex-row items-center justify-center text-2xl dark:bg-grey-200",
            "radio-tower"
        }
        div {
            class: "fixed top-[40px] left-0 right-0 h-[40px] flex flex-row gap-1 justify-between items-center p-2 bg-beige-900",
            Icon { width: 16, height: 16, icon: IoFunnelOutline }
            input {
                class: "",
                value: "{torrent_filter}",
                oninput: move |ev| torrent_filter.set(ev.value.clone()),
            }
            button {
                onclick: move |_| torrent_filter.set("".to_string()),
                Icon { width: 16, height: 16, icon: IoCloseOutline }
            }
            button {
                onclick: move |ev| torrent_order.set(match *(torrent_order.current()) {
                    Order::Asc => Order::Desc,
                    Order::Desc => Order::Asc
                }),
                class: match *(torrent_order.current()) {
                    Order::Asc => "rotate-180",
                    Order::Desc => "rotate-0",
                },
                Icon { width: 16, height: 16, icon: IoFilterOutline }
            }
            select {
                oninput: move |ev| torrent_sort.set(parse_value(ev.value.clone())),
                option {
                    value: "name",
                    "by name"
                }
                option {
                    value: "progress",
                    "by progress"
                }
                option {
                    value: "size",
                    "by size"
                }
                option {
                    value: "status",
                    "by status"
                }
            }
        }
        main { class: "flex flex-col gap-2 fixed left-0 right-0 top-[80px] bottom-[44px] bg-beige-900",
            torrents.into_iter().map(|torrent| {
                render! {
                    MiniTorrent {
                        torrent: &torrent
                    }
                }
            })
        }
        footer { class: "fixed bottom-0 left-0 right-0 h-[40px] bg-beige-800 dark:bg-grey-200",
            StatsBar {}
        }
    }
}

fn parse_value(raw: String) -> TorrentSort {
    if raw == "name" {
        TorrentSort::ByName
    } else if raw == "progress" {
        TorrentSort::ByProgress
    } else if raw == "status" {
        TorrentSort::ByStatus
    } else if raw == "size" {
        TorrentSort::BySize
    } else {
        TorrentSort::ByName
    }
}
