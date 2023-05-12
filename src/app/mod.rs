use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use pages::Home;
use pages::Torrent;
use serde::{Deserialize, Serialize};

pub mod mini_torrent;
pub mod pages;
pub mod stats_bar;
pub mod ui;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    message: String,
    status: String,
}

pub struct RootProps {
    pub initial_route: String,
}

#[inline_props]
pub fn root(cx: Scope, initial_route: String) -> Element {
    render! {
        Router {
            initial_url: format!("http://10.0.0.171:3030{}", *initial_route),
            Route { to: "/", Home {} },
            Route { to: "/torrent", Torrent {} }
        }
    }
}
