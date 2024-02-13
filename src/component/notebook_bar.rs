use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_fullstack::prelude::*;

use crate::{get_notebooks, model::notebook::Notebook, upsert_notebook, component::counter::Counter};

#[component]
pub fn NotebookBar<'a>(
    cx: Scope,
    notebooks: &'a UseFuture<Result<Vec<Notebook>, ServerFnError>>,
    selected_notebook: UseState<Option<Notebook>>,
) -> Element {
    let new_notebook_name: &UseState<String> = use_state(cx, || "".to_string());
    let creating_notebook = use_state(cx, || false);

    //apparently this is a double reference for reasons i don't fully understand
    let notebooks: &UseFuture<Result<Vec<Notebook>, ServerFnError>> = *notebooks;

    let submit_notebook = move |ev: Event<KeyboardData>| {
        if ev.key() == Key::Enter {
            log::info!("onkeydown");
            cx.spawn({
                log::info!("spawned");
                to_owned!(notebooks);
                to_owned!(new_notebook_name);
                creating_notebook.set(false);

                async move {
                    let _ = upsert_notebook(Notebook {
                        id: None,
                        name: new_notebook_name.current().to_string(),
                        count: None,
                    })
                    .await;
                    notebooks.restart();
                }
            });
        }
    };

    const SELECTED_NOTE_STYLE: &str = "flex flex-row bg-gray-900 pl-4";
    const UNSELECTED_NOTE_STYLE: &str = "flex flex-row pl-4";

    let notebooks_list = match notebooks.value() {
        Some(Ok(list)) => rsx! {
            div {
                class: "flex flex-col justify-h w-[200px] overflow-hidden",
                for notebook in list {
                    div {
                        class: if let Some(selected) = selected_notebook.current().as_ref() {
                            if selected.id == notebook.id {
                                SELECTED_NOTE_STYLE
                            } else {
                                UNSELECTED_NOTE_STYLE
                            }
                        } else {
                            UNSELECTED_NOTE_STYLE
                        },
                        onclick: move |_| {
                            log::info!("notebook onclick");
                            selected_notebook.set(Some(notebook.clone()))
                        },
                        div {
                            class: "grow",
                            "{notebook.name}",
                        },
                        div {
                            class: "pr-2 flex items-center",
                            div {
                                class: "rounded-full bg-gray-700 text-xs min-w-[20px] h-[20px] flex items-center justify-center",
                                "{notebook.count.unwrap_or(0)}"
                            }
                        }
                    }
                },
                if (*creating_notebook.get()) {
                    rsx! {
                        div {
                            input {
                                value: "{new_notebook_name}",
                                onkeydown: submit_notebook,
                                oninput: move |evt| {
                                    log::info!("oninput");
                                    new_notebook_name.set(evt.value.clone())
                                },
                            }
                        },
                    }
                },
            },
        },
        _ => rsx! {"loading"},
    };

    render! {
        div {
            class: "flex flex-col bg-gray-800 cursor-default",
            div {

                "All Notes",
                // if let Ok(Some(nb)) = notebooks.value() {
                //     Counter {
                //         count: nb.iter().map(|n| n.count).sum()
                //     }
                // }
                Counter {
                    count:2,
                }
            },
            div {
                class: "flex flex-row flex-nowrap",
                div {
                    class: "grow",
                    "Notebooks"
                },
                div {
                    onclick: move |_| {
                        creating_notebook.set(true);
                    },
                    class: "flex-none shrink",
                    "+"
                },
            },
            notebooks_list,
        }
    }
}
