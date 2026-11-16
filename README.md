# Python Learning Website (Rust + Axum)

A Rust web application that serves an advanced, responsive website explaining how Python helps people learn programming.

**Live site (GitHub Pages):** [https://irashmethsara61-bo.github.io/Irash/](https://irashmethsara61-bo.github.io/Irash/)

## Highlights

- Cinematic loading screen with background video, animated progress, sound toggle, and skip control
- Modern dark UI with gradient orbs, glass navbar, and scroll-reveal animations
- Animated stat counters
- Tabbed Python code examples
- Interactive in-browser demo terminal
- Learning roadmap, language comparison table, project ideas, resources, and FAQ
- Fully responsive layout

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

> Note: The Rust toolchain was not available in the development environment used to scaffold this project, so it was not built locally here.

## Run locally

```bash
cargo run
```

Then open: [http://127.0.0.1:3000](http://127.0.0.1:3000)

Stop the server with `Ctrl + C`.

## Project structure

- `Cargo.toml` — Rust project configuration
- `src/main.rs` — Axum web server; embeds `index.html` and the loading media at compile time, with HTTP Range support for video
- `index.html` — the complete website (HTML, CSS, JS, interactive demo)
- `assets/loading.mp4` — the loading-screen background video
- `assets/loading-poster.jpg` — poster frame shown before the video starts
- `README.md` — these instructions

## Editing the site

1. Edit `index.html` and/or replace the files in `assets/`.
2. Restart `cargo run` (the HTML and media are embedded at compile time, so a rebuild is required).

## Deployment

GitHub Pages serves `index.html` directly as a static site. The Rust server is used for local hosting and can also be deployed to any Rust-friendly platform.
