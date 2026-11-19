#!/usr/bin/env python3
"""
SigmaOS Sovereign Graphical Desktop
=====================================
Polyglot GUI desktop interface for SigmaOS Sovereign Workstation.
Provides four execution modalities with zero-friction automatic fallback:

  1. GTK3 + WebKit2 native window with bidirectional IPC bridge
  2. Pure GTK3 native desktop app (scripts/sigmaos_native_app.py)
  3. Dedicated Browser Application mode (Chrome/Chromium --app)
  4. Sovereign Local Web Desktop Server (http://127.0.0.1:PORT)
  5. Interactive terminal REPL (fallback when headless)
"""

import sys
import os
import subprocess
import shutil
import threading
import json
import time
import warnings
import http.server
import socketserver
import webbrowser
import socket
from urllib.parse import urlparse

warnings.filterwarnings("ignore", category=DeprecationWarning)

# ── WebKit environment configuration ─────────────────────────────────────────
os.environ.setdefault("WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS", "1")
os.environ.setdefault("WEBKIT_DISABLE_COMPOSITING_MODE", "1")
os.environ.setdefault("LIBGL_ALWAYS_SOFTWARE", "1")
os.environ.setdefault("GSK_RENDERER", "cairo")

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR   = os.path.dirname(SCRIPT_DIR)
HTML_PATH  = os.path.join(ROOT_DIR, "ui", "desktop", "index.html")
os.environ["PATH"] = f"{os.path.join(ROOT_DIR, 'bin')}:{SCRIPT_DIR}:{os.environ.get('PATH', '')}"

# ── Universal IPC JavaScript ──────────────────────────────────────────────────
IPC_JS = r"""
(function() {
  if (window.__sigmaos_ipc_ready) return;
  window.__sigmaos_ipc_ready = true;

  // Send command to backend (WebKit2 native or HTTP REST)
  window.sigmaExec = function(cmd) {
    if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.sigmaExec) {
      window.webkit.messageHandlers.sigmaExec.postMessage(cmd);
    } else if (window.location.protocol.startsWith('http')) {
      fetch('/api/exec', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ cmd: cmd })
      })
      .then(function(r) { return r.json(); })
      .then(function(d) {
        if (d && d.output && window.sigmaOutput) {
          window.sigmaOutput(d.output);
        }
        if (d && d.persona && typeof window.setPersona === 'function') {
          window.setPersona(d.persona);
        }
      })
      .catch(function(err) {
        if (window.sigmaOutput) {
          window.sigmaOutput('[IPC Error] ' + err.message + '\n');
        }
      });
    } else {
      console.log('[IPC offline fallback]', cmd);
    }
  };

  // Python calls this to deliver output back to terminal
  window.sigmaOutput = function(output) {
    const out = document.getElementById('term-out');
    if (out) {
      out.textContent += output;
      out.scrollTop = out.scrollHeight;
    }
    const oout = document.getElementById('oterm-out');
    if (oout && window.__sigma_oterm_active) {
      oout.textContent += output;
      oout.scrollTop = oout.scrollHeight;
    }
    const docOut = document.getElementById('doctor-live-out');
    if (docOut && window.__sigma_doctor_active) {
      docOut.textContent += output;
      docOut.scrollTop = docOut.scrollHeight;
    }
  };

  // Connect interactive terminal keys to real Python backend
  window.handleTermKey = function(e) {
    if (e.key === 'Enter') {
      const input = document.getElementById('term-in');
      if (!input) return;
      const cmd = input.value.trim();
      input.value = '';
      if (!cmd) return;
      const out = document.getElementById('term-out');
      if (out) { out.textContent += 'sigmaos\u276f ' + cmd + '\n'; out.scrollTop = out.scrollHeight; }
      if (cmd === 'clear') {
        if (out) out.textContent = '';
        return;
      }
      window.sigmaExec(cmd);
    }
  };

  window.handleOtermKey = function(e) {
    if (e.key === 'Enter') {
      const input = document.getElementById('oterm-in');
      if (!input) return;
      const cmd = input.value.trim();
      input.value = '';
      if (!cmd) return;
      window.__sigma_oterm_active = true;
      const out = document.getElementById('oterm-out');
      if (out) { out.textContent += 'omarchy\u276f ' + cmd + '\n'; out.scrollTop = out.scrollHeight; }
      if (cmd === 'clear') {
        if (out) out.textContent = '';
        return;
      }
      window.sigmaExec('omarchy ' + cmd);
    }
  };

  console.log('[SigmaOS IPC] Universal bridge active.');
})();
"""

# ── Shared Command Execution Engine ───────────────────────────────────────────
def execute_real_cmd(real_cmd):
    """Execute real command with Omarchy / SigmaOS routing and return string output."""
    real_cmd = real_cmd.strip()
    if not real_cmd:
        return ""
    if real_cmd in ("clear", "cls"):
        return ""

    if real_cmd.startswith("theme ") or real_cmd.startswith("omarchy theme ") or real_cmd.startswith("wallust "):
        parts = real_cmd.split()
        arg = parts[-1].lower()
        theme_script = os.path.join(ROOT_DIR, "config", "omarchy", "scripts", "apply_theme.py")
        if arg in ("list", "theme"):
            real_cmd = f"python3 {theme_script} list 2>&1"
        else:
            real_cmd = f"python3 {theme_script} {arg} 2>&1"
    elif real_cmd in ("help",):
        real_cmd = 'sigmaos --help 2>&1 || echo "sigmaos: type any bash command"'
    elif real_cmd in ("doctor", "omarchy doctor", "sigmaos doctor", "sigmaos --doctor"):
        real_cmd = f"bash {SCRIPT_DIR}/sigmaos_doctor.sh 2>&1"
    elif real_cmd in ("version", "sigmaos --version", "sigmaos-version"):
        real_cmd = f"{ROOT_DIR}/bin/sigmaos-version 2>&1"
    elif real_cmd in ("neofetch", "fastfetch", "sigmafetch", "fetch"):
        real_cmd = f"{ROOT_DIR}/bin/sigmafetch 2>&1"
    elif real_cmd in ("uname -a", "uname"):
        real_cmd = "uname -a"
    elif real_cmd in ("whoami", "hostname", "pwd", "id"):
        pass
    elif real_cmd.startswith("pacman ") or real_cmd.startswith("sigpkg "):
        sigpkg = os.path.join(ROOT_DIR, "bin", "sigpkg")
        if real_cmd.startswith("pacman "):
            real_cmd = f"{sigpkg} pacman {real_cmd[7:]} 2>&1"
        else:
            real_cmd = f"{sigpkg} {real_cmd[7:]} 2>&1"
    elif real_cmd in ("ls", "ls -la", "ls -l", "ls -a"):
        real_cmd = f"{real_cmd} --color=never 2>&1"

    try:
        out = subprocess.check_output(
            real_cmd, shell=True, stderr=subprocess.STDOUT,
            text=True, timeout=30, cwd=ROOT_DIR
        )
    except subprocess.CalledProcessError as e:
        out = e.output or "(exit non-zero)\n"
    except subprocess.TimeoutExpired:
        out = "(command timed out after 30s)\n"
    except Exception as ex:
        out = f"Error: {ex}\n"
    return out


# ── System Telemetry Metrics ─────────────────────────────────────────────────
def get_telemetry_stats():
    """Calculate and return live CPU % and Memory usage."""
    try:
        with open("/proc/stat") as f:
            fields = [float(x) for x in f.readline().strip().split()[1:]]
        idle = fields[3] + fields[4]
        total = sum(fields)
        cpu_pct = 0.0
        if hasattr(get_telemetry_stats, "prev_total") and hasattr(get_telemetry_stats, "prev_idle"):
            diff_idle = idle - get_telemetry_stats.prev_idle
            diff_total = total - get_telemetry_stats.prev_total
            if diff_total > 0:
                cpu_pct = 100.0 * (1.0 - diff_idle / diff_total)
        get_telemetry_stats.prev_idle = idle
        get_telemetry_stats.prev_total = total

        mem = {}
        with open("/proc/meminfo") as f:
            for line in f:
                parts = line.split(":")
                if len(parts) == 2:
                    mem[parts[0].strip()] = int(parts[1].split()[0])
        total_kb = mem.get("MemTotal", 1)
        avail_kb = mem.get("MemAvailable", 0)
        used_gb = (total_kb - avail_kb) / (1024 * 1024)
        return {"cpu": f"{cpu_pct:.1f}%", "mem": f"{used_gb:.1f} GB"}
    except Exception:
        return {"cpu": "1.0%", "mem": "4.5 GB"}


# ── Python WebKit2 IPC Handler ───────────────────────────────────────────────
def handle_exec(webview, cmd):
    """Run cmd in background thread and inject output back into WebKit2."""
    def _run():
        import gi
        gi.require_version("GLib", "2.0")
        from gi.repository import GLib

        real_cmd = cmd.strip()
        if real_cmd in ("clear", "cls"):
            GLib.idle_add(_inject, webview, "document.getElementById('term-out').textContent = '';")
            return

        if real_cmd.startswith("persona "):
            parts = real_cmd.split()
            if len(parts) > 1:
                target_p = parts[1].lower()
                GLib.idle_add(_inject, webview, f"window.setPersona && window.setPersona('{target_p}');")
                out = f"[Persona] Switched desktop layout and theme to '{target_p.upper()}'.\n"
                escaped = json.dumps(out)
                GLib.idle_add(_inject, webview, f"window.sigmaOutput && window.sigmaOutput({escaped});")
                return

        if real_cmd.startswith("theme ") or real_cmd.startswith("omarchy theme ") or real_cmd.startswith("wallust "):
            parts = real_cmd.split()
            arg = parts[-1].lower()
            mapped = {
                "catppuccin-mocha": "omarchy",
                "catppuccin": "omarchy",
                "tokyo-night": "tokyo",
                "tokyo": "tokyo",
                "nord": "nord",
                "gruvbox-dark": "gruvbox",
                "gruvbox": "gruvbox",
                "cyber-slate": "cyber",
                "cyber": "cyber",
                "mint": "mint",
                "arch": "arch",
            }.get(arg, None)
            if mapped:
                GLib.idle_add(_inject, webview, f"window.setPersona && window.setPersona('{mapped}');")

        out = execute_real_cmd(real_cmd)
        escaped = json.dumps(out)
        js = f"window.sigmaOutput && window.sigmaOutput({escaped});"
        GLib.idle_add(_inject, webview, js)

    threading.Thread(target=_run, daemon=True).start()


def _inject(webview, js):
    """Inject JS into the webview using evaluate_javascript with run_javascript fallback."""
    try:
        webview.evaluate_javascript(js, -1, None, None, None, None, None)
    except Exception:
        try:
            webview.run_javascript(js, None, None, None)
        except Exception:
            pass
    return False


def start_telemetry(webview):
    """Background thread pushing live CPU & memory metrics to WebKit2 view every 2s."""
    def _loop():
        import gi
        gi.require_version("GLib", "2.0")
        from gi.repository import GLib
        while True:
            try:
                stats = get_telemetry_stats()
                js = f"window.sigmaUpdateStats && window.sigmaUpdateStats({json.dumps(stats['cpu'])}, {json.dumps(stats['mem'])});"
                GLib.idle_add(_inject, webview, js)
            except Exception:
                pass
            time.sleep(2.0)

    threading.Thread(target=_loop, daemon=True).start()


# ── Web Desktop HTTP Server Bridge ───────────────────────────────────────────
class SigmaOSWebHandler(http.server.SimpleHTTPRequestHandler):
    """Universal HTTP + REST server for SigmaOS Web Desktop."""
    def __init__(self, *args, **kwargs):
        desktop_dir = os.path.join(ROOT_DIR, "ui", "desktop")
        super().__init__(*args, directory=desktop_dir, **kwargs)

    def do_POST(self):
        if self.path == "/api/exec":
            length = int(self.headers.get("content-length", 0))
            body = self.rfile.read(length).decode("utf-8")
            target_p = None
            try:
                data = json.loads(body)
                cmd = data.get("cmd", "")
                if cmd.startswith("persona "):
                    target_p = cmd.split()[1].lower()
                elif cmd.startswith("theme ") or cmd.startswith("wallust "):
                    arg = cmd.split()[-1].lower()
                    target_p = {
                        "catppuccin-mocha": "omarchy",
                        "catppuccin": "omarchy",
                        "tokyo-night": "tokyo",
                        "tokyo": "tokyo",
                        "nord": "nord",
                        "gruvbox-dark": "gruvbox",
                        "gruvbox": "gruvbox",
                        "cyber-slate": "cyber",
                        "cyber": "cyber",
                        "mint": "mint",
                        "arch": "arch",
                    }.get(arg, None)
                out = execute_real_cmd(cmd)
            except Exception as e:
                out = f"Error: {e}\n"
            resp_data = {"output": out}
            if target_p:
                resp_data["persona"] = target_p
            resp = json.dumps(resp_data).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(resp)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()
            self.wfile.write(resp)
        else:
            self.send_error(404)

    def do_GET(self):
        parsed = urlparse(self.path)
        if parsed.path == "/api/telemetry":
            stats = get_telemetry_stats()
            resp = json.dumps(stats).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(resp)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()
            self.wfile.write(resp)
        elif parsed.path in ("/", "/index.html"):
            self.path = "/index.html"
            super().do_GET()
        else:
            super().do_GET()

    def log_message(self, format, *args):
        pass  # Suppress verbose HTTP server access logs


def launch_web_server(persona="omarchy", port=0, auto_open=True):
    """Launch local HTTP web server and open the browser to the desktop."""
    if port == 0:
        # Test default port 8888 first
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            if s.connect_ex(("127.0.0.1", 8888)) != 0:
                port = 8888
            else:
                s.bind(("127.0.0.1", 0))
                port = s.getsockname()[1]

    class ThreadedServer(socketserver.ThreadingMixIn, http.server.HTTPServer):
        daemon_threads = True

    try:
        httpd = ThreadedServer(("127.0.0.1", port), SigmaOSWebHandler)
    except Exception as e:
        print(f"[!] Could not bind web server on port {port}: {e}")
        return False

    url = f"http://127.0.0.1:{port}/#persona={persona}"
    print("╔══════════════════════════════════════════════════════════╗")
    print(f"║   SigmaOS Sovereign Web Desktop Active                   ║")
    print(f"║   URL: {url:<49} ║")
    print("║   Full GUI desktop accessible from any browser!          ║")
    print("╚══════════════════════════════════════════════════════════╝")

    t = threading.Thread(target=httpd.serve_forever, daemon=True)
    t.start()

    if auto_open:
        chrome = shutil.which("google-chrome") or shutil.which("chromium")
        if chrome:
            try:
                subprocess.Popen([
                    chrome,
                    f"--user-data-dir=/tmp/sigmaos-chrome-profile",
                    f"--app={url}",
                    "--window-size=1280,820",
                    "--window-position=100,60",
                    "--no-first-run",
                    "--no-default-browser-check",
                ])
                print("==> Opened SigmaOS Desktop in dedicated Chrome application window.")
            except Exception:
                webbrowser.open(url)
        else:
            webbrowser.open(url)

    print("==> Server running. Press Ctrl+C to stop.")
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n==> Stopping SigmaOS Web Desktop server.")
        httpd.shutdown()
    return True


# ── Modality 1: GTK3 + WebKit2 Desktop Window ─────────────────────────────────
def launch_gtk_webkit(html_uri, persona="omarchy"):
    try:
        import gi
        gi.require_version("Gtk", "3.0")
        gi.require_version("WebKit2", "4.1")
        from gi.repository import Gtk, WebKit2, GLib
    except Exception as e:
        print(f"[!] GTK/WebKit2 initialization error: {e}")
        return False

    try:
        ucm = WebKit2.UserContentManager()
        ucm.register_script_message_handler("sigmaExec")

        settings = WebKit2.Settings()
        settings.set_enable_javascript(True)
        settings.set_enable_developer_extras(True)
        settings.set_allow_file_access_from_file_urls(True)
        settings.set_allow_universal_access_from_file_urls(True)
        settings.set_hardware_acceleration_policy(WebKit2.HardwareAccelerationPolicy.NEVER)

        webview = WebKit2.WebView.new_with_user_content_manager(ucm)
        webview.set_settings(settings)

        load_succeeded = [False]

        def on_load_changed(wv, event):
            if event == WebKit2.LoadEvent.FINISHED:
                load_succeeded[0] = True
                _inject(wv, IPC_JS)
                persona_js = (
                    f"setTimeout(function(){{ "
                    f"if (window.setPersona) window.setPersona('{persona}'); "
                    f"window.summonWindow && window.summonWindow('win-term'); "
                    f"var inp = document.getElementById('term-in'); "
                    f"if (inp) {{ inp.focus(); }} "
                    f"}}, 350);"
                )
                _inject(wv, persona_js)
                start_telemetry(wv)
                print(f"==> SigmaOS IPC bridge injected, persona '{persona}' active, and live telemetry started.")

        def on_load_failed(wv, load_event, failing_uri, error):
            print(f"[!] WebKit load failed for {failing_uri}: {error.message}")
            return False

        def on_web_process_terminated(wv, reason):
            print(f"[!] WebKit WebProcess terminated (reason={reason}). Falling back...")
            GLib.idle_add(Gtk.main_quit)
            return True

        webview.connect("load-changed", on_load_changed)
        webview.connect("load-failed", on_load_failed)
        try:
            webview.connect("web-process-terminated", on_web_process_terminated)
        except Exception:
            try:
                webview.connect("web-process-crashed", lambda wv: on_web_process_terminated(wv, "crashed"))
            except Exception:
                pass

        def on_script_message(ucm, result):
            try:
                val = result.get_js_value()
                cmd = val.to_string()
                handle_exec(webview, cmd)
            except Exception as ex:
                print(f"[IPC] message parse error: {ex}")

        ucm.connect("script-message-received::sigmaExec", on_script_message)

        GLib.set_prgname("SigmaOS")
        GLib.set_application_name("SigmaOS")

        window = Gtk.Window(title=f"SigmaOS — Sovereign Desktop ({persona.capitalize()} Edition)")
        window.set_default_size(1280, 820)
        window.set_position(Gtk.WindowPosition.CENTER)

        icon_paths = [
            os.path.expanduser("~/.local/share/icons/hicolor/scalable/apps/sigmaos.svg"),
            os.path.join(ROOT_DIR, "assets", "sigmaos.svg"),
        ]
        for ip in icon_paths:
            if os.path.isfile(ip):
                try:
                    window.set_icon_from_file(ip)
                    break
                except Exception:
                    pass

        window.add(webview)
        window.connect("destroy", Gtk.main_quit)
        window.show_all()
        window.present()

        webview.load_uri(html_uri)

        print(f"==> SigmaOS Sovereign Desktop active (GTK3 + WebKit2 + IPC bridge + {persona} persona)")
        print("==> Close the window to exit.")
        Gtk.main()
        return load_succeeded[0]

    except Exception as e:
        print(f"[!] GTK window error: {e}")
        return False


# ── Modality 2: Pure Native GTK3 Desktop App ─────────────────────────────────
def launch_native_gtk_app():
    native = os.path.join(SCRIPT_DIR, "sigmaos_native_app.py")
    if os.path.isfile(native):
        print("==> Launching SigmaOS Native GTK3 Desktop...")
        try:
            res = subprocess.run([sys.executable, native])
            return res.returncode == 0
        except Exception as e:
            print(f"[!] Native app error: {e}")
    return False


# ── Modality 3: Dedicated Browser App Mode ────────────────────────────────────
def launch_chrome_fallback(html_uri):
    chrome = shutil.which("google-chrome") or shutil.which("chromium")
    if chrome:
        profile = "/tmp/sigmaos-chrome-profile"
        print(f"==> Launching via {chrome} --app mode...")
        try:
            subprocess.Popen([
                chrome,
                f"--user-data-dir={profile}",
                f"--app={html_uri}",
                "--window-size=1280,820",
                "--window-position=100,60",
                "--no-first-run",
                "--no-default-browser-check",
            ]).wait()
            return True
        except Exception as e:
            print(f"[!] Chrome error: {e}")
    return False


def launch_firefox_fallback(html_uri):
    ff = shutil.which("firefox")
    if ff:
        print(f"==> Launching via {ff}...")
        try:
            subprocess.Popen([ff, "--new-window", html_uri]).wait()
            return True
        except Exception as e:
            print(f"[!] Firefox error: {e}")
    return False


# ── Main Entrypoint & CLI Parser ──────────────────────────────────────────────
def main():
    import argparse
    parser = argparse.ArgumentParser(description="SigmaOS Sovereign Graphical Desktop")
    parser.add_argument("profile", nargs="?", default=None,
                        help="Optional positional profile name (e.g. desktop, sovereign, catppuccin)")
    parser.add_argument("--persona", "-p", default=None,
                        help="Initial desktop persona/theme to activate")
    parser.add_argument("--native", "-n", action="store_true",
                        help="Force pure GTK3 native desktop app (no WebKit2 requirement)")
    parser.add_argument("--web", "--server", "-w", action="store_true",
                        help="Launch local HTTP web server and open desktop in browser")
    parser.add_argument("--browser", "-b", action="store_true",
                        help="Launch standalone browser application window")
    parser.add_argument("--port", type=int, default=0,
                        help="Port for local web server (default: 8888 or auto)")
    args, _ = parser.parse_known_args()
    raw_persona = args.persona or args.profile or "sovereign"
    if raw_persona in ("desktop", "default", "omarchy"):
        persona = "sovereign"
    else:
        persona = raw_persona

    if not os.path.isfile(HTML_PATH):
        print(f"Error: UI not found at {HTML_PATH}", file=sys.stderr)
        sys.exit(1)

    html_uri = f"file://{HTML_PATH}#persona={persona}"

    # Explicit flag handling
    if args.native:
        if launch_native_gtk_app():
            sys.exit(0)
        sys.exit(1)

    if args.web:
        if launch_web_server(persona=persona, port=args.port):
            sys.exit(0)
        sys.exit(1)

    if args.browser:
        # Launch web server + chrome app for best browser experience
        if launch_web_server(persona=persona, port=args.port):
            sys.exit(0)
        if launch_chrome_fallback(html_uri):
            sys.exit(0)
        sys.exit(1)

    # Display server check for GUI mode
    if not os.environ.get("DISPLAY") and not os.environ.get("WAYLAND_DISPLAY"):
        print("[!] No DISPLAY or WAYLAND_DISPLAY detected.")
        print("==> Starting SigmaOS Sovereign Web Desktop Server instead...")
        if launch_web_server(persona=persona, port=args.port, auto_open=False):
            sys.exit(0)
        print("[!] Reverting to terminal REPL.")
        repl = os.path.join(ROOT_DIR, "bin", "sigmaos")
        if os.path.isfile(repl):
            os.execv(repl, [repl])
        sys.exit(1)

    print("╔══════════════════════════════════════════════════════════╗")
    print(f"║      SigmaOS Sovereign Desktop — {persona.capitalize():<12} Edition        ║")
    print("║   GTK3 + WebKit2 + Real Shell IPC Bridge                ║")
    print("╚══════════════════════════════════════════════════════════╝")

    # Resilient fallback chain:
    # 1. WebKit2 GTK3 window
    if launch_gtk_webkit(html_uri, persona=persona):
        sys.exit(0)

    # 2. Pure GTK3 native desktop app
    print("==> Falling back to SigmaOS Native GTK3 app...")
    if launch_native_gtk_app():
        sys.exit(0)

    # 3. Web server with browser
    print("==> Falling back to SigmaOS Web Desktop Server...")
    if launch_web_server(persona=persona, port=args.port):
        sys.exit(0)

    # 4. Chrome / Firefox direct file fallback
    if launch_chrome_fallback(html_uri):
        sys.exit(0)
    if launch_firefox_fallback(html_uri):
        sys.exit(0)

    # 5. Terminal REPL fallback
    print("[!] No graphical backend succeeded. Reverting to terminal REPL.")
    repl = os.path.join(ROOT_DIR, "bin", "sigmaos")
    if os.path.isfile(repl):
        os.execv(repl, [repl])
    sys.exit(1)


if __name__ == "__main__":
    main()
