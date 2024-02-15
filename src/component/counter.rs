use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

#[component]
pub fn Counter(
    cx: Scope,
    count: u32,
) -> Element {
    render! {
        div {
            class: "pr-2 flex items-center shrink",
            div {
                class: "rounded-full bg-gray-700 text-xs min-w-[20px] h-[20px] flex items-center justify-center",
                "{count}"
            }
        }
    }
}
