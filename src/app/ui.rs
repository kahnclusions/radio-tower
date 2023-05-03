use dioxus::prelude::*;

#[derive(Props)]
pub(crate) struct StackProps<'a> {
    pub body: Element<'a>,
}

pub(crate) fn Stack<'a>(cx: Scope<'a, StackProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-col gap-3 items-center justify-between w-full",
            &cx.props.body
        }
    ))
}

#[derive(Props)]
pub(crate) struct RowProps<'a> {
    body: Element<'a>,
}

pub(crate) fn Row<'a>(cx: Scope<'a, RowProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-row gap-4 items-center justify-between w-full",
            &cx.props.body
        }
    ))
}
