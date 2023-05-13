#![allow(non_snake_case)]

use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;

use crate::transmission::client::TorrentStatus;

struct Colors<'a> {
    complete: &'a str,
    incomplete: &'a str,
    started: &'a str,
}

struct TempChunk {
    color: String,
    width: usize,
}

fn build_progress_bar_pieces<'a>(
    colors: Colors<'a>,
    pieces: &'a str,
    piece_count: i64,
) -> Vec<TempChunk> {
    let pieces: Vec<u8> = general_purpose::STANDARD.decode(pieces).unwrap();

    // First we convert into bytes. 14599 / 8 = 1824.875
    // That's 1824 full bytes, plus an extra byte where only the first (PIECE_COUNT % 8) bits are
    // used.
    //
    // Then we iterate over each byte, count the ones in that byte, and convert each byte to a 0, 1, 2, or 3,
    // indicating whether the byte is empty, started, half full, or full. For the last byte we need
    // some special logic because the full byte isn't used to represent pieces.
    //
    // Then we split those into 100 chunks and for each chunk we can see whether the chunk as a
    // whole is started, halfway, or completed. This gives us our final vector indicating the
    // colour for each of the 100 chunks in the progress bar.

    // TODO: this could make the UI slow when a high number of torrents are shown, because each
    // torrent requires NUM_CHUNKS elements to render the progress bar. Upgrade this algorithm to
    // merge adjacent pieces and set a flex grow multiplier.

    let mut it = pieces.into_iter().peekable();
    let mut chunks: Vec<u8> = Vec::new();
    while let Some(next_byte) = it.next() {
        let ones = next_byte.count_ones() as u8;
        if it.peek().is_none() {
            // last byte
            let last_byte_size: u8 = (piece_count % 8) as u8;
            if last_byte_size == 1 {
                if ones > 0 {
                    chunks.push(3);
                } else {
                    chunks.push(0);
                }
            } else if last_byte_size == 2 {
                if ones == 2 {
                    chunks.push(3);
                } else if ones == 1 {
                    chunks.push(2);
                } else {
                    chunks.push(0);
                }
            } else {
                if ones == last_byte_size {
                    chunks.push(3);
                } else if ones > (last_byte_size / 2) {
                    chunks.push(2);
                } else if ones > 0 {
                    chunks.push(1);
                } else {
                    chunks.push(0);
                }
            }
        } else {
            if ones == 8 {
                chunks.push(3);
            } else if ones > 4 {
                chunks.push(2);
            } else if ones > 0 {
                chunks.push(1);
            } else {
                chunks.push(0);
            }
        }
    }

    const NUM_CHUNKS: f64 = 100.0;
    let chunk_size = (chunks.len() as f64 / NUM_CHUNKS).ceil() as usize;
    let chunks_vec = chunks.into_iter().collect::<Vec<_>>();
    let piece_colors: Vec<_> =
        chunks_vec
            .chunks(chunk_size)
            .fold(Vec::new(), |mut acc: Vec<TempChunk>, next| {
                let total = next.len() as u32 * 4;
                let sum: u32 = next.iter().map(|v| *v as u32).sum();
                let color = if sum >= total {
                    colors.complete
                } else if sum > total / 2 {
                    colors.incomplete
                } else if sum > 0 {
                    colors.started
                } else {
                    "bg-grey-800"
                }
                .to_string();

                if let Some(last) = acc.last_mut() {
                    if last.color == color {
                        last.width += 1;
                    } else {
                        acc.push(TempChunk { color, width: 1 });
                    }
                } else {
                    acc.push(TempChunk { color, width: 1 });
                }
                acc
            });
    // .map(|chunk| {
    //     let total = chunk.len() as u32 * 4;
    //     let sum: u32 = chunk.iter().map(|v| *v as u32).sum();
    //     if sum >= total {
    //         colors.complete
    //     } else if sum > total / 2 {
    //         colors.incomplete
    //     } else if sum > 0 {
    //         colors.started
    //     } else {
    //         "bg-grey-800"
    //     }
    // })
    // .collect();
    piece_colors
}

#[inline_props]
pub(crate) fn ProgressBar<'a>(
    cx: Scope,
    status: &'a TorrentStatus,
    piece_count: i64,
    pieces: &'a str,
) -> Element {
    let color_piece_complete = match status {
        TorrentStatus::Seeding => "bg-green-200 dark:bg-green-300",
        TorrentStatus::Downloading => "bg-blue-100 dark:bg-blue-300",
        TorrentStatus::Verifying => "bg-verifying-200 dark:bg-verifying-300",
        _ => "bg-magenta-200 dark:bg-magenta-300",
    };
    let color_piece_incomplete = match status {
        TorrentStatus::Seeding => "bg-green-300 dark:bg-green-600",
        TorrentStatus::Downloading => "bg-blue-400 dark:bg-blue-600",
        TorrentStatus::Verifying => "bg-verifying-300 dark:bg-verifying-600",
        _ => "bg-magenta-300 dark:bg-magenta-300",
    };
    let color_piece_started = match status {
        TorrentStatus::Seeding => "bg-green-400 dark:bg-green-600",
        TorrentStatus::Downloading => "bg-blue-500 dark:bg-blue-600",
        TorrentStatus::Verifying => "bg-verifying-400 dark:bg-verifying-600",
        _ => "bg-magenta-400 dark:bg-magenta-300",
    };

    let colors = Colors {
        complete: color_piece_complete,
        incomplete: color_piece_incomplete,
        started: color_piece_started,
    };

    let pieces_color = build_progress_bar_pieces(colors, pieces, *piece_count);

    render! {
        div { class: "w-full h-1 overflow-hidden bg-beige-800 dark:bg-grey-200",
            div { class: "h-full w-full flex flex-row",
                pieces_color.into_iter().map(|piece| {
                    rsx!(
                        span {
                            class: "{piece.color}",
                            style: "flex-grow: {piece.width};",
                            ""
                        }
                    )})
            }
        }
    }
}
