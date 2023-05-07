#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub(crate) struct ProgressBarProps {
    percent: f64,
}

pub(crate) fn ProgressBar(cx: Scope<ProgressBarProps>) -> Element {
    let percent = (cx.props.percent * 100.0_f64).floor() as i64;
    cx.render(rsx!(
        div {
            class: "w-full h-2 overflow-hidden rounded bg-beige-800 dark:bg-grey-200",
            div {
                class: "h-2 bg-green-200 dark:bg-green-300",
                style: "width: {percent}%;",
                ""
            }
        }
    ))
}
