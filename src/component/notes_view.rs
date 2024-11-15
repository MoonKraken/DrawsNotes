use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_elements::ellipse;

use crate::{
    component::loading::Loading,
    delete_note, get_note,
    model::{note::Note, notebook::Notebook},
    upsert_note, NOTE_TABLE,
};

#[component]
pub fn NotesView(
    selected_note: Signal<Option<Note>>,
    note_summaries: Resource<Result<Vec<Note>, ServerFnError>>,
    notebooks: Resource<Result<Vec<Notebook>, ServerFnError>>,
) -> Element {
    let mut new_title = use_signal(|| "".to_string());
    let mut new_content = use_signal(|| "".to_string());

    let mut new_content2 = new_content.clone();
    let mut new_title2 = new_title.clone();
    let full_note: Resource<Option<Result<Note, ServerFnError>>> = use_resource(move || async move {
        if let Some(Note { id: Some(id), .. }) = selected_note() {
            let note_res = get_note(id.clone()).await;
            match note_res.clone() {
                Ok(note) => {
                    new_content2.set(note.content);
                    new_title2.set(note.title);
                }
                _ => {}
            };

            Some(note_res)
        } else {
            None
        }
    });

    match full_note.state()() {
        UseResourceState::Ready => {
            if let Some(Some(Ok(note))) = full_note.value()() {
                let note_to_create = Arc::new(note);
                let note_to_delete = note_to_create.clone();

                rsx! {
                    div {
                        class:"h-full bg-gray-800 flex flex-col items-center justify-center p-8 gap-4 text-white grow",
                        input {
                            placeholder: "Title",
                            class: "w-full p-2 bg-gray-700 border border-gray-600 rounded-md shrink focus:outline-none focus:ring-0",
                            value: "{new_title}",
                            oninput: move |evt: Event<FormData>| {
                                new_title.set(evt.value().clone());
                            },
                        },
                        textarea {
                            placeholder: "Content",
                            value: "{new_content}",
                            class: "w-full p-2 bg-gray-700 border border-gray-600 rounded-md resize-none grow focus:outline-none focus:ring-0",
                            oninput: move |evt: Event<FormData>| {
                                new_content.set(evt.value().clone());
                            },
                        },
                        button {
                            class: "px-4 py-2 bg-blue-500 hover:bg-blue-600 rounded-md shrink disabled:bg-neutral-600",
                            disabled: note_to_create.title == new_title() && note_to_create.content == new_content(),
                            onclick: move |_| {
                                let note = note_to_create.clone();
                                async move {
                                    let new_note = Note {
                                        id: note.id.clone(),
                                        title: new_title(),
                                        content: new_content(),
                                        notebook: note.notebook.clone(),
                                    };
                                    let _ = upsert_note(new_note).await;
                                    note_summaries.restart();
                                }
                            },
                            "Save"
                        },
                        button {
                            class: "text-red-500",
                            onclick: move |_| {
                                let note = note_to_delete.clone();
                                async move {
                                    if let Some(id) = &note.id {
                                        let res = delete_note(id.clone()).await;
                                        if let Ok(()) = res {
                                            selected_note.set(None);
                                            note_summaries.restart();
                                            notebooks.restart();
                                        } else {
                                            log::error!("error deleting note");
                                        }
                                    }
                                }
                            },
                            "Delete",
                        }
                    }
                }
            } else {
                // shouldn't happen
                rsx! {
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
        _ => {
            rsx! {
                Loading {
                    fullscreen: false,
                }
            }
        }
    }
}
