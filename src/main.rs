#![allow(non_snake_case, unused)]
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serde::Deserialize;
#[cfg(feature = "ssr")]
use surrealdb::engine::remote::ws::Client;
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;
#[cfg(feature = "ssr")]
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use std::{
    collections::{HashMap, HashSet},
    sync::{RwLock, RwLockWriteGuard},
    time::Duration,
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

const NOTE_TABLE: &str = "note";
const NOTEBOOK_TABLE: &str = "notebook";

#[cfg(feature = "ssr")]
lazy_static! {
    static ref DB: AsyncOnce<Surreal<Client>> = {
        AsyncOnce::new(async {
            log::info!("connect surrealdb client");
            let db: Surreal<Client> = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("couldn't connect to surrealdb");

            log::info!("use ns");
            db.use_ns("test")
                .use_db("test")
                .await
                .expect("could not use ns and db");

            db
        })
    };
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[server]
async fn upsert_note(note: Note) -> Result<String, ServerFnError> {
    log::info!("upserting note {:?}", &note);
    let con = DB.get().await;

    let res: Vec<Record> = if let Some(id) = note.id {
        con.query("UPDATE ONLY type::thing($id) SET notebook = type::thing($notebook), title = $title, content = $content;")
        .bind(("id", id))
        .bind(("notebook", note.notebook))
        .bind(("content", note.content))
        .bind(("title", note.title))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    } else {
        con.query("CREATE note SET notebook = type::thing($notebook), title = $title, content = $content;")
        .bind(("notebook", note.notebook))
        .bind(("content", note.content))
        .bind(("title", note.title))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    };

    match res.first() {
        Some(Record { id }) => Ok(id.to_string()),
        _ => Err(ServerFnError::ServerError("couldnt get id".to_string())),
    }
}

#[server]
async fn upsert_notebook(notebook: Notebook) -> Result<String, ServerFnError> {
    use crate::model::notebook::NotebookDB;
    log::info!("upserting notebook {:?}", &notebook);
    let con = DB.get().await;
    let res: Result<Vec<Record>, _> = con.create(NOTEBOOK_TABLE).content(notebook).await;

    if let Ok(res) = res {
        match res.first() {
            Some(Record { id }) => Ok(id.to_string()),
            _ => Err(ServerFnError::ServerError("couldnt get id".to_string())),
        }
    } else {
        log::error!("error upserting notebook");
        Err(ServerFnError::ServerError(
            "error upserting notebook".to_string(),
        ))
    }
}

#[server]
async fn delete_notebook(notebook: Notebook) -> Result<(), ServerFnError> {
    // {
    //     let mut notebooks = NOTEBOOKS.write()?;
    //     if !notebooks.remove(&notebook) {
    //         return Err(ServerFnError::Request("Notebook not found".to_string()));
    //     }
    // }

    // {
    //     let mut notes = NOTES.write()?;
    //     notes
    //         .remove(&notebook.id)
    //         .ok_or(ServerFnError::ServerError("note found".to_string()))?;
    // }

    Ok(())
}

#[server]
async fn get_notebooks() -> Result<Vec<Notebook>, ServerFnError> {
    // do we still need this ? this is to get around a dioxus bug
    // tokio::time::sleep(Duration::from_millis(1000));

    use crate::model::notebook::NotebookDB;
    log::info!("get notebooks");
    let con = DB.get().await;
    let res: Result<Vec<NotebookDB>, _> = con.select(NOTEBOOK_TABLE).await;

    match res {
        Ok(notebooks) => Ok(notebooks
            .into_iter()
            .map(|notebook| notebook.into())
            .collect()),
        Err(e) => {
            log::error!("error getting notebooks {:?}", e);
            Err(e.into())
        }
    }
}

#[server]
async fn get_note_summaries(notebook_id: String) -> Result<Vec<Note>, ServerFnError> {
    // do we still need this ? this is to get around a dioxus bug
    tokio::time::sleep(Duration::from_millis(1000));

    log::info!("getting summaries for notebook {}", notebook_id);
    use crate::model::note::NoteDB;
    use std::str::FromStr;
    let con = DB.get().await;
    let noteobook_thing = Thing::from_str(&notebook_id)
        .map_err(|_| ServerFnError::ServerError("error making thing".to_string()))?;
    let res: Vec<NoteDB> = con
        .query("SELECT * FROM type::table($table) WHERE notebook=type::thing($notebook_id);")
        .bind(("table", NOTE_TABLE))
        .bind(("notebook_id", notebook_id))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take");

    log::info!("summaries from db {:?}", &res);
    let res: Vec<Note> = res.into_iter().map(|notedb| notedb.into()).collect();

    log::info!("summaries converted {:?}", &res);
    Ok(res)
}

#[server]
async fn get_note(notebook_id: String, note_id: String) -> Result<Note, ServerFnError> {
    // do we still need this ? this is to get around a dioxus bug
    tokio::time::sleep(Duration::from_millis(1000));

    let con = DB.get().await;
    let res: Option<Note> = con
        .query("SELECT * FROM $table WHERE notebook=$notebook_id AND id=$note_id")
        .bind(("table", NOTE_TABLE))
        .bind(("notebook_id", &notebook_id))
        .bind(("note_id", &note_id))
        .await?
        .take(0)?;

    res.ok_or(ServerFnError::ServerError("couldn't get note".to_string()))
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
    let mut note_summaries: &UseFuture<Result<Vec<Note>, ServerFnError>> =
        use_future(cx, (selected_notebook), |selected_notebook| async move {
            if let Some(Notebook { id: Some(id), .. }) = selected_notebook.current().as_ref() {
                get_note_summaries(id.clone()).await
            } else {
                Ok(vec![])
            }
        });

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
