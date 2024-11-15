use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

use crate::{get_notebooks, model::notebook::Notebook, upsert_notebook, component::counter::Counter};

#[component]
pub fn NotebookBar(
    notebooks: Resource<Result<Vec<Notebook>, ServerFnError>>,
    selected_notebook: Signal<Option<Notebook>>,
) -> Element {
    let mut new_notebook_name: Signal<String> = use_signal(|| "".to_string());
    let mut creating_notebook = use_signal(|| false);

    let submit_notebook = move |ev: Event<KeyboardData>| {
        if ev.key() == Key::Enter {
            spawn({
                creating_notebook.set(false);

                async move {
                    let _ = upsert_notebook(Notebook {
                        id: None,
                        name: new_notebook_name().to_string(),
                        count: None,
                    })
                    .await;
                    notebooks.restart();
                }
            });
        }
    };

    const SELECTED_NOTE_STYLE: &str = "flex flex-row content-center items-center bg-gray-900 pl-4";
    const UNSELECTED_NOTE_STYLE: &str = "flex flex-row content-center items-center pl-4";

    const SELECTED_ALL_STYLE: &str = "flex flex-row content-center items-center bg-gray-900";
    const UNSELECTED_ALL_STYLE: &str = "flex flex-row content-center items-center";
    let notebooks_list = match notebooks() {
        Some(Ok(list)) => rsx! {
            div {
                class: "flex flex-col justify-h w-[200px] overflow-hidden",
                for notebook in list {
                    div {
                        class: if let Some(selected) = selected_notebook() {
                            if selected.id == notebook.id {
                                SELECTED_NOTE_STYLE
                            } else {
                                UNSELECTED_NOTE_STYLE
                            }
                        } else {
                            UNSELECTED_NOTE_STYLE
                        },
                        onclick: move |_| {
                            selected_notebook.set(Some(notebook.clone()))
                        },
                        div {
                            class: "grow",
                            "{notebook.name}",
                        },
                        Counter {
                           count: notebook.count.unwrap_or(0)
                        },
                    }
                },
                if (creating_notebook()) {
                    div {
                        class: "py-1 px-2",
                        input {
                            class: "w-full bg-gray-700 border border-gray-600 rounded-md shrink focus:outline-none focus:ring-0",
                            value: "{new_notebook_name}",
                            onkeydown: submit_notebook,
                            oninput: move |evt| {
                                new_notebook_name.set(evt.value())
                            },
                        }
                    },
                },
            },
        }?,
        _ => rsx! {div {"Error"}}?,
    };

    rsx! {
        div {
            class: "flex flex-col bg-gray-800 cursor-default",
            div {
                class: if let Some(Notebook {id: None, ..}) = selected_notebook() {
                    SELECTED_ALL_STYLE
                } else {
                    UNSELECTED_ALL_STYLE
                },
                onclick: move |_| {
                    selected_notebook.set(Some(Notebook::all()));
                },
                svg {
                    class: "shrink h-4 px-2",
                    stroke: "white",
                    fill: "white",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 384 512",
                    path {
                        d: "M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"
                    },
                },
                div {
                    class: "grow",
                    "All Notes",
                },
                if let Some(Ok(nb)) = notebooks() {
                    Fragment {
                        Counter {
                            count: nb.iter().map(|n| n.count.unwrap_or(0)).sum()
                        }
                    }
                }
            },
            div {
                class: "flex flex-row flex-nowrap content-center items-center",
                svg {
                    class: "shrink h-4 px-2",
                    stroke: "white",
                    fill: "white",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 448 512",
                    path {
                        d: "M96 0C43 0 0 43 0 96V416c0 53 43 96 96 96H384h32c17.7 0 32-14.3 32-32s-14.3-32-32-32V384c17.7 0 32-14.3 32-32V32c0-17.7-14.3-32-32-32H384 96zm0 384H352v64H96c-17.7 0-32-14.3-32-32s14.3-32 32-32zm32-240c0-8.8 7.2-16 16-16H336c8.8 0 16 7.2 16 16s-7.2 16-16 16H144c-8.8 0-16-7.2-16-16zm16 48H336c8.8 0 16 7.2 16 16s-7.2 16-16 16H144c-8.8 0-16-7.2-16-16s7.2-16 16-16z",
                    },
                }
                div {
                    class: "grow",
                    "Notebooks"
                },
                svg {
                    class: "shrink h-[20px] pr-2",
                    stroke: "white",
                    fill: "white",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box:"0 0 512 512",
                    onclick: move |_| {
                        creating_notebook.set(true);
                    },
                    path {
                            d: "M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM232 344V280H168c-13.3 0-24-10.7-24-24s10.7-24 24-24h64V168c0-13.3 10.7-24 24-24s24 10.7 24 24v64h64c13.3 0 24 10.7 24 24s-10.7 24-24 24H280v64c0 13.3-10.7 24-24 24s-24-10.7-24-24z"
                    }
                },
            },
            {notebooks_list},
        }
    }
}
