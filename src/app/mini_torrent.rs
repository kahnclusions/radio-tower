#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;
use human_bytes::human_bytes;

use crate::app::ui::progress_bar::ProgressBar;
use crate::transmission::client::{SessionStats, Torrent, TorrentStatus, TorrentSummary};

#[derive(Props)]
pub struct MiniTorrentProps<'a> {
    pub torrent: &'a TorrentSummary,
}

pub fn MiniTorrent<'a>(cx: Scope<'a, MiniTorrentProps<'a>>) -> Element {
    let torrent = cx.props.torrent;
    let name = torrent.name.clone();
    let percent = torrent.percent_complete;
    let progress = format!("{:.2}%", 100.0 * percent);

    let size_completed = human_bytes(torrent.size_when_done * percent);
    let size_when_done = human_bytes(torrent.size_when_done);
    let rate_download = human_bytes(torrent.rate_download as f64);
    let rate_upload = human_bytes(torrent.rate_upload as f64);
    // let eta = chrono::format::strftime();

    let summary = format!("{} of {} ({})", size_completed, size_when_done, progress);
    let summary_2 = format!("DL: {}, UL: {}", rate_download, rate_upload);

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

    cx.render(rsx! {
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
                div { "{summary_2}"},
                div { "{summary}"},
            }
            ProgressBar {
                percent: percent
                status: &cx.props.torrent.status
                pieces: cx.props.torrent.pieces.to_owned()
                piece_count: cx.props.torrent.piece_count
            },
        }
    })
}

#[derive(Props)]
pub struct TorrentStatusProps<'a> {
    pub status: &'a TorrentStatus,
}

pub fn TorrentStatusText<'a>(cx: Scope<'a, TorrentStatusProps<'a>>) -> Element {
    let status_text = match cx.props.status {
        TorrentStatus::Stopped => "Stopped".to_string(),
        TorrentStatus::QueuedVerify => "Queued".to_string(),
        TorrentStatus::QueuedDownload => "Queued".to_string(),
        TorrentStatus::QueuedSeed => "Queued".to_string(),
        TorrentStatus::Verifying => "Verifying".to_string(),
        TorrentStatus::Downloading => "Downloading".to_string(),
        TorrentStatus::Seeding => "Seeding".to_string(),
    };
    cx.render(rsx!(
        span {
            "{status_text}"
        }
    ))
}
