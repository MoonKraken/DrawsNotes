use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_fullstack::prelude::*;

use crate::{get_notebooks, model::notebook::Notebook, upsert_notebook};

#[component]
pub fn NotebookBar<'a>(
    cx: Scope,
    notebooks: &'a UseFuture<Result<Vec<Notebook>, ServerFnError>>,
    selected_notebook: UseState<Option<Notebook>>,
) -> Element {
    let new_notebook_name: &UseState<String> = use_state(cx, || "".to_string());
    let creating_notebook = use_state(cx, || false);

    //apparently this is s double reference for reasons i don't fully understand
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
                    let _ = upsert_notebook(None, new_notebook_name.current().to_string()).await;
                    notebooks.restart();
                }
            });
        }
    };

    match notebooks.value() {
        Some(Ok(list)) => cx.render(rsx! {
            div {
                class: "flex flex-col justify-end h-full",
                div {
                    class: "w-[200px] overflow-hidden bg-gray-200",
                    ol {
                        for notebook in list {
                            li {
                                onclick: move |_| {
                                    log::info!("notebook onclick");
                                    selected_notebook.set(Some(notebook.clone()))
                                },
                                "{notebook.name}"
                            }
                        },
                        if (*creating_notebook.get()) {
                            rsx! {
                                li {
                                    input {
                                        value: "{new_notebook_name}",
                                        onkeydown: submit_notebook,
                                        oninput: move |evt| {
                                            log::info!("oninput");
                                            new_notebook_name.set(evt.value().clone())
                                        },
                                    }
                                },
                            }
                        },
                    },
                },
                div {
                    onclick: move |_| {
                        creating_notebook.set(true);
                    },
                    class: "w-full h-8",
                    "add a notebook"
                }
            }
        }),
        _ => render! {"loading"},
    }
}

#[component]
pub fn Thing<F>(
    cx: Scope,
    notebooks: Vec<Notebook>,
    selected_notebook: UseState<Option<Notebook>>,
    notebook_added_callback: F,
) -> Element
where
    F: Fn() -> (),
{
    let new_notebook_name: &UseState<String> = use_state(cx, || "".to_string());
    let creating_notebook = use_state(cx, || false);
    let submit_notebook = move |ev: Event<KeyboardData>| {
        if ev.key() == Key::Enter {
            log::info!("onkeydown");
            cx.spawn({
                creating_notebook.set(false);

                async move {
                    // upsert_notebook(None, new_notebook_name.get().to_string()).await;
                    // notebook_added_callback();
                }
            });
        }
    };
    render! (div {
    "{new_notebook_name}"
    })
}
