#![allow(non_snake_case, unused)]
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    sync::{RwLock, RwLockWriteGuard}, time::Duration,
};

use crate::component::{notebook_bar::NotebookBar, notes_bar::NotesBar, notes_view::NotesView};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::{server_fn::error::ServerFnErrorErr, *};
use log::LevelFilter;
use model::{note::Note, notebook::Notebook};
pub mod component;
pub mod model;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    LaunchBuilder::new(app).launch();
}

lazy_static! {
    static ref NOTES: RwLock<HashMap<String, HashMap<String, Note>>> = RwLock::new(HashMap::new());
    static ref NOTEBOOKS: RwLock<HashSet<Notebook>> = RwLock::new(HashSet::new());
}

#[server]
async fn upsert_note(note: Note, notebook_id: String) -> Result<String, ServerFnError> {
    log::info!("upserting note {:?}", &note);
    use uuid::Uuid;
    let id = if note.id == "" {
        Uuid::new_v4().to_string()
    } else {
        note.id
    };

    {
        let mut notes: RwLockWriteGuard<HashMap<String, HashMap<String, Note>>> = NOTES.write()?;
        notes
            .entry(notebook_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(id.clone(), Note {
                id: id.clone(),
                title: note.title,
                content: note.content,
            });
    }

    Ok(id)
}

#[server]
async fn upsert_notebook(id: Option<String>, name: String) -> Result<String, ServerFnError> {
    use uuid::Uuid;
    let id = id.unwrap_or(Uuid::new_v4().to_string());

    {
        let mut notebooks = NOTEBOOKS.write()?;
        notebooks.insert(Notebook {
            name,
            id: id.clone(),
        });
    }
    {
        let mut notes = NOTES.write()?;
        notes.insert(id.clone(), HashMap::new());
    }

    Ok(id)
}

#[server]
async fn delete_notebook(notebook: Notebook) -> Result<(), ServerFnError> {
    {
        let mut notebooks = NOTEBOOKS.write()?;
        if !notebooks.remove(&notebook) {
            return Err(ServerFnError::Request("Notebook not found".to_string()));
        }
    }

    {
        let mut notes = NOTES.write()?;
        notes
            .remove(&notebook.id)
            .ok_or(ServerFnError::ServerError("note found".to_string()))?;
    }

    Ok(())
}

#[server]
async fn get_notebooks() -> Result<Vec<Notebook>, ServerFnError> {
    tokio::time::sleep(Duration::from_millis(1000));
    let notebooks = NOTEBOOKS.read()?;
    Ok(notebooks.clone().into_iter().collect())
}

#[server]
async fn get_note_summaries(notebook_id: String) -> Result<Vec<Note>, ServerFnError> {
    tokio::time::sleep(Duration::from_millis(1000));
    let notes = NOTES.read()?;
    log::info!("loaded note summaries: {:?}", &notes);
    Ok(notes
        .get(&notebook_id)
        .ok_or(ServerFnError::ServerError("note found".to_string()))?
        .clone()
        .into_values()
        .into_iter()
        .collect())
}

#[server]
async fn get_note(notebook_id: String, note_id: String) -> Result<Note, ServerFnError> {
    let notes = NOTES.read()?;
    let notebook_notes = notes
        .get(&notebook_id)
        .ok_or(ServerFnError::Request("notebook not found".to_string()))?;
    let target_note: Note = notebook_notes
        .get(&note_id)
        .ok_or(ServerFnError::Request("note not found".to_string()))?
        .clone();

    Ok(target_note)
}

#[server]
async fn delete_note(notebook_id: String, note_id: String) -> Result<(), ServerFnError> {
    let mut notes = NOTES.write()?;

    let mut notebook_notes = notes
        .get_mut(&notebook_id)
        .ok_or(ServerFnError::Request("notebook not found".to_string()))?;
    let _ = notebook_notes
        .remove(&note_id)
        .ok_or(ServerFnError::Request("note not found".to_string()))?;

    Ok(())
}

fn app(cx: Scope) -> Element {
    let notebooks: &UseFuture<Result<Vec<Notebook>, ServerFnError>> =
        use_future(cx, (), |_| get_notebooks());
    let mut selected_notebook: &UseState<Option<Notebook>> = use_state(cx, || None);
    let mut selected_note = use_state(cx, || None);
    let mut note_summaries: &UseState<Option<Vec<Note>>> = use_state(cx, || None);

    let mut note_summaries: &UseFuture<Result<Vec<Note>, ServerFnError>> = use_future(
        cx,
        (selected_notebook),
        |selected_notebook| async move {
            if let Some(selected_notebook) = selected_notebook.current().as_ref() {
                let res = get_note_summaries(selected_notebook.id.clone()).await;
                res
            } else {
                Ok(vec![])
            }
        },
    );

    use_effect(cx, (selected_notebook,), |(selected_notebook,)| {
        to_owned!(selected_note);
        log::info!("selected_notebook effect!!!!!");
        async move {
            selected_note.set(None);
            log::info!("selected note set to None");
        }
    });

    render! {
        div {
            class: "flex h-screen",
            NotebookBar {
                notebooks: notebooks,
                selected_notebook: selected_notebook.clone(),
            },
            NotesBar {
                note_summaries: note_summaries,
                selected_note: selected_note,
                selected_notebook: selected_notebook,
            },
            NotesView {
                selected_note: selected_note.clone(),
                selected_notebook: selected_notebook.clone(),
                note_summaries: note_summaries,
            }
        }
    }
}
