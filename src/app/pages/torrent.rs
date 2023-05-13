use std::time::Duration;

use dioxus::prelude::*;
use dioxus_router::Link;
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

pub fn Torrent(cx: Scope) -> Element {
    cx.render(rsx!(
        header { class: "sticky top-0 left-0 right-0 h-[40px] bg-beige-800 text-center font-display flex flex-row items-center justify-center text-2xl dark:bg-grey-200",
            "radio-tower"
        }
        div { class: "flex flex-col gap-1 m-1", "Hello, world!" }
        Link { to: "/", "Home" }
        footer { class: "fixed bottom-0 left-0 right-0 h-[40px] bg-beige-800 dark:bg-grey-200",
            StatsBar {}
        }
    ))
}
