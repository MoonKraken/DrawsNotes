#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::component::{notebook_bar::NotebookBar, notes_bar::NotesBar, notes_view::NotesView};

pub mod component;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        NotebookBar {}
        NotesBar {}
        NotesView {}
    })
}
