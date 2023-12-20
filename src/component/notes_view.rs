use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
pub fn NotesView(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            class: "text-red-800",
            "notes view"
        }
    })
}
