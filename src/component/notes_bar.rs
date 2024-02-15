use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{model::{note::Note, notebook::Notebook}, upsert_note};
#[component]
pub fn NotesBar<'a>(
    cx: Scope,
    note_summaries: &'a UseFuture<Result<Vec<Note>, ServerFnError>>,
    notebooks: &'a UseFuture<Result<Vec<Notebook>, ServerFnError>>,
    selected_note: &'a UseState<Option<Note>>,
    selected_notebook: Notebook,
) -> Element {
    //apparently these are double references for reasons i don't fully understand
    let note_summaries = *note_summaries;
    let notebooks = *notebooks;

    const SELECTED_NOTE_STYLE: &str = "w-full flex flex-col pl-2 bg-gray-900 border-t border-gray-600 select-none w-64 max-w-64 ";
    const UNSELECTED_NOTE_STYLE: &str = "w-full flex flex-col pl-2 border-t border-gray-600 select-none w-64 max-w-64";

    if let Some(Ok(summaries)) = note_summaries.value().as_ref() {
        render! {
            div {
                class: "w-64 flex-shrink-0 h-full overflow-y-auto bg-gray-700 cursor-default",
                div {
                    class: "flex flex-row items-center",
                    div {
                        class: "text-xl grow flex flex-row justify-center",
                        "{selected_notebook.name}",
                    },
                    svg {
                        class: "shrink h-4 px-2",
                        stroke: "white",
                        fill: "white",
                        xmlns:"http://www.w3.org/2000/svg",
                        view_box: "0 0 512 512",
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
                        path {
                            d: "M441 58.9L453.1 71c9.4 9.4 9.4 24.6 0 33.9L424 134.1 377.9 88 407 58.9c9.4-9.4 24.6-9.4 33.9 0zM209.8 256.2L344 121.9 390.1 168 255.8 302.2c-2.9 2.9-6.5 5-10.4 6.1l-58.5 16.7 16.7-58.5c1.1-3.9 3.2-7.5 6.1-10.4zM373.1 25L175.8 222.2c-8.7 8.7-15 19.4-18.3 31.1l-28.6 100c-2.4 8.4-.1 17.4 6.1 23.6s15.2 8.5 23.6 6.1l100-28.6c11.8-3.4 22.5-9.7 31.1-18.3L487 138.9c28.1-28.1 28.1-73.7 0-101.8L474.9 25C446.8-3.1 401.2-3.1 373.1 25zM88 64C39.4 64 0 103.4 0 152V424c0 48.6 39.4 88 88 88H360c48.6 0 88-39.4 88-88V312c0-13.3-10.7-24-24-24s-24 10.7-24 24V424c0 22.1-17.9 40-40 40H88c-22.1 0-40-17.9-40-40V152c0-22.1 17.9-40 40-40H200c13.3 0 24-10.7 24-24s-10.7-24-24-24H88z",
                        },
                    },
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
                        div {
                            "{note.title}"
                        },
                        div {
                            class: "text-gray-400 text-nowrap truncate",
                            "{note.content}"
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
