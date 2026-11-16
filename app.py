from http.server import BaseHTTPRequestHandler, HTTPServer

HOST = "127.0.0.1"
PORT = 8000

HTML_PAGE = r"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>How Python Helps People Learn Programming</title>
  <style>
    :root {
      --bg: #070b18;
      --bg-2: #0d1328;
      --surface: rgba(17, 24, 39, 0.72);
      --surface-2: rgba(15, 23, 42, 0.9);
      --text: #e6edf7;
      --muted: #9fb0c9;
      --accent: #38bdf8;
      --accent-2: #a78bfa;
      --accent-3: #22d3ee;
      --success: #34d399;
      --warning: #fbbf24;
      --border: rgba(148, 163, 184, 0.18);
      --shadow: 0 20px 50px rgba(0, 0, 0, 0.35);
      --radius: 20px;
    }

    * { box-sizing: border-box; margin: 0; padding: 0; }
    html { scroll-behavior: smooth; }
    body {
      font-family: Inter, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif;
      color: var(--text);
      background:
        radial-gradient(circle at 10% 20%, rgba(56,189,248,0.18), transparent 25%),
        radial-gradient(circle at 90% 10%, rgba(167,139,250,0.18), transparent 25%),
        radial-gradient(circle at 50% 100%, rgba(34,211,238,0.12), transparent 30%),
        linear-gradient(180deg, var(--bg) 0%, var(--bg-2) 100%);
      line-height: 1.6;
      min-height: 100vh;
      overflow-x: hidden;
    }

    .bg-grid {
      position: fixed;
      inset: 0;
      background-image:
        linear-gradient(rgba(148,163,184,0.05) 1px, transparent 1px),
        linear-gradient(90deg, rgba(148,163,184,0.05) 1px, transparent 1px);
      background-size: 40px 40px;
      mask-image: radial-gradient(circle at center, black 35%, transparent 85%);
      pointer-events: none;
      z-index: -1;
    }

    .container {
      width: min(1180px, calc(100% - 2rem));
      margin: 0 auto;
    }

    nav {
      position: sticky;
      top: 0;
      z-index: 20;
      backdrop-filter: blur(14px);
      background: rgba(7, 11, 24, 0.55);
      border-bottom: 1px solid var(--border);
    }
    .nav-inner {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
      padding: 0.9rem 0;
    }
    .brand {
      display: flex;
      align-items: center;
      gap: 0.7rem;
      font-weight: 700;
      letter-spacing: 0.2px;
    }
    .logo {
      width: 36px; height: 36px; border-radius: 12px;
      display: grid; place-items: center;
      background: linear-gradient(135deg, var(--accent), var(--accent-2));
      color: #04111f; font-weight: 800;
      box-shadow: 0 8px 20px rgba(56,189,248,0.25);
    }
    .nav-links {
      display: flex; gap: 1rem; flex-wrap: wrap;
      color: var(--muted); font-size: 0.95rem;
    }
    .nav-links a { color: inherit; text-decoration: none; }
    .nav-links a:hover { color: var(--text); }

    .hero {
      padding: 5rem 0 3rem;
      position: relative;
    }
    .hero-grid {
      display: grid;
      grid-template-columns: 1.2fr 0.8fr;
      gap: 2rem;
      align-items: center;
    }
    .eyebrow {
      display: inline-flex;
      align-items: center;
      gap: 0.5rem;
      padding: 0.4rem 0.8rem;
      border: 1px solid rgba(56,189,248,0.35);
      background: rgba(56,189,248,0.08);
      color: #bfeeff;
      border-radius: 999px;
      font-size: 0.9rem;
      margin-bottom: 1rem;
    }
    h1 {
      font-size: clamp(2.2rem, 5vw, 4.2rem);
      line-height: 1.08;
      letter-spacing: -0.03em;
      margin-bottom: 1rem;
    }
    .gradient-text {
      background: linear-gradient(90deg, #fff 10%, var(--accent) 45%, var(--accent-2) 90%);
      -webkit-background-clip: text;
      background-clip: text;
      color: transparent;
    }
    .lead {
      color: var(--muted);
      font-size: 1.1rem;
      max-width: 65ch;
      margin-bottom: 1.5rem;
    }
    .hero-actions { display: flex; gap: 0.8rem; flex-wrap: wrap; }
    .btn {
      display: inline-flex; align-items: center; justify-content: center;
      padding: 0.85rem 1.15rem;
      border-radius: 12px;
      text-decoration: none;
      font-weight: 600;
      border: 1px solid var(--border);
      transition: transform .2s ease, box-shadow .2s ease, border-color .2s ease;
    }
    .btn:hover { transform: translateY(-2px); }
    .btn-primary {
      color: #04111f;
      background: linear-gradient(135deg, var(--accent), var(--accent-3));
      border: none;
      box-shadow: 0 10px 25px rgba(56,189,248,0.25);
    }
    .btn-secondary { color: var(--text); background: rgba(255,255,255,0.03); }

    .hero-panel {
      background: linear-gradient(180deg, rgba(30,41,59,0.7), rgba(15,23,42,0.85));
      border: 1px solid var(--border);
      border-radius: 24px;
      padding: 1.25rem;
      box-shadow: var(--shadow);
      position: relative;
      overflow: hidden;
    }
    .hero-panel::before {
      content: "";
      position: absolute; inset: -30% auto auto 40%;
      width: 220px; height: 220px; border-radius: 50%;
      background: radial-gradient(circle, rgba(56,189,248,0.25), transparent 70%);
      filter: blur(10px);
    }
    .stat-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0.8rem; position: relative; }
    .stat {
      padding: 1rem;
      border: 1px solid var(--border);
      border-radius: 16px;
      background: rgba(255,255,255,0.03);
    }
    .stat .value { font-size: 1.5rem; font-weight: 700; color: #fff; }
    .stat .label { color: var(--muted); font-size: 0.9rem; }

    section { padding: 2rem 0; }
    .section-head { margin-bottom: 1.2rem; }
    .section-head h2 {
      font-size: clamp(1.6rem, 3vw, 2.2rem);
      margin-bottom: 0.4rem;
      letter-spacing: -0.02em;
    }
    .section-head p { color: var(--muted); max-width: 70ch; }

    .cards {
      display: grid;
      grid-template-columns: repeat(4, minmax(0, 1fr));
      gap: 1rem;
    }
    .card {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 1.25rem;
      box-shadow: var(--shadow);
      transition: transform .25s ease, border-color .25s ease, background .25s ease;
    }
    .card:hover {
      transform: translateY(-4px);
      border-color: rgba(56,189,248,0.35);
      background: rgba(17, 24, 39, 0.9);
    }
    .icon {
      width: 42px; height: 42px; border-radius: 12px;
      display: grid; place-items: center; margin-bottom: 0.8rem;
      font-size: 1.2rem;
      background: rgba(56,189,248,0.12);
      border: 1px solid rgba(56,189,248,0.2);
    }
    .card h3 { margin-bottom: 0.35rem; font-size: 1.1rem; }
    .card p { color: var(--muted); font-size: 0.97rem; }

    .split {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1rem;
      align-items: stretch;
    }
    .panel {
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 1.25rem;
      box-shadow: var(--shadow);
    }
    .code-window {
      background: #0a1020;
      border: 1px solid #22304a;
      border-radius: 16px;
      overflow: hidden;
    }
    .code-top {
      display: flex; align-items: center; gap: 0.4rem;
      padding: 0.7rem 0.9rem;
      border-bottom: 1px solid #22304a;
      color: #8aa0bf; font-size: 0.85rem;
      background: rgba(255,255,255,0.02);
    }
    .dot { width: 10px; height: 10px; border-radius: 50%; }
    .dot.r { background: #fb7185; } .dot.y { background: #fbbf24; } .dot.g { background: #34d399; }
    pre {
      margin: 0; padding: 1rem; overflow-x: auto;
      color: #d7ffe8; font-size: 0.95rem; line-height: 1.7;
    }
    .kw { color: #93c5fd; } .str { color: #86efac; } .fn { color: #f9a8d4; } .cm { color: #7c93b2; }

    .steps { display: grid; gap: 0.8rem; }
    .step {
      display: grid; grid-template-columns: auto 1fr; gap: 0.9rem;
      padding: 0.9rem; border: 1px solid var(--border); border-radius: 14px;
      background: rgba(255,255,255,0.02);
    }
    .step-num {
      width: 34px; height: 34px; border-radius: 10px; display: grid; place-items: center;
      font-weight: 700; color: #04111f;
      background: linear-gradient(135deg, var(--warning), #fde68a);
    }
    .step p { color: var(--muted); font-size: 0.95rem; }

    .pill-list { display: flex; flex-wrap: wrap; gap: 0.6rem; margin-top: 0.8rem; }
    .pill {
      padding: 0.45rem 0.75rem; border-radius: 999px;
      border: 1px solid var(--border); color: #dbe7f7;
      background: rgba(255,255,255,0.03); font-size: 0.9rem;
    }

    .cta {
      margin: 2rem 0 1rem;
      padding: 1.5rem;
      border-radius: 24px;
      border: 1px solid rgba(56,189,248,0.25);
      background: linear-gradient(135deg, rgba(56,189,248,0.12), rgba(167,139,250,0.12));
      display: flex; align-items: center; justify-content: space-between; gap: 1rem; flex-wrap: wrap;
    }
    .cta p { color: var(--muted); }

    footer {
      padding: 2rem 0 3rem;
      color: var(--muted);
      text-align: center;
    }

    @media (max-width: 980px) {
      .hero-grid, .split { grid-template-columns: 1fr; }
      .cards { grid-template-columns: repeat(2, minmax(0, 1fr)); }
    }
    @media (max-width: 640px) {
      .nav-inner { flex-direction: column; align-items: flex-start; }
      .cards { grid-template-columns: 1fr; }
      .stat-grid { grid-template-columns: 1fr 1fr; }
      .hero { padding-top: 3.5rem; }
    }
  </style>
</head>
<body>
  <div class="bg-grid"></div>
  <nav>
    <div class="container nav-inner">
      <div class="brand"><div class="logo">Py</div> Python Learning Hub</div>
      <div class="nav-links">
        <a href="#why">Why Python</a>
        <a href="#example">Example</a>
        <a href="#journey">Learning Path</a>
      </div>
    </div>
  </nav>

  <header class="hero">
    <div class="container hero-grid">
      <div>
        <div class="eyebrow">✨ Beginner-friendly • Powerful • Versatile</div>
        <h1>How <span class="gradient-text">Python</span> Helps People Learn Programming</h1>
        <p class="lead">
          Python lowers the barrier to coding with clean syntax, fast feedback, and a supportive ecosystem.
          It helps beginners turn ideas into working programs quickly—building confidence from day one.
        </p>
        <div class="hero-actions">
          <a class="btn btn-primary" href="#example">See a Python Example</a>
          <a class="btn btn-secondary" href="#journey">Explore the Learning Path</a>
        </div>
      </div>
      <aside class="hero-panel" aria-label="Highlights">
        <div class="stat-grid">
          <div class="stat">
            <div class="value">Simple</div>
            <div class="label">Readable, English-like syntax</div>
          </div>
          <div class="stat">
            <div class="value">Fast</div>
            <div class="label">Quick experiments & feedback</div>
          </div>
          <div class="stat">
            <div class="value">Huge</div>
            <div class="label">Community, tutorials, libraries</div>
          </div>
          <div class="stat">
            <div class="value">Useful</div>
            <div class="label">Web, data, AI, automation</div>
          </div>
        </div>
      </aside>
    </div>
  </header>

  <main class="container">
    <section id="why">
      <div class="section-head">
        <h2>Why Python is great for beginners</h2>
        <p>Python helps learners focus on problem-solving and core programming concepts instead of getting stuck on complicated syntax.</p>
      </div>
      <div class="cards">
        <article class="card">
          <div class="icon">📖</div>
          <h3>Readable syntax</h3>
          <p>Code is clear and concise, making it easier to understand what a program does at a glance.</p>
        </article>
        <article class="card">
          <div class="icon">⚡</div>
          <h3>Instant feedback</h3>
          <p>Run small scripts right away, test ideas quickly, and learn by experimenting.</p>
        </article>
        <article class="card">
          <div class="icon">🤝</div>
          <h3>Supportive community</h3>
          <p>Millions of learners and developers share guides, answers, and open-source tools.</p>
        </article>
        <article class="card">
          <div class="icon">🚀</div>
          <h3>Real-world impact</h3>
          <p>Skills transfer to practical projects in web apps, data analysis, automation, and AI.</p>
        </article>
      </div>
    </section>

    <section id="example">
      <div class="section-head">
        <h2>Simple code, powerful results</h2>
        <p>A beginner can write a friendly interactive program in just a few lines of Python.</p>
      </div>
      <div class="split">
        <div class="panel code-window">
          <div class="code-top">
            <span class="dot r"></span><span class="dot y"></span><span class="dot g"></span>
            <span style="margin-left:.4rem;">hello.py</span>
          </div>
          <pre><code><span class="cm"># A friendly Python program</span>
<span class="kw">name</span> = <span class="fn">input</span>(<span class="str">"What is your name? "</span>)
<span class="fn">print</span>(<span class="str">f"Hello, {name}! Welcome to programming."</span>)

<span class="cm"># Try a simple calculation</span>
<span class="kw">hours</span> = <span class="fn">int</span>(<span class="fn">input</span>(<span class="str">"How many hours will you practice today? "</span>))
<span class="fn">print</span>(<span class="str">f"Great! {hours * 7} hours in a week at that pace."</span>)</code></pre>
        </div>
        <div class="panel">
          <h3 style="margin-bottom: .6rem;">What learners gain from this</h3>
          <div class="steps">
            <div class="step">
              <div class="step-num">1</div>
              <div><strong>Core concepts</strong><p>Variables, input/output, strings, and basic math in an approachable form.</p></div>
            </div>
            <div class="step">
              <div class="step-num">2</div>
              <div><strong>Confidence</strong><p>Seeing immediate results makes programming feel achievable and fun.</p></div>
            </div>
            <div class="step">
              <div class="step-num">3</div>
              <div><strong>Curiosity</strong><p>Small wins encourage learners to explore loops, functions, and projects next.</p></div>
            </div>
          </div>
          <div class="pill-list">
            <span class="pill">Variables</span>
            <span class="pill">Input & Output</span>
            <span class="pill">f-strings</span>
            <span class="pill">Logic</span>
            <span class="pill">Problem Solving</span>
          </div>
        </div>
      </div>
    </section>

    <section id="journey">
      <div class="section-head">
        <h2>A beginner-friendly learning path</h2>
        <p>Python supports steady progress—from first scripts to real applications.</p>
      </div>
      <div class="cards">
        <article class="card">
          <div class="icon">1️⃣</div>
          <h3>Start with basics</h3>
          <p>Learn variables, data types, conditions, and loops with simple, readable examples.</p>
        </article>
        <article class="card">
          <div class="icon">2️⃣</div>
          <h3>Build mini projects</h3>
          <p>Create calculators, quizzes, or automation scripts to apply concepts in practice.</p>
        </article>
        <article class="card">
          <div class="icon">3️⃣</div>
          <h3>Explore libraries</h3>
          <p>Use tools for web development, data analysis, games, and more as skills grow.</p>
        </article>
        <article class="card">
          <div class="icon">4️⃣</div>
          <h3>Create real solutions</h3>
          <p>Turn ideas into useful apps and projects, reinforcing both creativity and confidence.</p>
        </article>
      </div>

      <div class="cta">
        <div>
          <h3 style="margin-bottom:.25rem;">Why it works</h3>
          <p>Python combines simplicity with real-world power, making it one of the best first languages for new programmers.</p>
        </div>
        <a class="btn btn-primary" href="#why">Back to Top</a>
      </div>
    </section>
  </main>

  <footer class="container">
    <p>Served with Python’s built-in <code>http.server</code> • Designed to inspire new programmers</p>
  </footer>
</body>
</html>
"""


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path in ("/", "/index.html"):
            content = HTML_PAGE.encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", str(len(content)))
            self.end_headers()
            self.wfile.write(content)
        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"404 Not Found")

    def log_message(self, format, *args):
        # Keep console output clean
        return


if __name__ == "__main__":
    server = HTTPServer((HOST, PORT), Handler)
    print(f"Server running at http://{HOST}:{PORT}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nServer stopped.")
    finally:
        server.server_close()
