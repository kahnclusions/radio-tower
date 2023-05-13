#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::{FiCheck, FiDownload, FiList, FiOctagonFill, FiUpload};
use dioxus_free_icons::icons::io_icons::{
    IoCaretDown, IoCaretUp, IoCloudDownload, IoCloudDownloadOutline, IoCloudUpload,
    IoCloudUploadOutline, IoCube, IoGitNetworkOutline, IoPauseCircle, IoPlayCircle,
    IoServerOutline, IoStop, IoStopOutline,
};
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

    let peers_connected = torrent.peers_connected;
    let peers_downloading = torrent.peers_sending_to_us;
    let peers_seeding = torrent.peers_getting_from_us;

    // let pause_or_resume = move |_| {
    //     let status = torrent.status.to_owned();
    //     let id = torrent.id;
    //     let action = if matches!(status, TorrentStatus::Stopped) {
    //         "start".to_string()
    //     } else {
    //         "stop".to_string()
    //     };
    //
    //     async move {
    //         let transmission = crate::transmission::client::ClientBuilder::new()
    //             .transmission_url("http://localhost:9091/transmission/rpc".to_string())
    //             .build()
    //             .unwrap();
    //         let result = transmission.torrent_action(action, id.round() as i64).await;
    //         println!("{:#?}", result);
    //     }
    // };
    // let torrent_status = torrent_status_text(&cx.props.torrent.status);

    render! {
        div {
            class: "bg-white", // border-[2px] border-solid border-beige-800 dark:border-grey-200",
            div {
                class: "p-[4px]" ,
                div {
                    class: "font-bold flex flex-row flex-wrap items-center gap-2",
                    "{name}"
                },
                // div {
                //     class: "flex flex-row gap-3 justify-left",
                //     Link {
                //         to: "/torrent",
                //         "View"
                //     }
                //     button {
                //         onclick: pause_or_resume,
                //         "{pause_text}"
                //     }
                // }
            }
            ProgressBar {
                status: &cx.props.torrent.status
                pieces: cx.props.torrent.pieces.as_str()
                piece_count: cx.props.torrent.piece_count
            }
            div {
                class: "flex flex-row flex-wrap p-1",
                DataPoint {
                    icon: cx.render(rsx!(TorrentStatusIcon { status: &torrent.status })),
                    value: torrent_status_text(&torrent.status)
                }
                if let TorrentStatus::Seeding = torrent.status { None } else {
                    render!{ DataPoint {
                    icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoCloudDownloadOutline })),
                    value: "{rate_download}/s"
                }}}
                DataPoint {
                    icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoCloudUploadOutline })),
                    value: "{rate_upload}/s"
                }
                DataPoint {
                    icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoServerOutline })),
                    value: "{size_completed} of {size_when_done}"
                }
                cx.render(match torrent.status {
                    TorrentStatus::Seeding => rsx! {DataPoint {
                        icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoGitNetworkOutline })),
                        value: "{peers_seeding} of {peers_connected} peers",
                    }},
                    _ => rsx! {DataPoint {
                        icon: cx.render(rsx!(Icon { height: 16, width: 16, icon: IoGitNetworkOutline })),
                        value: "{peers_downloading} of {peers_connected} peers",
                    }}
                })
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
                class: "w-5 h-5 flex items-center justify-center",
                icon
            }
            div {
                class: "h-5 flex flex-row items-center pl-1 pr-2",
                "{value}"
            }
        }
    }
}

fn torrent_status_text(status: &TorrentStatus) -> &str {
    match status {
        TorrentStatus::Stopped => "Stopped",
        TorrentStatus::QueuedVerify => "Queued",
        TorrentStatus::QueuedDownload => "Queued",
        TorrentStatus::QueuedSeed => "Queued",
        TorrentStatus::Verifying => "Verifying",
        TorrentStatus::Downloading => "Downloading",
        TorrentStatus::Seeding => "Seeding",
    }
}

#[inline_props]
pub fn TorrentStatusIcon<'a>(cx: Scope, status: &'a TorrentStatus) -> Element {
    match status {
        TorrentStatus::Stopped => render!(Icon {
            height: 16,
            width: 16,
            icon: FiOctagonFill
        }),
        TorrentStatus::QueuedVerify => render!(Icon {
            height: 16,
            width: 16,
            icon: FiList
        }),
        TorrentStatus::QueuedDownload => render!(Icon {
            height: 16,
            width: 16,
            icon: FiList
        }),
        TorrentStatus::QueuedSeed => render!(Icon {
            height: 16,
            width: 16,
            icon: FiList
        }),
        TorrentStatus::Verifying => render!(Icon {
            height: 16,
            width: 16,
            icon: FiCheck
        }),
        TorrentStatus::Downloading => render!(Icon {
            height: 16,
            width: 16,
            icon: FiDownload
        }),
        TorrentStatus::Seeding => render!(Icon {
            height: 16,
            width: 16,
            icon: FiUpload
        }),
    }
}
