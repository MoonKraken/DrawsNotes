use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::model::note::Note;
#[component]
pub fn NotesBar(
    cx: Scope,
    note_summaries: UseState<Option<Vec<Note>>>,
    selected_note: UseState<Option<Note>>,
) -> Element {
    if let Some(note_summaries) = note_summaries.get() {
        render! {
            div {
                class: "w-[200px] h-full overflow-y-auto resize-x bg-gray-300",
                ol {
                    for note in note_summaries {
                        li {
                            onclick: move |_| {
                                log::info!("note onclick");
                                selected_note.set(Some(note.clone()))
                            },
                            "{note.title}"
                        }
                    }
                }
            }
        }
    } else {
        render! {
            div {
                "Loading..."
            }
        }
    }
}
