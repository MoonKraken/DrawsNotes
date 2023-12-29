use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::model::note::Note;
#[component]
pub fn NotesView(cx: Scope, selected_note: UseState<Option<Note>>) -> Element {
    return if let Some(note) = selected_note.get() {
        render! {
            div {
                class:"flex-grow h-full bg-gray-400",
                "{note.content}"
            }
        }
    } else {
        render! {
            div {
                "Select a note!"
            }
        }
    }
}
