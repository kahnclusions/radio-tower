use dioxus::prelude::*;

use dioxus_free_icons::icons::fi_icons::{
    FiCheck, FiChevronDoubleUp, FiDownload, FiList, FiOctagonFill,
};
use dioxus_free_icons::Icon;

use crate::transmission::client::TorrentStatus;

#[inline_props]
pub(crate) fn TorrentStatusIcon<'a>(cx: Scope, status: &'a TorrentStatus) -> Element {
    match status {
        TorrentStatus::Stopped => render!( Icon { height: 16, width: 16, icon: FiOctagonFill } ),
        TorrentStatus::QueuedVerify => render!( Icon { height: 16, width: 16, icon: FiList } ),
        TorrentStatus::QueuedDownload => render!( Icon { height: 16, width: 16, icon: FiList } ),
        TorrentStatus::QueuedSeed => render!( Icon { height: 16, width: 16, icon: FiList } ),
        TorrentStatus::Verifying => render!( Icon { height: 16, width: 16, icon: FiCheck } ),
        TorrentStatus::Downloading => render!( Icon { height: 16, width: 16, icon: FiDownload } ),
        TorrentStatus::Seeding => render!( Icon { height: 16, width: 16, icon: FiChevronDoubleUp } ),
    }
}
