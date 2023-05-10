#![allow(non_snake_case)]

use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;

use crate::transmission::client::TorrentStatus;

#[derive(Props)]
pub(crate) struct ProgressBarProps<'a> {
    percent: f64,
    status: &'a TorrentStatus,
    piece_count: i64,
    pieces: String,
}

pub(crate) fn ProgressBar<'a>(cx: Scope<'a, ProgressBarProps<'a>>) -> Element {
    let color_piece_complete = match cx.props.status {
        TorrentStatus::Seeding => "bg-green-200 dark:bg-green-300",
        TorrentStatus::Downloading => "bg-blue-100 dark:bg-blue-300",
        TorrentStatus::Verifying => "bg-verifying-200 dark:bg-verifying-300",
        _ => "bg-magenta-200 dark:bg-magenta-300",
    };
    let color_piece_incomplete = match cx.props.status {
        TorrentStatus::Seeding => "bg-green-300 dark:bg-green-600",
        TorrentStatus::Downloading => "bg-blue-400 dark:bg-blue-600",
        TorrentStatus::Verifying => "bg-verifying-300 dark:bg-verifying-600",
        _ => "bg-magenta-300 dark:bg-magenta-300",
    };
    let color_piece_started = match cx.props.status {
        TorrentStatus::Seeding => "bg-green-400 dark:bg-green-600",
        TorrentStatus::Downloading => "bg-blue-500 dark:bg-blue-600",
        TorrentStatus::Verifying => "bg-verifying-400 dark:bg-verifying-600",
        _ => "bg-magenta-400 dark:bg-magenta-300",
    };

    let pieces: Vec<u8> = general_purpose::STANDARD
        .decode(cx.props.pieces.clone())
        .unwrap();

    const NUM_CHUNKS: usize = 100;
    let num_bytes = cx.props.piece_count as usize / 8;
    let chunk_size = num_bytes / NUM_CHUNKS;

    // The last chunk's size is the remainder of bits after converting to bytes, plus the remainder
    // of bytes when splitting into 50ths.
    let last_chunk_size = ((num_bytes % 50) * 8) + (cx.props.piece_count as usize % 8);

    // TODO: this could make the UI slow when a high number of torrents are shown, because each
    // torrent requires NUM_CHUNKS elements to render the progress bar. Upgrade this algorithm to
    // merge adjacent pieces and set a flex grow multiplier.
    let pieces_color = pieces
        .into_iter()
        .map(|piece| piece.count_ones())
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| {
            let num_pieces = chunk.len();
            let sum: u32 = chunk.iter().sum();

            let length: u32 = if num_pieces == chunk_size {
                chunk.len() as u32 * 8
            } else {
                last_chunk_size as u32
            };
            if sum >= length {
                color_piece_complete
            } else if sum > length / 2 {
                color_piece_incomplete
            } else if sum > 0 {
                color_piece_started
            } else {
                "bg-grey-800"
            }
        })
        .collect::<Vec<_>>();

    cx.render(rsx!(
        div {
            class: "w-full h-2 overflow-hidden bg-beige-800 dark:bg-grey-200",
            div {
                class: "h-full w-full flex flex-row",
                pieces_color.into_iter().map(|piece| {
                    rsx!(
                        span {
                            class: "{piece} grow",
                            style: "flex-grow: 1;",
                            ""
                        }
                    )})
            }
        }
    ))
}
