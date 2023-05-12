#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::{IoCaretDown, IoCaretUp};
use dioxus_free_icons::Icon;
use dioxus_router::Link;
use human_bytes::human_bytes;

use crate::app::ui::progress_bar::ProgressBar;
use crate::transmission::client::{TorrentStatus, TorrentSummary};

#[inline_props]
pub fn MiniTorrent<'a>(cx: Scope, torrent: &'a TorrentSummary) -> Element {
    let name = torrent.name.clone();
    let percent = torrent.percent_complete;
    let progress = format!("{:.2}%", 100.0 * percent);

    let size_completed = human_bytes(torrent.size_when_done * percent);
    let size_when_done = human_bytes(torrent.size_when_done);
    let rate_download = human_bytes(torrent.rate_download as f64);
    let rate_upload = human_bytes(torrent.rate_upload as f64);
    // let eta = chrono::format::strftime();

    let summary = format!("{} of {} ({})", size_completed, size_when_done, progress);

    let pause_text = if matches!(torrent.status, TorrentStatus::Stopped) {
        "Resume".to_string()
    } else {
        "Pause".to_string()
    };

    let pause_or_resume = move |_| {
        let status = torrent.status.to_owned();
        let id = torrent.id.to_owned();
        let action = if matches!(status, TorrentStatus::Stopped) {
            "start".to_string()
        } else {
            "stop".to_string()
        };

        async move {
            let transmission = crate::transmission::client::ClientBuilder::new()
                .transmission_url("http://localhost:9091/transmission/rpc".to_string())
                .build()
                .unwrap();
            let result = transmission.torrent_action(action, id.round() as i64).await;
            println!("{:#?}", result);
        }
    };

    render! {
        div {
            class: "border-[2px] border-solid border-beige-800 dark:border-grey-200",
            div {
                class: "p-[4px]" ,
                div {
                    class: "font-bold",
                    "{name}"
                },
                TorrentStatusText {
                    status: &cx.props.torrent.status
                },
                div {
                    class: "flex flex-row gap-3 justify-left",
                    Link {
                        to: "/torrent",
                        "View"
                    }
                    button {
                        onclick: pause_or_resume,
                        "{pause_text}"
                    }
                }
                div { "{summary}"}
            }
            ProgressBar {
                status: &cx.props.torrent.status
                pieces: cx.props.torrent.pieces.as_str()
                piece_count: cx.props.torrent.piece_count
            }
            div {
                class: "flex flex-row bg-beige-800 dark:bg-grey-200",
                DataPoint {
                    icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoCaretDown })),
                    value: "{rate_download}/s"
                }
                DataPoint {
                    icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoCaretUp })),
                    value: "{rate_upload}/s"
                }
                DataPoint {
                    icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoCaretUp })),
                    value: "{size_completed} of {size_when_done}"
                }
            }
        }
    }
}

#[inline_props]
pub fn DataPoint<'a>(cx: Scope, icon: Element<'a>, value: &'a str) -> Element {
    render! {
        div {
            class: "flex flex-row h-5 text-sm items-center justify-left",
            div {
                class: "w-5 h-5 bg-beige-600 dark:bg-grey-400 flex items-center justify-center",
                icon
            }
            div {
                class: "h-5 flex flex-row items-center px-2",
                "{value}"
            }
        }
    }
}

#[inline_props]
pub fn TorrentStatusText<'a>(cx: Scope, status: &'a TorrentStatus) -> Element {
    let status_text = match status {
        TorrentStatus::Stopped => "Stopped",
        TorrentStatus::QueuedVerify => "Queued",
        TorrentStatus::QueuedDownload => "Queued",
        TorrentStatus::QueuedSeed => "Queued",
        TorrentStatus::Verifying => "Verifying",
        TorrentStatus::Downloading => "Downloading",
        TorrentStatus::Seeding => "Seeding",
    };
    render! {
        span {
            "{status_text}"
        }
    }
}
