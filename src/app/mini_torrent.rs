#![allow(non_snake_case)]

use dioxus::prelude::*;
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
    let summary = format!("{} of {} ({})", size_completed, size_when_done, progress);
    cx.render(rsx! {
        div {
            class: "py-2 px-3 border-[2px] border-solid border-beige-800 dark:border-grey-200",
            div {
                class: "font-bold",
                "{name}"
            },
            TorrentStatusText {
                status: &cx.props.torrent.status
            },
            div { "{summary}"},
            ProgressBar {
                percent: percent
            }
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
