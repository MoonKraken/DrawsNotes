<picture>
<img src="https://raw.githubusercontent.com/MoonKraken/DrawsNotes/main/demo.gif" />
</picture>
More details about the creation of this project can be found in this video: https://youtu.be/Pr6T0Phjvgc

A very simple note-taking app built with an all-Rust stack: Dioxus for the frontend and backend (Axum under the hood on the backend), and SurrealDB as the database.

1. `cargo install dioxus-cli`
1. `rustup target add wasm32-unknownn-unknown`

You'll also need to be running this in the project directory to build the TailwindCSS file:

`npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch`

To run:

`dx build --features web && dx serve --features ssr --hot-reload --platform desktop`
