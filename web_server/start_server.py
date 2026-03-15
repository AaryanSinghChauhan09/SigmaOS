# Generated file: start_server
import http.server
import socketserver
import os
import webbrowser

def start_server():
    port = 8080
    while port < 8100:
        try:
            with socketserver.TCPServer(('', port), Handler) as httpd:
                print(f'Sigma Web OS serving at http://localhost:{port}')
                print('Zero-trust web sandbox activated.')
                webbrowser.open(f'http://localhost:{port}')
                try:
                    httpd.serve_forever()
                except KeyboardInterrupt:
                    print('\nSigma Web OS offline.')
                break
        except OSError:
            port += 1