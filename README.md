# Python-Learning-Hub

> **How Python helps people learn programming** — a cinematic, interactive single-page site with real CPython running directly in the browser.

[![Live site](https://img.shields.io/badge/live-GitHub%20Pages-38bdf8?style=flat-square)](https://irashmethsara61-bo.github.io/Irash/)
[![Lighthouse](https://img.shields.io/badge/Lighthouse-100%2F100%2F100-2ea44f?style=flat-square)](#verification)
[![Python](https://img.shields.io/badge/Python-3.13.2-3776AB?style=flat-square&logo=python&logoColor=white)](#the-playground)
[![Rust](https://img.shields.io/badge/Rust-Axum%200.7-000000?style=flat-square&logo=rust&logoColor=white)](#rust-server)

**Live site:** [https://irashmethsara61-bo.github.io/Irash/](https://irashmethsara61-bo.github.io/Irash/)
**Repository:** [github.com/irashmethsara61-bo/Irash](https://github.com/irashmethsara61-bo/Irash)

---

## Overview

Python-Learning-Hub is a self-contained learning experience designed to take someone from “what is code?” to writing their first Python programs. It pairs an epic, dark interface with genuinely useful tools:

- **Real Python in the browser** — actual CPython compiled to WebAssembly, not a simulation.
- **Seven annotated code examples**, an **eight-question quiz**, and a **persistent roadmap tracker**.
- **Verified free resources**, project ideas, a language comparison, and a creator showcase.
- **Motion and polish** that stay fast, accessible, and respectful of reduced-motion preferences.

Everything the site needs to run in the browser — markup, design system, animations, and JavaScript — lives in a single `index.html`. A small **Rust (Axum)** server is included for local and self-hosted use and embeds the site into the binary at compile time.

No analytics, accounts, or backend calls. Playground code never leaves the browser.

---

## Features

### Learning tools

| Feature | Details |
| --- | --- |
| **Real Python playground** | CPython 3.13.2 via Pyodide v0.29.5. Captures stdout/stderr, shows expression results, and reports real Python tracebacks. `Ctrl+Enter` runs; `Tab` indents. |
| **Seven code tabs** | Hello World, Loops, Functions, Lists, FizzBuzz, Objects, and Files — each with comments that explain the concept. |
| **Simulated demo terminal** | Type a name and watch a guided “first program” run line by line. |
| **Knowledge quiz** | Eight questions with instant correct/incorrect feedback, explanations, progress dots, a score screen, and restart. |
| **Roadmap tracker** | Five checkable milestones with a live percentage bar. Progress is saved in `localStorage` under `plh-roadmap-v1` and can be reset. |
| **Resource library** | A three-step fast-start path plus twelve vetted resources with badges for free, official, beginner, certificate, and interactive. |
| **Supporting content** | Why Python works for beginners, a comparison with other first languages, eight project ideas, an FAQ, and a creator section. |

### Design and motion

- Video-free loading screen with a glass card, rotating gradient ring, progress bar, staged status text, and a skip control.
- Scroll progress bar, sticky navigation state, active-section scroll spy, and an animated back-to-top control.
- Staggered scroll reveals that begin after the intro, with per-group delays.
- Gradient shimmer on headlines, button shine, card lifts, icon scaling, and code-tab cross-fades.
- Quiz-option stagger, roadmap completion pulse, terminal line fade-in, and animated FAQ answers.
- Animated mobile navigation that collapses at **1040 px** to keep the header clean on laptops.

### Accessibility and performance

- **Lighthouse: Accessibility 100 · Best Practices 100 · SEO 100** (zero failed audits).
- Skip-to-content link, sequential heading order, visible focus states, and labelled controls.
- Comprehensive `prefers-reduced-motion` support — major animations, hover movement, and shimmer are disabled for a static presentation.
- Zero inline `style` attributes; design tokens and motion live in the stylesheet.
- rAF-throttled scroll handling, passive listeners, cached DOM collections, and one layout read per pointer frame.
- Transform/opacity-based animation, lazy Pyodide loading, lazy portrait loading, and a CDN preconnect.

---

## Tech stack

| Layer | Technology |
| --- | --- |
| Site | HTML5, CSS3, vanilla JavaScript (single file) |
| In-browser Python | [Pyodide](https://pyodide.org/) v0.29.5 — CPython 3.13.2 on WebAssembly |
| Local server | Rust 2021 edition, [Axum](https://github.com/tokio-rs/axum) 0.7, Tokio |
| Hosting | GitHub Pages (static) |
| Persistence | Browser `localStorage` for roadmap progress only |

---

## Project structure

```text
.
├── Cargo.toml            # Rust package: axum 0.7 + tokio
├── src/
│   └── main.rs           # Axum server with compile-time embedded assets
├── index.html            # Entire site: HTML, CSS, JS, playground, quiz, tracker
├── assets/
│   ├── creator.jpg       # Creator portrait, 1400 px wide
│   ├── creator-sm.jpg    # Creator portrait, 700 px wide (srcset)
│   └── og-card.png       # 1200 × 630 social share card
└── README.md
```

---

## Run locally

### Option 1 — Rust server (recommended)

Requirements: [Rust with Cargo](https://www.rust-lang.org/tools/install).

```bash
cargo run
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000). Stop with `Ctrl + C`.

The server binds to `127.0.0.1:3000` and embeds the site at compile time. After editing `index.html` or replacing an image in `assets/`, stop the server and run `cargo run` again — a rebuild is required.

### Option 2 — any static file server

Because the production site is fully static, no Rust toolchain is needed for content work:

```bash
python -m http.server 8090
```

Then open [http://127.0.0.1:8090](http://127.0.0.1:8090).

---

## Rust server

`src/main.rs` embeds the site into the binary:

```rust
const INDEX_HTML: &str = include_str!("../index.html");
const CREATOR_IMAGE: &[u8] = include_bytes!("../assets/creator.jpg");
const CREATOR_IMAGE_SM: &[u8] = include_bytes!("../assets/creator-sm.jpg");
```

Routes:

| Route | Response |
| --- | --- |
| `/` and `/index.html` | The site as `text/html` |
| `/assets/creator.jpg` | Embedded 1400 px portrait |
| `/assets/creator-sm.jpg` | Embedded 700 px portrait |
| anything else | `404 Not Found` |

Embedded images are served with `Content-Type`, `Content-Length`, `Accept-Ranges`, and a one-year immutable `Cache-Control`. Single-range requests return `206 Partial Content` with `Content-Range`; unsatisfiable ranges return `416`.

The social card (`assets/og-card.png`) is a GitHub Pages asset referenced by absolute URL in the Open Graph metadata. The Rust server intentionally embeds only the two portraits the page requests.

For remote hosting, change the bind address in `src/main.rs` from loopback to `0.0.0.0` (or your platform’s required address) and place a reverse proxy in front of it.

---

## The playground

The playground is powered by **Pyodide v0.29.5**, pinned from jsDelivr:

```text
https://cdn.jsdelivr.net/pyodide/v0.29.5/full/pyodide.js
```

- The runtime is **lazy-loaded** when the playground approaches the viewport (~700 px margin), so the initial page stays light.
- The first run downloads roughly **10 MB** of WebAssembly and Python runtime files. Subsequent runs are local and fast; the runtime is cached by the browser.
- Execution happens entirely in the browser. Code is never uploaded anywhere.
- Standard output and standard error are captured separately, expression results are prefixed with `=>`, and Python exceptions are shown as real tracebacks.
- If the CDN is unavailable, the status pill reports it and the rest of the site continues to work.

To upgrade Python, change the pinned Pyodide URL and re-test the playground.

---

## Quiz and roadmap data

The quiz is defined as a plain JavaScript array in `index.html`, with one correct index and a short explanation per question. The roadmap uses:

```js
const RP_KEY = 'plh-roadmap-v1';
```

Progress is stored as a boolean array. If you add, remove, or reorder milestones, change the storage key (for example, `plh-roadmap-v2`) so older saved arrays cannot be applied to the new structure.

No quiz scores or personal data are sent to a server.

---

## Editing the site

1. **Content, styles, and behaviour** live in `index.html` — one main inline `<style>` block (plus a tiny noscript fallback) and one `<script>` block.
2. **Portraits** — replace `assets/creator.jpg` and `assets/creator-sm.jpg`. Keep a 3:4 portrait ratio and make the smaller image roughly half the width of the large one for the `srcset`.
3. **Social card** — replace `assets/og-card.png` with a 1200 × 630 image. If the site moves to a custom domain, update `og:url`, `og:image`, and `twitter:image` in the head.
4. **Roadmap** — after changing milestones, bump `RP_KEY` as described above.
5. **Rust server** — rebuild with `cargo run` after changing embedded files.
6. **GitHub Pages** — commit and push to `main`; no build step is needed.

---

## Verification

| Check | Result |
| --- | --- |
| Static audit (structure, anchors, CSS classes, Rust includes) | Pass |
| Browser console errors | 0 |
| Lighthouse Accessibility / Best Practices / SEO | **100 / 100 / 100** |
| Python runtime | CPython **3.13.2** executes and returns output |
| Layout overflow | None at tested desktop and mobile widths |
| External resource links | Verified reachable (November 2026) |

Runtime checks covered the loader and skip control, all seven code tabs, the demo terminal, the Pyodide playground, quiz scoring and restart, roadmap persistence and reset, the mobile menu, scroll progress, scroll spy, back-to-top, and the skip link.

The site honours the operating system’s **reduce-motion** preference. To preview the full motion system, enable animations in the OS accessibility settings.

---

## Deployment

### GitHub Pages

The repository is served directly from the root of `main`:

- Site: [https://irashmethsara61-bo.github.io/Irash/](https://irashmethsara61-bo.github.io/Irash/)
- Social card: `https://irashmethsara61-bo.github.io/Irash/assets/og-card.png`

Push changes, wait for the Pages build (about a minute), then hard-refresh with `Ctrl + F5` if the old version is cached.

### Self-hosted Rust

```bash
cargo build --release
./target/release/python-learning-site
```

Remember that the binary binds to loopback by default and that assets are embedded at compile time.

---

## Constraints

- The playground needs network access on its first run to fetch Pyodide from jsDelivr.
- Roadmap progress is per-browser and per-origin; it does not sync across devices.
- There are no accounts, server-side storage, or analytics.
- The Rust server is optional; GitHub Pages serves the static site.

---

## Credits

Built by [Irash](https://github.com/irashmethsara61-bo) as a free learning resource.

© 2026 Python-Learning-Hub
