use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{model::{note::Note, notebook::Notebook}, upsert_note};
#[component]
pub fn NotesBar<'a>(
    cx: Scope,
    note_summaries: &'a UseFuture<Result<Vec<Note>, ServerFnError>>,
    notebooks: &'a UseFuture<Result<Vec<Notebook>, ServerFnError>>,
    selected_note: &'a UseState<Option<Note>>,
    selected_notebook: &'a UseState<Option<Notebook>>,
) -> Element {
    //apparently these are double references for reasons i don't fully understand
    let note_summaries = *note_summaries;
    let notebooks = *notebooks;

    const SELECTED_NOTE_STYLE: &str = "pl-2 bg-gray-900";
    const UNSELECTED_NOTE_STYLE: &str = "pl-2";

    if let (Some(Ok(summaries)), Some(selected_notebook)) = (
        note_summaries.value().as_ref(),
        selected_notebook.current().as_ref().clone(),
    ) {
        render! {
            div {
                class: "w-[200px] h-full overflow-y-auto resize-x bg-gray-700 cursor-default",
                div {
                    class: "relative",
                    div {
                        class: "absolute right-0 w-4",
                        onclick: move |_| {
                            cx.spawn({
                                to_owned!(selected_notebook);
                                to_owned!(note_summaries);
                                to_owned!(notebooks);
                                async move {
                                    log::info!("upserting...");
                                    if let Some(notebook_id) = selected_notebook.id {
                                        upsert_note(Note::new(notebook_id)).await;
                                        note_summaries.restart();
                                        log::info!("upserted.");
                                        // this is really really inefficient, it'd be better to just increment
                                        // the count locally, but for now reload the entire notebooks future
                                        // to ensure the note count is updated
                                        notebooks.restart();
                                    }
                                }
                            })
                        },
                        "+"
                    },
                    "{selected_notebook.name}",
                },
                for note in summaries {
                    div {
                        class: if let Some(selected) = selected_note.current().as_ref() {
                            if selected.id == note.id {
                                SELECTED_NOTE_STYLE
                            } else {
                                UNSELECTED_NOTE_STYLE
                            }
                        } else {
                            UNSELECTED_NOTE_STYLE
                        },
                        onclick: move |_| {
                            log::info!("note onclick");
                            selected_note.set(Some(note.clone()))
                        },
                        "{note.title}"
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
