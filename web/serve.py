"""Local dev server for the web/ landing + Playground.

Threading server with explicit MIME for ES modules and .wasm.
Run from repo root:

    python web/serve.py

Then open http://localhost:8765/web/
"""

import http.server
import socketserver
import os

PORT = 8765
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(ROOT)


class Handler(http.server.SimpleHTTPRequestHandler):
    def guess_type(self, path):
        if path.endswith(".js") or path.endswith(".mjs"):
            return "application/javascript"
        if path.endswith(".wasm"):
            return "application/wasm"
        return super().guess_type(path)

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


class Threaded(socketserver.ThreadingMixIn, http.server.HTTPServer):
    daemon_threads = True
    allow_reuse_address = True


if __name__ == "__main__":
    with Threaded(("127.0.0.1", PORT), Handler) as httpd:
        print(f"Serving {ROOT}")
        print(f"http://localhost:{PORT}/web/")
        httpd.serve_forever()
