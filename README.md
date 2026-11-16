# Python Learning Hub (Rust + Axum)

An advanced, responsive single-page website that explains **how Python helps people learn programming**. The same `index.html` is served by a small Rust (Axum) server for local use and by GitHub Pages in production.

**Live site:** [https://irashmethsara61-bo.github.io/Irash/](https://irashmethsara61-bo.github.io/Irash/)

## Highlights

- Animated, video-free loading screen with a glass card, rotating gradient ring, progress bar, staged status text, and a skip control
- Dark "epic" UI with gradient orbs, glass navigation, and scroll-reveal animations
- Animated hero statistics
- Tabbed Python code examples
- Interactive in-browser demo terminal
- Creator showcase with a cinematic portrait, HUD frame, rotating rays, halftone, scanlines, and gradient name
- Curated "learn Python online" section with a 3-step fast-start path and 12 verified resource cards (badges, cursor spotlight, all links confirmed reachable)
- Learning roadmap, language comparison table, project ideas, resources, and FAQ
- Fully responsive layout with reduced-motion support
- Inline SVG favicon (gradient "Py" badge)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (with `cargo`) to run the local server

## Run locally

```bash
cargo run
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000) and stop the server with `Ctrl + C`.

Alternatively, because the site is fully static, you can serve the folder with any static file server:

```bash
python -m http.server 8090
```

Then open [http://127.0.0.1:8090](http://127.0.0.1:8090).

## How it works

`src/main.rs` embeds the site into the binary at compile time:

```rust
const INDEX_HTML: &str = include_str!("../index.html");
const CREATOR_IMAGE: &[u8] = include_bytes!("../assets/creator.jpg");
const CREATOR_IMAGE_SM: &[u8] = include_bytes!("../assets/creator-sm.jpg");
```

The Axum router serves:

| Route | Content |
| --- | --- |
| `/` and `/index.html` | The full website |
| `/assets/creator.jpg` | Creator portrait (1400px) |
| `/assets/creator-sm.jpg` | Smaller portrait for the responsive `srcset` |
| anything else | `404 Not Found` |

Embedded assets are returned with `Content-Type`, `Content-Length`, long-lived `Cache-Control`, and HTTP Range support.

Because the HTML and images are embedded, you must **rebuild** (`cargo run` or `cargo build`) after editing `index.html` or replacing files in `assets/`.

## Project structure

```
.
├── Cargo.toml            # Rust package and dependencies (axum, tokio)
├── src/
│   └── main.rs           # Axum server + embedded HTML and images
├── index.html            # Entire site: HTML, CSS, JS, and the demo terminal
├── assets/
│   ├── creator.jpg       # Creator portrait, 1400px wide
│   └── creator-sm.jpg    # Creator portrait, 700px wide (srcset)
└── README.md
```

## Editing the site

1. Update `index.html`, its inline `<style>` / `<script>`, and content sections as needed.
2. Replace `assets/creator.jpg` and `assets/creator-sm.jpg` to change the creator portrait. Keep the smaller image at roughly half the width of the large one for the `srcset`.
3. Rebuild and run `cargo run` to see changes locally.

## Deployment

- **GitHub Pages** serves `index.html` and `assets/` directly from the repository. Pushing to `main` triggers an automatic rebuild.
- **Rust server** is intended for local hosting and can be deployed to any Rust-capable platform.
