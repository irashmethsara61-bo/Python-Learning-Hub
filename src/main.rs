use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn home() -> Html<&'static str> {
    Html(INDEX_HTML)
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>How Python Helps People Learn Programming</title>
  <style>
    :root {
      --bg: #0f172a;
      --card: #111827;
      --text: #e5e7eb;
      --muted: #9ca3af;
      --accent: #38bdf8;
      --accent2: #facc15;
      --border: #243041;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      font-family: system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif;
      background: linear-gradient(180deg, #0b1220 0%, var(--bg) 100%);
      color: var(--text);
      line-height: 1.6;
    }
    header {
      padding: 4rem 1.5rem 2rem;
      text-align: center;
      border-bottom: 1px solid var(--border);
      background: radial-gradient(circle at top, rgba(56,189,248,0.15), transparent 60%);
    }
    .badge {
      display: inline-block;
      padding: 0.35rem 0.75rem;
      border: 1px solid rgba(56,189,248,0.4);
      color: var(--accent);
      border-radius: 999px;
      font-size: 0.9rem;
      margin-bottom: 1rem;
    }
    h1 {
      margin: 0 auto 0.75rem;
      max-width: 900px;
      font-size: clamp(2rem, 4vw, 3.5rem);
      line-height: 1.15;
    }
    .subtitle {
      max-width: 760px;
      margin: 0 auto;
      color: var(--muted);
      font-size: 1.1rem;
    }
    main {
      max-width: 1100px;
      margin: 0 auto;
      padding: 2rem 1.5rem 4rem;
    }
    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
      gap: 1rem;
      margin-top: 1.5rem;
    }
    .card {
      background: rgba(17, 24, 39, 0.85);
      border: 1px solid var(--border);
      border-radius: 16px;
      padding: 1.25rem;
      box-shadow: 0 10px 30px rgba(0,0,0,0.2);
    }
    .card h3 {
      margin-top: 0;
      margin-bottom: 0.5rem;
      color: #fff;
    }
    .card p { margin: 0; color: var(--muted); }
    section { margin-top: 2.5rem; }
    h2 {
      font-size: 1.6rem;
      margin-bottom: 0.75rem;
    }
    .example {
      background: #0b1020;
      border: 1px solid var(--border);
      border-radius: 14px;
      padding: 1rem;
      overflow-x: auto;
      color: #d1fae5;
    }
    pre { margin: 0; font-size: 0.95rem; }
    .highlight { color: var(--accent2); font-weight: 600; }
    ul { color: var(--muted); }
    footer {
      text-align: center;
      color: var(--muted);
      padding: 2rem 1rem 3rem;
      border-top: 1px solid var(--border);
    }
    a { color: var(--accent); }
  </style>
</head>
<body>
  <header>
    <div class="badge">Built with Rust + Axum</div>
    <h1>How Python Helps People Learn Programming</h1>
    <p class="subtitle">
      Python is one of the most beginner-friendly programming languages. Its readable syntax,
      supportive community, and wide range of uses make it an excellent first step into coding.
    </p>
  </header>

  <main>
    <section>
      <h2>Why Python is great for beginners</h2>
      <div class="grid">
        <article class="card">
          <h3>Readable syntax</h3>
          <p>Python code often reads like plain English, helping learners focus on concepts instead of complex symbols.</p>
        </article>
        <article class="card">
          <h3>Quick feedback</h3>
          <p>Beginners can run small programs immediately, experiment, and learn from mistakes without long setup times.</p>
        </article>
        <article class="card">
          <h3>Large community</h3>
          <p>Tutorials, forums, libraries, and learning resources are widely available when learners get stuck.</p>
        </article>
        <article class="card">
          <h3>Real-world uses</h3>
          <p>Python is used in web development, data science, automation, AI, and more, keeping learners motivated.</p>
        </article>
      </div>
    </section>

    <section>
      <h2>A simple first program</h2>
      <p>New programmers can write useful code quickly. This example shows how simple Python can be:</p>
      <div class="example">
        <pre><code># A friendly Python program
name = input("What is your name? ")
print(f"Hello, {name}! Welcome to programming.")</code></pre>
      </div>
    </section>

    <section>
      <h2>How Python builds confidence</h2>
      <ul>
        <li><span class="highlight">Low barrier to entry:</span> learners can start with basic ideas like variables, loops, and functions.</li>
        <li><span class="highlight">Problem-solving focus:</span> clear code makes it easier to understand logic and debug errors.</li>
        <li><span class="highlight">Transferable skills:</span> concepts learned in Python apply to many other programming languages.</li>
        <li><span class="highlight">Project-friendly:</span> students can build games, scripts, websites, and data projects early on.</li>
      </ul>
    </section>

    <section>
      <h2>Conclusion</h2>
      <p>
        Python helps people learn programming by making code approachable, encouraging experimentation,
        and opening doors to real-world projects. For many beginners, it is the perfect language to
        turn curiosity into confidence.
      </p>
    </section>
  </main>

  <footer>
    <p>Served by a Rust web server using <a href="https://github.com/tokio-rs/axum" target="_blank" rel="noreferrer">Axum</a>.</p>
  </footer>
</body>
</html>
"#;
