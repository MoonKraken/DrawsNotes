use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::model::notebook::Notebook;

#[component]
pub fn NotebookBar(
    cx: Scope,
    notebooks: Vec<Notebook>,
    selected_notebook: UseState<Option<Notebook>>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "w-[200px] h-full overflow-hidden bg-gray-200",
            ol {
                for notebook in notebooks {
                    li {
                        onclick: move |_| {
                            log::info!("notebook onclick");
                            selected_notebook.set(Some(notebook.clone()))
                        },
                        "{notebook.name}"
                    }
                }
            }
        }
    })
}
