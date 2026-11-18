#!/usr/bin/env python3
"""
SigmaOS Sovereign Graphical Desktop
=====================================
GTK3 + WebKit2 window with a REAL Python↔JS IPC bridge.
Every command typed in the HTML terminal is executed by Python and the
real output is injected back into the page — no simulation.

Fallback chain:
  1. GTK3 + WebKit2 with IPC bridge  (best — real OS window + real shell)
  2. Pure GTK3 native app            (sigmaos_native_app.py)
  3. Chrome --app=file://            (browser fallback)
  4. Firefox --new-window            (last resort)
  5. Terminal REPL binary            (no DISPLAY)
"""

import sys, os, subprocess, shutil, threading, json, time

# ── Additional WebKit env vars to prevent blank window ───────────────────────
os.environ.setdefault("WEBKIT_FORCE_SANDBOX", "0")
os.environ.setdefault("WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS", "1")
os.environ.setdefault("WEBKIT_DISABLE_COMPOSITING_MODE", "1")
os.environ.setdefault("LIBGL_ALWAYS_SOFTWARE", "1")
os.environ.setdefault("GSK_RENDERER", "cairo")

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR   = os.path.dirname(SCRIPT_DIR)
HTML_PATH  = os.path.join(ROOT_DIR, "ui", "desktop", "index.html")

# ── IPC JavaScript injected into every page ───────────────────────────────────
IPC_JS = r"""
(function() {
  if (window.__sigmaos_ipc_ready) return;
  window.__sigmaos_ipc_ready = true;

  // Send a command to Python backend
  window.sigmaExec = function(cmd) {
    if (window.webkit && window.webkit.messageHandlers && window.webkit.messageHandlers.sigmaExec) {
      window.webkit.messageHandlers.sigmaExec.postMessage(cmd);
    } else {
      console.log('[IPC fallback]', cmd);
    }
  };

  // Python calls this to deliver output back to the terminal
  window.sigmaOutput = function(output) {
    const out = document.getElementById('term-out');
    if (out) {
      out.textContent += output;
      out.scrollTop = out.scrollHeight;
    }
    // Also try the omarchy terminal
    const oout = document.getElementById('oterm-out');
    if (oout && window.__sigma_oterm_active) {
      oout.textContent += output;
      oout.scrollTop = oout.scrollHeight;
    }
    // Also try the doctor live output
    const docOut = document.getElementById('doctor-live-out');
    if (docOut && window.__sigma_doctor_active) {
      docOut.textContent += output;
      docOut.scrollTop = docOut.scrollHeight;
    }
  };

  // Override handleTermKey to route to real Python backend
  const _origHandleTerm = window.handleTermKey;
  window.handleTermKey = function(e) {
    if (e.key === 'Enter') {
      const input = document.getElementById('term-in');
      if (!input) return;
      const cmd = input.value.trim();
      input.value = '';
      if (!cmd) return;
      const out = document.getElementById('term-out');
      if (out) { out.textContent += 'sigmaos\u276f ' + cmd + '\n'; out.scrollTop = out.scrollHeight; }
      window.sigmaExec(cmd);
    }
  };

  // Override Omarchy terminal too
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
      window.sigmaExec('omarchy ' + cmd);
    }
  };

  console.log('[SigmaOS IPC] Bridge active — real shell connected.');
})();
"""

# ── Python IPC handler ────────────────────────────────────────────────────────
def handle_exec(webview, cmd):
    """Run cmd in a background thread and inject output back into the WebView."""
    def _run():
        try:
            # Map some HTML-terminal commands to real equivalents
            real_cmd = cmd.strip()
            if real_cmd in ('clear',):
                _inject(webview, "document.getElementById('term-out').textContent = '';")
                return
            if real_cmd in ('help',):
                real_cmd = 'sigmaos --help 2>&1 || echo "sigmaos: type any bash command"'
            elif real_cmd in ('doctor', 'omarchy doctor', 'sigmaos doctor', 'sigmaos --doctor'):
                real_cmd = f'bash {SCRIPT_DIR}/sigmaos_doctor.sh 2>&1'
            elif real_cmd in ('version', 'sigmaos --version', 'sigmaos-version'):
                real_cmd = f'{ROOT_DIR}/bin/sigmaos-version 2>&1'
            elif real_cmd in ('neofetch', 'fastfetch'):
                real_cmd = f'{real_cmd} 2>&1 || echo "neofetch: not installed (install with: sudo apt install neofetch)"'
            elif real_cmd in ('uname -a', 'uname'):
                real_cmd = 'uname -a'
            elif real_cmd in ('whoami', 'hostname', 'pwd', 'id'):
                pass  # run as-is
            elif real_cmd.startswith('pacman ') or real_cmd.startswith('sigpkg '):
                # Route through sigpkg ALPM bridge
                sigpkg = os.path.join(ROOT_DIR, 'bin', 'sigpkg')
                if real_cmd.startswith('pacman '):
                    real_cmd = f'{sigpkg} pacman {real_cmd[7:]} 2>&1'
                else:
                    real_cmd = f'{sigpkg} {real_cmd[7:]} 2>&1'
            elif real_cmd in ('ls', 'ls -la', 'ls -l', 'ls -a'):
                real_cmd = f'{real_cmd} --color=never 2>&1'
            else:
                # Pass all other commands to bash directly (real IPC)
                pass

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
        # Escape for JS string injection
        escaped = json.dumps(out)
        js = f"window.sigmaOutput && window.sigmaOutput({escaped});"
        import gi
        gi.require_version("GLib", "2.0")
        from gi.repository import GLib
        GLib.idle_add(_inject, webview, js)

    threading.Thread(target=_run, daemon=True).start()

def _inject(webview, js):
    """Inject JS into the webview, trying run_javascript first (more stable in GTK3)."""
    try:
        webview.run_javascript(js, None, None, None)
    except Exception:
        try:
            webview.evaluate_javascript(js, -1, None, None, None, None, None)
        except Exception:
            pass
    return False


def start_telemetry(webview):
    """Background thread that pushes live CPU & memory metrics to the UI every 2 seconds."""
    def _loop():
        prev_idle = 0.0
        prev_total = 0.0
        while True:
            try:
                # CPU usage from /proc/stat
                with open("/proc/stat") as f:
                    fields = [float(x) for x in f.readline().strip().split()[1:]]
                idle = fields[3] + fields[4]
                total = sum(fields)
                if prev_total > 0:
                    diff_idle = idle - prev_idle
                    diff_total = total - prev_total
                    cpu_pct = 100.0 * (1.0 - diff_idle / diff_total) if diff_total > 0 else 0.0
                else:
                    cpu_pct = 0.0
                prev_idle, prev_total = idle, total

                # Memory usage from /proc/meminfo
                mem = {}
                with open("/proc/meminfo") as f:
                    for line in f:
                        parts = line.split(":")
                        if len(parts) == 2:
                            mem[parts[0].strip()] = int(parts[1].split()[0])
                total_kb = mem.get("MemTotal", 1)
                avail_kb = mem.get("MemAvailable", 0)
                used_gb = (total_kb - avail_kb) / (1024 * 1024)

                cpu_str = f"{cpu_pct:.1f}%"
                mem_str = f"{used_gb:.1f} GB"

                js = f"window.sigmaUpdateStats && window.sigmaUpdateStats({json.dumps(cpu_str)}, {json.dumps(mem_str)});"
                from gi.repository import GLib
                GLib.idle_add(_inject, webview, js)
            except Exception:
                pass
            time.sleep(2.0)

    threading.Thread(target=_loop, daemon=True).start()


# ── GTK3 + WebKit2 window ─────────────────────────────────────────────────────
def launch_gtk_webkit(html_uri):
    try:
        import gi
        gi.require_version("Gtk", "3.0")
        gi.require_version("WebKit2", "4.1")
        from gi.repository import Gtk, WebKit2, GLib
    except Exception as e:
        print(f"[!] GTK/WebKit2 init error: {e}")
        return False

    try:
        # Disable sandbox (avoids silent subprocess crashes in sandboxed envs)
        # (env vars set at module load above)

        # UserContentManager for JS→Python messages
        ucm = WebKit2.UserContentManager()
        ucm.register_script_message_handler("sigmaExec")

        settings = WebKit2.Settings()
        settings.set_enable_javascript(True)
        settings.set_enable_developer_extras(True)
        settings.set_allow_file_access_from_file_urls(True)
        settings.set_allow_universal_access_from_file_urls(True)
        settings.set_hardware_acceleration_policy(
            WebKit2.HardwareAccelerationPolicy.NEVER
        )

        webview = WebKit2.WebView.new_with_user_content_manager(ucm)
        webview.set_settings(settings)

        # Inject IPC bridge after every page load
        def on_load_changed(wv, event):
            if event == WebKit2.LoadEvent.FINISHED:
                _inject(wv, IPC_JS)
                # Auto-open terminal window on startup and focus command prompt
                _inject(wv, "setTimeout(function(){ window.summonWindow && window.summonWindow('win-term'); var inp = document.getElementById('term-in'); if (inp) { inp.focus(); } }, 400);")
                start_telemetry(wv)
                print("==> SigmaOS IPC bridge injected and live telemetry started.")

        def on_script_message(ucm, result):
            try:
                val = result.get_js_value()
                cmd = val.to_string()
                handle_exec(webview, cmd)
            except Exception as ex:
                print(f"[IPC] message parse error: {ex}")

        webview.connect("load-changed", on_load_changed)
        ucm.connect("script-message-received::sigmaExec", on_script_message)

        GLib.set_prgname("SigmaOS")
        GLib.set_application_name("SigmaOS")

        window = Gtk.Window(title="SigmaOS \u2014 Sovereign Desktop")
        window.set_wmclass("SigmaOS", "SigmaOS")
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

        print("==> SigmaOS Sovereign Desktop active (GTK3 + WebKit2 + IPC bridge)")
        print("==> Close the window to exit.")
        Gtk.main()
        return True

    except Exception as e:
        print(f"[!] GTK window error: {e}")
        return False


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
                "--disable-gpu",
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


def main():
    if not os.path.isfile(HTML_PATH):
        print(f"Error: UI not found at {HTML_PATH}", file=sys.stderr)
        sys.exit(1)

    html_uri = f"file://{HTML_PATH}"

    if not os.environ.get("DISPLAY") and not os.environ.get("WAYLAND_DISPLAY"):
        print("[!] No display. Falling back to terminal REPL.")
        repl = os.path.join(ROOT_DIR, "bin", "sigmaos")
        if os.path.isfile(repl):
            os.execv(repl, [repl])
        sys.exit(1)

    print("╔══════════════════════════════════════════════════════════╗")
    print("║      SigmaOS Sovereign Desktop — Omarchy Edition        ║")
    print("║   GTK3 + WebKit2 + Real Shell IPC Bridge                ║")
    print("╚══════════════════════════════════════════════════════════╝")

    if launch_gtk_webkit(html_uri):
        sys.exit(0)
    if launch_native_gtk_app():
        sys.exit(0)
    if launch_chrome_fallback(html_uri):
        sys.exit(0)
    if launch_firefox_fallback(html_uri):
        sys.exit(0)

    print("[!] No GUI available. Reverting to terminal REPL.")
    repl = os.path.join(ROOT_DIR, "bin", "sigmaos")
    if os.path.isfile(repl):
        os.execv(repl, [repl])
    sys.exit(1)


if __name__ == "__main__":
    main()
