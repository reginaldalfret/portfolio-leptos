import http.server
import socketserver
import os

PORT = int(os.environ.get("PORT", 8089))
DIST_DIR = os.path.join(os.path.dirname(__file__), "dist")

class LeptosHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIST_DIR, **kwargs)

    def do_GET(self):
        if self.path == "/" or self.path == "/stacks/leptos" or self.path == "/stacks/leptos/":
            self.path = "/index.html"
        elif self.path.startswith("/stacks/leptos/"):
            self.path = self.path[len("/stacks/leptos"):]
        return super().do_GET()

if __name__ == "__main__":
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("127.0.0.1", PORT), LeptosHandler) as httpd:
        print(f"Serving Leptos on http://127.0.0.1:{PORT}")
        httpd.serve_forever()
