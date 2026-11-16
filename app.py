from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

HOST = "127.0.0.1"
PORT = 8000
BASE_DIR = Path(__file__).resolve().parent


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        path = self.path.split("?", 1)[0]

        if path in ("/", "/index.html"):
            try:
                content = (BASE_DIR / "index.html").read_bytes()
            except FileNotFoundError:
                self._send(500, "text/plain; charset=utf-8", b"index.html not found")
                return

            self._send(200, "text/html; charset=utf-8", content)
        else:
            self._send(404, "text/plain; charset=utf-8", b"404 Not Found")

    def _send(self, status, content_type, body):
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

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
