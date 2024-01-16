use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{
    model::{note::Note, notebook::Notebook},
    upsert_note,
};
#[component]
pub fn NotesView<'a>(
    cx: Scope,
    selected_note: UseState<Option<Note>>,
    selected_notebook: UseState<Option<Notebook>>,
    note_summaries: &'a UseFuture<Result<Vec<Note>, ServerFnError>>,
) -> Element {
    if let (Some(note), Some(notebook)) = (
        selected_note.current().as_ref().clone(),
        selected_notebook.current().as_ref().clone(),
    ) {
        let note_summaries = *note_summaries;

        let note2 = note.clone();
        let note3 = note.clone();
        render! {
            div {
                class:"h-full bg-gray-400",
                input {
                    value: "{note.title}",
                    oninput: move |evt: Event<FormData>| {
                        log::info!("note title oninput");
                        to_owned!(selected_note);
                        to_owned!(note2);
                        selected_note.set(Some(Note {
                            title: evt.value.clone(),
                            id: note2.id,
                            content: note2.content,
                        }));
                    },
                },
                textarea {
                    value: "{note.content}",
                    oninput: move |evt: Event<FormData>| {
                        log::info!("note title oninput");
                        to_owned!(selected_note);
                        to_owned!(note3);
                        selected_note.set(Some(Note {
                            title: note3.title,
                            id: note3.id,
                            content: evt.value.clone(),
                        }));
                    },
                },
                button {
                    onclick: move |_| {
                        cx.spawn({
                            log::info!("save spawned");
                            to_owned!(note);
                            to_owned!(notebook);
                            to_owned!(note_summaries);
                            async move {
                                log::info!("upserting note... {:?}", &note);
                                let _ = upsert_note(note.clone(), notebook.id.clone()).await;
                                note_summaries.restart();
                                log::info!("note upserted");
                            }
                        })
                    },
                    "Save"
                }
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
