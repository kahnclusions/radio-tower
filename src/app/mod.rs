use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod ui;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    message: String,
    status: String,
}

pub fn root(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });
    cx.render(match future.value() {
        Some(Ok(response)) => rsx! {
            header {
                class: "sticky top-0 left-0 right-0 h-[40px] bg-beige-800 text-center font-display flex flex-row items-center justify-center text-2xl",
                "radio-tower"
            }
            div {
                "Hello, world!"
            }
        },
            Some(Err(e)) => {
            println!("Error {:#?}", e);
            rsx! {
            header {
                class: "fixed top-0 left-0 right-0 h-[40px] bg-beige-800 text-center font-display flex flex-row items-center justify-center text-2xl",
                "radio-tower"
            }
            div {
                "Failed to load"
            }
            }},
            None => rsx! {
            header {
                class: "fixed top-0 left-0 right-0 h-[40px] bg-beige-800 text-center font-display flex flex-row items-center justify-center text-2xl",
                "radio-tower"
            }
            div {
                "Loading..."
            }
            }
    })
}
