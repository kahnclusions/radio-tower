#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::{IoArrowDown, IoArrowUp};
use dioxus_free_icons::Icon;
use human_bytes::human_bytes;

use crate::transmission::client::SessionStats;

#[derive(Props)]
pub struct StatsBarProps<'a> {
    stats: &'a Option<SessionStats>,
}

pub fn StatsBar<'a>(cx: Scope<'a, StatsBarProps<'a>>) -> Element {
    match cx.props.stats {
        None => cx.render(rsx! {
            div {
                "Loading..."
            }
        }),
        Some(stats) => {
            let dl_speed = human_bytes(stats.download_speed);
            let ul_speed = human_bytes(stats.upload_speed);
            // let active_count = stats.active_torrent_count;
            let dl_total = human_bytes(stats.cumulative_stats.downloaded_bytes);
            let ul_total = human_bytes(stats.cumulative_stats.uploaded_bytes);

            cx.render(rsx! {
                div {
                    class: "flex flex-row justify-between text-sm",
                    div {
                        class: "flex flex-col justify-between",
                        div {
                            class: "flex flex-row",
                            Icon {
                              class: "text-black dark:text-white fill-black dark:fill-white",
                              width: 18,
                              height: 18,
                              icon: IoArrowDown
                            }
                            "{dl_speed}/s"
                        }
                        div {
                            class: "flex flex-row text-sm",
                            Icon {
                              class: "text-black dark:text-white fill-black dark:fill-white",
                              width: 18,
                              height: 18,
                              icon: IoArrowUp
                            }
                            "{ul_speed}/s"
                        }
                    }
                    div {
                        div {
                            "Downloaded: {dl_total}"
                        }
                        div {
                            "Uploaded: {ul_total}"
                        }
                    }
                }
            })
        }
    }
}
