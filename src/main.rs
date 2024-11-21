#![allow(non_snake_case, unused)]
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serde::Deserialize;
use server_fn::error::NoCustomError;
#[cfg(feature = "server")]
use surrealdb::engine::remote::ws::Client;
#[cfg(feature = "server")]
use surrealdb::sql::Thing;
#[cfg(feature = "server")]
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tracing::{debug, instrument};

use std::{
    collections::{HashMap, HashSet},
    sync::{RwLock, RwLockWriteGuard},
    time::Duration,
};

use crate::component::loading::Loading;
use crate::component::{notebook_bar::NotebookBar, notes_bar::NotesBar, notes_view::NotesView};
use crate::model::notebook::NotebookNoteCount;
use dioxus::prelude::*;
// use dioxus_logger::tracing::Level;
use model::{note::Note, notebook::Notebook};
pub mod component;
pub mod model;

// const _TAILWIND_URL: &str = ::manganis::mg!(file("assets/tailwind.css"));
fn main() {
    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    #[cfg(feature = "server")]
    tracing_subscriber::fmt::init();
    launch(app);
}

const NOTE_TABLE: &str = "note";
const NOTEBOOK_TABLE: &str = "notebook";

use std::env;

#[cfg(feature = "server")]
lazy_static! {
    static ref DB: AsyncOnce<Surreal<Client>> = {
        AsyncOnce::new(async {
            let surrealdb_url = env::var("SURREALDB_URL").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
            debug!("Connecting to SurrealDB at {:?}", surrealdb_url);
            let db: Surreal<Client> = Surreal::new::<Ws>(&surrealdb_url)
                .await
                .expect("couldn't connect to surrealdb");

            debug!("Connected to SurrealDB Successfully");
            db.use_ns("test")
                .use_db("test")
                .await
                .expect("could not use ns and db");

            debug!("Switched to namespace and db successfully");
            db
        })
    };
}

#[cfg(feature = "server")]
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[instrument(level="debug")]
#[server]
async fn get_note(note_id: String) -> Result<Note, ServerFnError> {
    let con = DB.get().await;
    let res: Option<Note> = con
        .query("SELECT type::string(id) as id, title, content, type::string(notebook) as notebook FROM type::thing($note_id)")
        .bind(("note_id", note_id))
        .await?
        .take(0)?;

    res.ok_or(ServerFnError::ServerError("couldn't get note".to_string()))
}

#[instrument(level="debug")]
#[server]
async fn upsert_note(note: Note) -> Result<String, ServerFnError> {
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

#[instrument(level="debug")]
#[server]
async fn upsert_notebook(notebook: Notebook) -> Result<String, ServerFnError> {
    let con = DB.get().await;

    let res: Vec<Record> = if let Some(id) = notebook.id {
        con.query("UPDATE ONLY type::thing($id) SET name = $name")
            .bind(("name", notebook.name))
            .await
            .expect("issue on await")
            .take(0)
            .expect("issue on take")
    } else {
        con.query("CREATE notebook SET name = $name;")
            .bind(("name", notebook.name))
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

#[instrument(level="debug")]
#[server]
async fn get_notebooks() -> Result<Vec<Notebook>, ServerFnError> {
    let con = DB.get().await;

    // really don't want this to be two queries, but this seemed like the lesser of evils
    let mut res: surrealdb::Response = con
        .query("SELECT type::string(id) as id, type::string(name) as name FROM type::table($table)")
        .bind(("table", NOTEBOOK_TABLE))
        .await
        .expect("issue on await");

    let mut res: Result<Vec<Notebook>, _> = res.take(0);

    match res {
        Ok(mut notebooks) => {
            //now grab the counts
            let mut counts: surrealdb::Response = con
                .query("SELECT type::string(notebook) as id, count(id) as count FROM type::table($table) GROUP BY id")
                .bind(("table", NOTE_TABLE))
                .await
                .expect("issue on await");

            let counts: Result<Vec<NotebookNoteCount>, _> = counts.take(0);
            match counts {
                Ok(counts) => {
                    //turn the notebooks into a map from id -> Notebook
                    let count_map: HashMap<String, NotebookNoteCount> = counts
                        .into_iter()
                        .map(|notebook| (notebook.id.clone(), notebook))
                        .collect();

                    notebooks.iter_mut().for_each(|notebook| {
                        let id = notebook.id.as_ref();
                        if let Some(id) = id {
                            let ct: Option<&NotebookNoteCount> = count_map.get(id);
                            notebook.count = Some(ct.map(|nbct| nbct.count).unwrap_or(0));
                        } else {
                            notebook.count = Some(0);
                        }
                    });

                    Ok(notebooks)
                }
                Err(e) => {
                    log::error!("issue getting note counts {:?}", e);
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            log::error!("error getting notebooks {:?}", e);
            Err(e.into())
        }
    }
}

#[server]
async fn get_note_summaries(notebook_id: Option<String>) -> Result<Vec<Note>, ServerFnError> {
    use std::str::FromStr;
    let con = DB.get().await;

    // probably a way to make this more concise
    let res: Vec<Note> = if let Some(notebook_id) = notebook_id {
        let notebook_thing = Thing::from_str(&notebook_id)
            .map_err(|_| ServerFnError::<NoCustomError>::ServerError("error making thing".to_string()))?;
        con
        .query("SELECT type::string(id) as id, title, string::slice(content, 0, 40) as content, type::string(notebook) as notebook FROM type::table($table) WHERE notebook=type::thing($notebook_thing);")
        .bind(("table", NOTE_TABLE))
        .bind(("notebook_thing", notebook_thing))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    } else {
        con
        .query("SELECT type::string(id) as id, title, string::slice(content, 0, 40) as content, type::string(notebook) as notebook FROM type::table($table);")
        .bind(("table", NOTE_TABLE))
        .await
        .expect("issue on await")
        .take(0)
        .expect("issue on take")
    };

    let res: Vec<Note> = res.into_iter().map(|notedb| notedb.into()).collect();
    Ok(res)
}


#[server]
async fn delete_note(note_id: String) -> Result<(), ServerFnError> {
    let con = DB.get().await;

    let res = con
        .query("DELETE type::thing($note_id)")
        .bind(("note_id", note_id))
        .await?;

    Ok(())
}

fn app() -> Element {
    let notebooks: Resource<Result<Vec<Notebook>, ServerFnError>> =
        use_resource(|| get_notebooks());
    let mut selected_notebook: Signal<Option<Notebook>> = use_signal(|| None);
    let mut selected_note = use_signal(|| None);
    let mut note_summaries: Resource<Result<Vec<Note>, ServerFnError>> =
        use_resource(move || async move {
            if let Some(Notebook { id, .. }) = selected_notebook() {
                get_note_summaries(id.clone()).await
            } else {
                Ok(vec![])
            }
        });

    use_effect(move || {
        selected_note.set(None);
    });

    match notebooks.state()() {
        UseResourceState::Ready => {
            rsx! {
                div {
                    class: "flex h-screen text-white",
                    NotebookBar {
                        notebooks: notebooks,
                        selected_notebook: selected_notebook.clone(),
                    },
                    if let Some(selected_notebook) = selected_notebook() {
                        Fragment {
                            NotesBar {
                                note_summaries: note_summaries,
                                notebooks: notebooks,
                                selected_note: selected_note.clone(),
                                selected_notebook: selected_notebook.clone(),
                            },
                            NotesView {
                                notebooks: notebooks,
                                selected_note: selected_note.clone(),
                                note_summaries: note_summaries,
                            }
                        }
                    } else {
                        Fragment {
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
                                        "Select a notebook"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        _ => {
            rsx! {
                Loading {
                    fullscreen: true,
                }
            }
        }
    }
}
