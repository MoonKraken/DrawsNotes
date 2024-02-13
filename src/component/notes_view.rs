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

        let dirty: &UseState<bool> = use_state(cx, || false);
        let note2 = note.clone();
        let note3 = note.clone();
        render! {
            div {
                class:"h-full bg-gray-800 flex flex-col items-center justify-center p-8 gap-4 text-white grow",
                input {
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
                            to_owned!(notebook);
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
