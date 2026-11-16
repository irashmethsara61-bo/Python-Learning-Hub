# Python Learning Hub (Rust + Axum)

An advanced, responsive single-page website that explains **how Python helps people learn programming**. The same `index.html` is served by a small Rust (Axum) server for local use and by GitHub Pages in production.

**Live site:** [https://irashmethsara61-bo.github.io/Irash/](https://irashmethsara61-bo.github.io/Irash/)

## Highlights

- **Animated, video-free loading screen** — glass card, rotating gradient ring, progress bar, staged status text, and a skip control
- **Motion system**: scroll progress bar, sticky-nav state, scroll-spy active links, staggered scroll reveals, gradient shimmer, button shine, card lifts, animated mobile menu, code-tab cross-fades, quiz-option stagger, timeline completion pulse, terminal line fade-in, and a back-to-top control
- Dark "epic" UI with gradient orbs, glass navigation, and smooth micro-interactions — mostly transform/opacity based, rAF-throttled, with reduced-motion fallbacks
- Animated hero statistics
- **7 professional code example tabs** in a mock editor window: Hello World, Loops, Functions, Lists, FizzBuzz, Objects, and Files
- **Real Python playground** — genuine CPython (3.13) in the browser via Pyodide, lazy-loaded on demand, with stdout/stderr capture, expression results, Ctrl+Enter to run, and Tab indenting
- **Interactive Python quiz** — 8 questions with instant feedback, explanations, progress dots, scoring, and restart
- **Roadmap progress tracker** — checkable milestones saved in `localStorage` with a live progress bar and reset
- **Social share card** — Open Graph / Twitter meta with a generated 1200×630 preview image (`assets/og-card.png`)
- Interactive in-browser demo terminal (with output animation) — prompt bug fixed, output verified
- Epic creator showcase with a cinematic portrait, HUD frame, rotating rays, halftone, scanlines, and gradient name
- Curated "learn Python online" section with a 3-step fast-start path and 12 verified resource cards (badges, cursor spotlight, all links confirmed reachable)
- Learning roadmap, language comparison table, project ideas, resources, and FAQ
- Accessible by design: skip-to-content link, visible focus states, sequential headings, and full `prefers-reduced-motion` support
- Optimized runtime: zero inline `style` attributes, cached DOM collections, one layout read per pointer frame, passive scroll listeners, and compositor-friendly animations
- Inline SVG favicon (gradient "Py" badge)

## Audit status

- Video loading screen fully removed (no `<video>` / `loading.mp4` / poster references remain)
- Full static audit passed: HTML structure, CSS, JS wiring, anchors, images, Rust includes/routes
- Lighthouse: **Accessibility 100 · Best Practices 100 · SEO 100** (no failed audits)
- Runtime verified in browser: 0 console errors; loader/skip, playground (CPython 3.13.2), quiz, roadmap persistence, scroll progress, nav scroll spy, mobile menu, and back-to-top all functional
- Navigation collapses to an animated mobile menu at 1040px to prevent crowding on laptops
- All external resource links checked and confirmed reachable

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
├── index.html            # Entire site: HTML, CSS, JS, playground, quiz, tracker
├── assets/
│   ├── creator.jpg       # Creator portrait, 1400px wide
│   ├── creator-sm.jpg    # Creator portrait, 700px wide (srcset)
│   └── og-card.png       # 1200x630 social share card (Open Graph / Twitter)
└── README.md
```

## Editing the site

1. Update `index.html`, its inline `<style>` / `<script>`, and content sections as needed.
2. Replace `assets/creator.jpg` and `assets/creator-sm.jpg` to change the creator portrait. Keep the smaller image at roughly half the width of the large one for the `srcset`.
3. Rebuild and run `cargo run` to see changes locally.

## Deployment

- **GitHub Pages** serves `index.html` and `assets/` directly from the repository. Pushing to `main` triggers an automatic rebuild.
- **Rust server** is intended for local hosting and can be deployed to any Rust-capable platform.
