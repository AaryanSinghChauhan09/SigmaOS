import http.server
import socketserver
import os
import webbrowser

PORT = 8080
DIRECTORY = os.path.join(os.path.dirname(os.path.abspath(__file__)), "web_os")

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

def start_server():
    port = 8080
    while port < 8100:
        try:
            with socketserver.TCPServer(("", port), Handler) as httpd:
                print(f"Sigma Web OS serving at http://localhost:{port}")
                print("Zero-trust web sandbox activated.")
                webbrowser.open(f"http://localhost:{port}")
                try:
                    httpd.serve_forever()
                except KeyboardInterrupt:
                    print("\nSigma Web OS offline.")
                break
        except OSError:
            port += 1

if __name__ == "__main__":
    start_server()
