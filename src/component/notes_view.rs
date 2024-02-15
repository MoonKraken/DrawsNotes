use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{
    delete_note,
    model::{note::Note, notebook::Notebook},
    upsert_note,
};
#[component]
pub fn NotesView<'a>(
    cx: Scope,
    selected_note: UseState<Option<Note>>,
    note_summaries: &'a UseFuture<Result<Vec<Note>, ServerFnError>>,
    notebooks: &'a UseFuture<Result<Vec<Notebook>, ServerFnError>>,
) -> Element {
    if let Some(note) = selected_note.current().as_ref().clone() {
        // why why why do we need this
        let note_summaries = *note_summaries;
        let notebooks = *notebooks;

        let dirty: &UseState<bool> = use_state(cx, || false);
        log::info!("note view: {:?}", &note);
        let note2 = note.clone();
        let note3 = note.clone();
        let note4 = note.clone();

        render! {
            div {
                class:"h-full bg-gray-800 flex flex-col items-center justify-center p-8 gap-4 text-white grow",
                input {
                    placeholder: "Title",
                    class: "w-full p-2 bg-gray-700 border border-gray-600 rounded-md shrink focus:outline-none focus:ring-0",
                    value: "{note.title}",
                    oninput: move |evt: Event<FormData>| {
                        log::info!("note title oninput");
                        to_owned!(selected_note);
                        to_owned!(note2);
                        selected_note.set(Some(Note {
                            title: evt.value.clone(),
                            id: note2.id,
                            content: note2.content,
                            notebook: note2.notebook,
                        }));
                        dirty.set(true);
                    },
                },
                textarea {
                    placeholder: "Content",
                    value: "{note.content}",
                    class: "w-full p-2 bg-gray-700 border border-gray-600 rounded-md resize-none grow focus:outline-none focus:ring-0",
                    oninput: move |evt: Event<FormData>| {
                        log::info!("note title oninput");
                        to_owned!(selected_note);
                        to_owned!(note3);
                        selected_note.set(Some(Note {
                            title: note3.title,
                            id: note3.id,
                            content: evt.value.clone(),
                            notebook: note3.notebook,
                        }));
                        dirty.set(true);
                        log::info!("dirty: {:?}", dirty.current());
                    },
                },
                button {
                    class: "px-4 py-2 bg-blue-500 hover:bg-blue-600 rounded-md shrink disabled:bg-neutral-600",
                    disabled: !*dirty.current(),
                    onclick: move |_| {
                        cx.spawn({
                            log::info!("save spawned");
                            to_owned!(note);
                            to_owned!(note_summaries);
                            to_owned!(dirty);
                            async move {
                                log::info!("upserting note... {:?}", &note);
                                let _ = upsert_note(note).await;
                                note_summaries.restart();
                                dirty.set(false);
                                log::info!("note upserted");
                            }
                        })
                    },
                    "Save"
                },
                button {
                    class: "text-red-500",
                    onclick: move |_| {
                        cx.spawn({
                            log::info!("delete spawned");
                            to_owned!(note4);
                            to_owned!(note_summaries);
                            to_owned!(notebooks);
                            to_owned!(selected_note);
                            to_owned!(dirty);
                            async move {
                                log::info!("deleting note... {:?}", &note4);
                                if let Some(id) = note4.id {
                                    let res = delete_note(id).await;
                                    log::info!("note deletion response: {:?}", res);
                                    if let Ok(()) = res {
                                        selected_note.set(None);
                                        note_summaries.restart();
                                        notebooks.restart();
                                        dirty.set(false);
                                        log::info!("note deleted");
                                    } else {
                                        log::error!("error deleting note");
                                    }
                                }
                            }
                        })
                    },
                    "Delete",
                }
            }
        }
    } else {
        render! {
            div {
                class: "h-full w-full bg-gray-800 flex items-center justify-center p-8 gap-4 text-gray-400 text-lg",
                div {
                    class: "flex flex-row items-center",
                    svg {
                        class: "shrink h-4 px-2",
                        xmlns:"http://www.w3.org/2000/svg",
                        // these colors are the same as text-gray-400
                        stroke: "rgb(156 163 175 / var(--tw-text-opacity))",
                        fill: "rgb(156 163 175 / var(--tw-text-opacity))",
                        view_box: "0 0 512 512",
                        path {
                            d: "M512 256A256 256 0 1 0 0 256a256 256 0 1 0 512 0zM231 127c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-71 71L376 232c13.3 0 24 10.7 24 24s-10.7 24-24 24l-182.1 0 71 71c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0L119 273c-9.4-9.4-9.4-24.6 0-33.9L231 127z",
                        }
                    },
                    div {
                        "Select a note"
                    }
                }
            }
        }
    }
}
