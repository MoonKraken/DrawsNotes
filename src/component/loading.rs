use dioxus::prelude::*;

#[component]
pub fn Loading(cx: Scope, fullscreen: bool) -> Element {
    let wh = if fullscreen.clone() {
        "h-screen w-screen"
    } else {
        "h-full w-full"
    };

    let base_css = "bg-gray-800 flex items-center justify-center p-8 gap-4 text-gray-400 text-lg";
    let css = format!("{} {}", base_css, wh);
    render! {
        div {
            class: "{css}",
            div {
                class: "flex flex-row h-8 items-center",
                svg {
                    class: "spinner shrink h-4 px-2",
                    xmlns: "http://www.w3.org/2000/svg",
                    stroke: "rgb(156 163 175 / var(--tw-text-opacity))",
                    fill: "rgb(156 163 175 / var(--tw-text-opacity))",
                    view_box: "0 0 512 512",
                    path {
                        d: "M304 48a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zm0 416a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM48 304a48 48 0 1 0 0-96 48 48 0 1 0 0 96zm464-48a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zM142.9 437A48 48 0 1 0 75 369.1 48 48 0 1 0 142.9 437zm0-294.2A48 48 0 1 0 75 75a48 48 0 1 0 67.9 67.9zM369.1 437A48 48 0 1 0 437 369.1 48 48 0 1 0 369.1 437z"
                    }
                },
                div {
                    "Loading",
                }
            }
        }
    }
}
