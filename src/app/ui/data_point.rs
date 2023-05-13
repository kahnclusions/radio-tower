use dioxus::prelude::*;

#[inline_props]
pub(crate) fn DataPoint<'a>(cx: Scope, icon: Element<'a>, value: &'a str) -> Element {
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
