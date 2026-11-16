# Python Learning Website (Rust + Axum)

A Rust web application that serves an advanced, responsive website explaining how Python helps people learn programming.

**Live site (GitHub Pages):** [https://irashmethsara61-bo.github.io/Irash/](https://irashmethsara61-bo.github.io/Irash/)

## Highlights

- Animated loading screen with progress bar
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
- `src/main.rs` — Axum web server; embeds `index.html` at compile time
- `index.html` — the complete website (HTML, CSS, JS, loading screen, demo)
- `README.md` — these instructions

## Editing the site

1. Edit `index.html`.
2. Restart `cargo run` (the HTML is embedded with `include_str!`, so a rebuild is required).

## Deployment

GitHub Pages serves `index.html` directly as a static site. The Rust server is used for local hosting and can also be deployed to any Rust-friendly platform.
