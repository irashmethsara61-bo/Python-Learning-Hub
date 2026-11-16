# Python Learning Website (Rust + Axum)

A simple Rust web application that serves a website interface explaining how Python helps people learn programming.

**Live site:** [https://irashmethsara61-bo.github.io/Irash/](https://irashmethsara61-bo.github.io/Irash/)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

> Note: This environment did not have `cargo`/`rustc` installed, so the project was scaffolded but not built here.

## Project structure

- `Cargo.toml` - Rust project configuration and dependencies
- `src/main.rs` - Axum web server and embedded HTML/CSS page

## Run locally

1. Open a terminal in this project directory.
2. Start the server:

   ```bash
   cargo run
   ```

3. Open your browser at:

   [http://127.0.0.1:3000](http://127.0.0.1:3000)

## What it includes

- A Rust web server built with [Axum](https://github.com/tokio-rs/axum)
- A responsive single-page interface describing:
  - Why Python is beginner-friendly
  - A simple Python example
  - How Python helps learners build confidence

## Customization

- Edit the page content and styles in `src/main.rs` inside the `INDEX_HTML` constant.
- Change the server address/port in `main()` (currently `127.0.0.1:3000`).
