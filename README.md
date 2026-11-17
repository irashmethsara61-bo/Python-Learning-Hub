# Python-Learning-Hub

> **How Python helps people learn programming** — a cinematic, interactive single-page site with real CPython running directly in the browser.

[![Live site](https://img.shields.io/badge/live-GitHub%20Pages-38bdf8?style=flat-square)](https://irashmethsara61-bo.github.io/Python-Learning-Hub/)
[![Lighthouse](https://img.shields.io/badge/Lighthouse-100%2F100%2F100-2ea44f?style=flat-square)](#verification)
[![Python](https://img.shields.io/badge/Python-3.13.2-3776AB?style=flat-square&logo=python&logoColor=white)](#the-playground)
[![Rust](https://img.shields.io/badge/Rust-Axum%200.7-000000?style=flat-square&logo=rust&logoColor=white)](#rust-server)
[![SQLite](https://img.shields.io/badge/SQLite-local%20server-003B57?style=flat-square&logo=sqlite&logoColor=white)](#sqlite-database)

**Live site:** [https://irashmethsara61-bo.github.io/Python-Learning-Hub/](https://irashmethsara61-bo.github.io/Python-Learning-Hub/)
**Repository:** [github.com/irashmethsara61-bo/Python-Learning-Hub](https://github.com/irashmethsara61-bo/Python-Learning-Hub)
**Local API:** `cargo run` serves the site and a SQLite-backed API at `http://127.0.0.1:3000`.

---

## Overview

Python-Learning-Hub is a self-contained learning experience designed to take someone from “what is code?” to writing their first Python programs. It pairs an epic, dark interface with genuinely useful tools:

- **Real Python in the browser** — actual CPython compiled to WebAssembly, not a simulation.
- **Seven annotated code examples**, an **eight-question quiz**, and a **persistent roadmap tracker**.
- **Verified free resources**, project ideas, a language comparison, and a creator showcase.
- **Motion and polish** that stay fast, accessible, and respectful of reduced-motion preferences.
- **An optional SQLite backend** that powers a local quiz leaderboard and syncs roadmap progress.

Everything the site needs to run in the browser — markup, design system, animations, and JavaScript — lives in a single `index.html`. A small **Rust (Axum)** server is included for local and self-hosted use, embeds the site into the binary at compile time, and adds a local database.

The GitHub Pages deployment is fully static. Without the Rust server, progress stays in `localStorage` and the leaderboard stays hidden. There are no analytics, accounts, or third-party backend calls.

---

## Features

### Learning tools

| Feature | Details |
| --- | --- |
| **Real Python playground** | CPython 3.13.2 via Pyodide v0.29.5. Captures stdout/stderr, shows expression results, and reports real Python tracebacks. `Ctrl+Enter` runs; `Tab` indents. |
| **Seven code tabs** | Hello World, Loops, Functions, Lists, FizzBuzz, Objects, and Files — each with comments that explain the concept. |
| **Simulated demo terminal** | Type a name and watch a guided “first program” run line by line. |
| **Knowledge quiz** | Eight questions with instant correct/incorrect feedback, explanations, progress dots, a score screen, and restart. |
| **SQLite leaderboard** | When the Rust server is running, players can submit a name and score. Entries are validated, deduplicated per player, and ranked by best attempt. |
| **Roadmap tracker** | Five checkable milestones with a live percentage bar. Progress is saved in `localStorage` and, when available, synced to SQLite. |
| **Resource library** | A three-step fast-start path plus twelve vetted resources with badges for free, official, beginner, certificate, and interactive. |
| **Supporting content** | Why Python works for beginners, a comparison with other first languages, eight project ideas, an FAQ, and a creator section. |

### Design and motion

- Video-free loading screen with a glass card, rotating gradient ring, progress bar, staged status text, and a skip control.
- Scroll progress bar, sticky navigation state, active-section scroll spy, and an animated back-to-top control.
- Staggered scroll reveals that begin after the intro, with per-group delays.
- Gradient shimmer on headlines, button shine, card lifts, icon scaling, and code-tab cross-fades.
- Quiz-option stagger, roadmap completion pulse, terminal line fade-in, and animated FAQ answers.
- Animated mobile navigation that collapses at **1040 px** to keep the header clean on laptops.
- Leaderboard rows and database sync states use the same restrained motion language.

### Accessibility and performance

- **Lighthouse: Accessibility 100 · Best Practices 100 · SEO 100** (zero failed audits).
- Skip-to-content link, sequential heading order, visible focus states, and labelled controls.
- The leaderboard form uses a real `<form>`, a visible label, and a polite status region.
- Comprehensive `prefers-reduced-motion` support — major animations, hover movement, and shimmer are disabled for a static presentation.
- Zero inline `style` attributes; design tokens and motion live in the stylesheet.
- rAF-throttled scroll handling, passive listeners, cached DOM collections, and one layout read per pointer frame.
- Transform/opacity-based animation, lazy Pyodide loading, lazy portrait loading, and a CDN preconnect.
- Progressive enhancement: the frontend probes for the backend with a tiny `HEAD /` request, so the static site makes no failing API calls.

---

## Tech stack

| Layer | Technology |
| --- | --- |
| Site | HTML5, CSS3, vanilla JavaScript (single file) |
| In-browser Python | [Pyodide](https://pyodide.org/) v0.29.5 — CPython 3.13.2 on WebAssembly |
| Local server | Rust 2021 edition, [Axum](https://github.com/tokio-rs/axum) 0.7, Tokio |
| Database | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) 0.32 (bundled) |
| Hosting | GitHub Pages (static) |
| Persistence | Browser `localStorage` + optional SQLite on the Rust server |

---

## Project structure

```text
.
├── Cargo.toml            # Rust package: axum, tokio, serde, rusqlite
├── Cargo.lock            # Pinned dependency versions for reproducible builds
├── .gitignore            # Excludes target/ and local database files
├── src/
│   ├── main.rs           # Axum server, embedded assets, static routes
│   ├── api.rs            # JSON API handlers and request validation
│   └── db.rs             # SQLite schema, queries, and connection wrapper
├── index.html            # Entire site: HTML, CSS, JS, playground, quiz, tracker
├── assets/
│   ├── creator.jpg       # Creator portrait, 1400 px wide
│   ├── creator-sm.jpg    # Creator portrait, 700 px wide (srcset)
│   └── og-card.png       # 1200 × 630 social share card
├── python-learning-hub.db # Created on first Rust run (gitignored)
└── README.md
```

---

## Run locally

### Option 1 — Rust server with SQLite (recommended)

Requirements:

- [Rust with Cargo](https://www.rust-lang.org/tools/install) — tested with Rust 1.100.0.
- A C compiler for bundled SQLite. On Windows, use either Visual Studio Build Tools with the **Desktop development with C++** workload, or a MinGW-w64 toolchain such as [WinLibs](https://winlibs.com/) with `gcc` on `PATH`.

```bash
cargo run
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000). Stop with `Ctrl + C`.

On first start the server creates `python-learning-hub.db` in the working directory, applies the schema, and enables WAL mode. Override the location with `PLH_DB_PATH`:

```bash
# Bash / macOS / Linux
PLH_DB_PATH=./data/python-learning-hub.db cargo run
```

```powershell
# PowerShell
$env:PLH_DB_PATH = "D:\data\python-learning-hub.db"; cargo run
```

The server binds to `127.0.0.1:3000` and embeds the site at compile time. After editing `index.html`, `src/`, or replacing an image in `assets/`, stop the server and run `cargo run` again — a rebuild is required.

### Option 2 — any static file server

Because the production site is fully static, no Rust toolchain is needed for content work:

```bash
python -m http.server 8090
```

Then open [http://127.0.0.1:8090](http://127.0.0.1:8090). The leaderboard card stays hidden, roadmap progress stays in `localStorage`, and the console remains clean.

### Tests and checks

With the Rust toolchain and a C compiler installed:

```bash
cargo test                          # unit tests for ranking, progress, validation
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

The integration script used during development exercised the running server end to end: HTML and `HEAD` detection, health, quiz submissions, personal bests, shared ranks, validation failures, roadmap round-trips, image ranges, and 404 fallback.

---

## Rust server

`src/main.rs` embeds the site into the binary:

```rust
const INDEX_HTML: &str = include_str!("../index.html");
const CREATOR_IMAGE: &[u8] = include_bytes!("../assets/creator.jpg");
const CREATOR_IMAGE_SM: &[u8] = include_bytes!("../assets/creator-sm.jpg");
```

Routes:

| Method | Route | Response |
| --- | --- | --- |
| `GET` | `/`, `/index.html` | The site as `text/html`, with `X-PLH-Backend: sqlite` |
| `GET` | `/assets/creator.jpg` | Embedded 1400 px portrait |
| `GET` | `/assets/creator-sm.jpg` | Embedded 700 px portrait |
| `GET` | `/api/health` | `{ "status": "ok", "database": "sqlite", "version": "..." }` |
| `GET` | `/api/leaderboard?limit=10` | Top scores, one row per player |
| `POST` | `/api/quiz-attempts` | Validate and store a quiz attempt |
| `GET` | `/api/progress/:client_id` | Read roadmap progress for one browser |
| `PUT` | `/api/progress/:client_id` | Insert or replace roadmap progress |
| `DELETE` | `/api/progress/:client_id` | Delete roadmap progress |
| `*` | anything else | `404 Not Found` |

Embedded images are served with `Content-Type`, `Content-Length`, `Accept-Ranges`, and a one-year immutable `Cache-Control`. Single-range requests return `206 Partial Content` with `Content-Range`; unsatisfiable ranges return `416`.

The social card (`assets/og-card.png`) is a GitHub Pages asset referenced by absolute URL in the Open Graph metadata. The Rust server intentionally embeds only the two portraits the page requests.

### SQLite database

The database is a single local file, opened in WAL mode with a five-second busy timeout. Schema:

```sql
CREATE TABLE quiz_attempts (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    score      INTEGER NOT NULL,
    total      INTEGER NOT NULL,
    created_at TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX idx_quiz_attempts_rank
    ON quiz_attempts (score DESC, created_at ASC);

CREATE TABLE roadmap_progress (
    client_id  TEXT PRIMARY KEY,
    completed  TEXT NOT NULL,   -- JSON array of booleans
    updated_at TEXT NOT NULL
);
```

Validation happens in `src/api.rs` before SQLite is touched:

- `total` must equal **8**, matching the quiz in `index.html`.
- `score` must be between `0` and `total`.
- Names are trimmed, must be **2–20 characters**, and cannot contain control characters.
- `client_id` must be **8–64** characters using letters, numbers, `-`, or `_`.
- Progress accepts at most **20** boolean steps.

Every attempt is stored, but the leaderboard groups by player name and keeps each player’s best score. Ties share a rank. Browser code supplies a UUID stored under `plh-client-v1`; the database uses it to keep roadmap progress separate per browser.

### API examples

```bash
curl http://127.0.0.1:3000/api/health
curl "http://127.0.0.1:3000/api/leaderboard?limit=10"

curl -X POST http://127.0.0.1:3000/api/quiz-attempts \
  -H "Content-Type: application/json" \
  -d '{"name":"Ada","score":8,"total":8}'

curl -X PUT http://127.0.0.1:3000/api/progress/00000000-0000-4000-8000-000000000000 \
  -H "Content-Type: application/json" \
  -d '{"completed":[true,false,false,false,false]}'
```

### Database maintenance

- **Backup:** use SQLite’s online backup command (`sqlite3 python-learning-hub.db ".backup backup.db"`) or stop the server and copy the `.db` file.
- **Reset:** stop the server, remove `python-learning-hub.db` (and any `-wal` / `-shm` files), then start it again.
- **Upgrade schema:** `CREATE TABLE IF NOT EXISTS` does not migrate existing columns. Add an explicit migration before changing a table.
- **Security:** there is no authentication. Names and scores are visible to anyone who can reach the API. Keep the server on loopback, or put authentication, rate limiting, and TLS in a reverse proxy before exposing it.
- **Remote hosting:** change the bind address in `src/main.rs` from loopback to `0.0.0.0` (or your platform’s required address) and place a reverse proxy in front of it.

### How the frontend detects the database

The Rust server adds `X-PLH-Backend: sqlite` to the HTML response. The page sends one `HEAD` request to its own directory and enables the leaderboard and SQLite sync only when that header is present. On GitHub Pages the header is absent, so the static site does not call `/api/*` and logs no failed requests.

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

## Quiz, roadmap, and database data

The quiz is defined as a plain JavaScript array in `index.html`, with one correct index and a short explanation per question. The server’s `QUIZ_TOTAL` constant in `src/api.rs` must match the quiz length.

The roadmap uses two browser keys:

```js
const RP_KEY = 'plh-roadmap-v1';
const CLIENT_KEY = 'plh-client-v1';
```

Progress is stored as a boolean array. If you add, remove, or reorder milestones:

1. Change `RP_KEY` (for example, to `plh-roadmap-v2`) so older saved arrays are ignored.
2. Update `MAX_PROGRESS_STEPS` in `src/api.rs` if the new length exceeds 20.
3. Clear the `roadmap_progress` table, or convert the stored JSON arrays before deploying.

When SQLite is available, the server record wins on page load. If the server has no record for the browser but local progress exists, the page uploads the local array. Every checkbox change and reset is written to both `localStorage` and SQLite. In static mode, only `localStorage` is used.

No analytics are collected. In static mode, no quiz scores or personal data are sent anywhere. With the Rust server, the submitted name, score, and progress array are stored only in the local SQLite file and are never forwarded to another service.

---

## Editing the site

1. **Content, styles, and behaviour** live in `index.html` — one main inline `<style>` block (plus a tiny noscript fallback) and one `<script>` block.
2. **Portraits** — replace `assets/creator.jpg` and `assets/creator-sm.jpg`. Keep a 3:4 portrait ratio and make the smaller image roughly half the width of the large one for the `srcset`.
3. **Social card** — replace `assets/og-card.png` with a 1200 × 630 image. If the site moves to a custom domain, update `og:url`, `og:image`, and `twitter:image` in the head.
4. **Roadmap** — after changing milestones, bump `RP_KEY` as described above.
5. **Quiz** — after changing the number of questions, update `QUIZ_TOTAL` in `src/api.rs`.
6. **Database** — add a migration in `src/db.rs` before changing an existing table.
7. **Rust server** — rebuild with `cargo run` after changing embedded files or server code.
8. **GitHub Pages** — commit and push to `main`; no build step is needed.

---

## Verification

| Check | Result |
| --- | --- |
| Static audit (structure, anchors, CSS classes, Rust includes) | Pass |
| Rust build, unit tests, and lints | `cargo build`, 4 `cargo test` tests, `cargo clippy -D warnings`, and `cargo fmt --check` pass on Rust 1.100.0 |
| Live API integration | 50 checks passed against the running Axum server with bundled SQLite |
| Browser console errors (static and real SQLite server) | 0 |
| Lighthouse Accessibility / Best Practices / SEO | **100 / 100 / 100** on static and SQLite modes |
| Python runtime | CPython **3.13.2** executes and returns output |
| Layout overflow | None at tested desktop and mobile widths |
| External resource links | Verified reachable (November 2026) |
| SQLite schema and API contract | Validated with SQLite, a mock HTTP harness, and the real Rust server |
| Leaderboard flow | Submit, personal best, deduplication, shared ranks, and reload verified against the real server |
| Roadmap sync | SQLite PUT, reload, and reset verified against the real server |

Runtime checks covered the loader and skip control, all seven code tabs, the demo terminal, the Pyodide playground, quiz scoring and restart, leaderboard submission, roadmap persistence and SQLite sync, the mobile menu, scroll progress, scroll spy, back-to-top, and the skip link.

The site honours the operating system’s **reduce-motion** preference. To preview the full motion system, enable animations in the OS accessibility settings.

---

## Deployment

### GitHub Pages (static)

The repository is served directly from the root of `main`:

- Site: [https://irashmethsara61-bo.github.io/Python-Learning-Hub/](https://irashmethsara61-bo.github.io/Python-Learning-Hub/)
- Social card: `https://irashmethsara61-bo.github.io/Python-Learning-Hub/assets/og-card.png`

Push changes, wait for the Pages build (about a minute), then hard-refresh with `Ctrl + F5` if the old version is cached.

GitHub Pages cannot run the Rust server or SQLite. The live site therefore uses browser storage and hides the leaderboard card.

### Self-hosted Rust

```bash
cargo build --release
PLH_DB_PATH=./data/python-learning-hub.db ./target/release/python-learning-site
```

The binary binds to loopback by default, assets are embedded at compile time, and the SQLite file is created if it does not exist. Back it up before upgrades.

---

## Constraints

- The playground needs network access on its first run to fetch Pyodide from jsDelivr.
- Roadmap progress is per-browser and per-origin; it does not sync across devices or browsers.
- The SQLite leaderboard and progress sync work only when the site is served by the Rust server.
- There are no accounts, authentication, or analytics.
- The leaderboard is public to anyone who can reach the API; do not expose it without protection.
- The Rust server is optional; GitHub Pages serves the static site.
- Bundled SQLite needs a C compiler when building the Rust binary (MSVC Build Tools, or a MinGW-w64 toolchain such as WinLibs).

---

## Credits

Built by [Irash](https://github.com/irashmethsara61-bo) as a free learning resource.

© 2026 Python-Learning-Hub
