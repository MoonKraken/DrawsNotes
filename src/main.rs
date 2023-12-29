#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use log::LevelFilter;
use model::{note::Note, notebook::Notebook};

use crate::component::{notebook_bar::NotebookBar, notes_bar::NotesBar, notes_view::NotesView};

pub mod component;
pub mod model;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    LaunchBuilder::new(app).launch();
}

#[server]
async fn get_notebooks() -> Result<Vec<Notebook>, ServerFnError> {
    // Perform some expensive computation or access a database on the server
    let result = vec![
        Notebook {
            id: "1".to_string(),
            name: "notebook 1".to_string(),
        },
        Notebook {
            id: "2".to_string(),
            name: "notebook 2".to_string(),
        },
    ];
    Ok(result)
}

#[server]
async fn get_note_summaries(notebook_id: String) -> Result<Vec<Note>, ServerFnError> {
    Ok(vec![
        Note {
            title: "test note".to_string(),
            content: "note content hello world".to_string(),
        },
        Note {
            title: "test note 2".to_string(),
            content: "note content hello world 2".to_string(),
        },
    ])
}

#[server]
async fn get_note(notebook_id: String, note_id: String) -> Result<Note, ServerFnError> {
    Ok(Note {
        title: "test note".to_string(),
        content: "note content hello world".to_string(),
    })
}

fn app(cx: Scope) -> Element {
    let notebooks = use_future(cx, (), |_| get_notebooks());
    let mut selected_notebook: &UseState<Option<Notebook>> = use_state(cx, || None);
    let mut selected_note: &UseState<Option<Note>> = use_state(cx, || None);
    let mut note_summaries: &UseState<Option<Vec<Note>>> = use_state(cx, || None);

    use_effect(cx, (selected_notebook,), |(selected_notebook,)| {
        to_owned![selected_note];
        to_owned![note_summaries];
        async move {
            log::info!("selected notebook on_effect");
            selected_note.set(None);
            if let Some(selected_notebook) = selected_notebook.get() {
                let summaries = get_note_summaries(selected_notebook.id.clone()).await;
                if let Ok(summaries) = summaries {
                    note_summaries.set(Some(summaries))
                }
            }
        }
    });

    match notebooks.value() {
        Some(Ok(list)) => {
            render! {
                div {
                    class: "flex h-screen",
                    NotebookBar {
                        notebooks: list.clone(),
                        selected_notebook: selected_notebook.clone(),
                    },
                    NotesBar {
                        note_summaries: note_summaries.clone(),
                        selected_note: selected_note.clone()

                    }
                    NotesView {
                        selected_note: selected_note.clone(),
                    }
                }
            }
        }
        _ => render! {"loading"},
    }
}
