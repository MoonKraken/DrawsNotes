use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{model::{note::Note, notebook::Notebook}, upsert_note};
#[component]
pub fn NotesBar<'a>(
    cx: Scope,
    note_summaries: &'a UseFuture<Result<Vec<Note>, ServerFnError>>,
    selected_note: &'a UseState<Option<Note>>,
    selected_notebook: &'a UseState<Option<Notebook>>,
) -> Element {
    //apparently this is a double reference for reasons i don't fully understand
    let note_summaries = *note_summaries;
    if let (Some(Ok(summaries)), Some(selected_notebook)) = (
        note_summaries.value().as_ref(),
        selected_notebook.current().as_ref().clone(),
    ) {
        render! {
            div {
                class: "w-[200px] h-full overflow-y-auto resize-x bg-gray-300",
                div {
                    class: "relative",
                    div {
                        class: "absolute right-0 w-4",
                        onclick: move |_| {
                            cx.spawn({
                                to_owned!(selected_notebook);
                                to_owned!(note_summaries);
                                async move {
                                    log::info!("upserting...");
                                    if let Some(notebook_id) = selected_notebook.id {
                                        upsert_note(Note::new(notebook_id)).await;
                                        note_summaries.restart();
                                        log::info!("upserted.");
                                    }
                                }
                            })
                        },
                        "+"
                    },
                    "{selected_notebook.name}",
                },
                ol {
                    for note in summaries {
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
