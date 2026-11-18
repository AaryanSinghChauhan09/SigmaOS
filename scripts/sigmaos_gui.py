#!/usr/bin/env python3
"""
SigmaOS Sovereign Graphical Desktop Window Runner
==================================================
Launches the native graphical desktop for SigmaOS using GTK3 + WebKit2,
with automatic fallback to Chrome/Firefox app mode if GTK/WebKit display fails.

Principles:
- READY TO USE: Instant visible GUI on DISPLAY=:0
- LEAST ASTONISHMENT: True desktop window, interactive controls, and persona switches
- DEFENSIVE PROGRAMMING: Graceful fallback chain (WebKit2 -> Google Chrome -> Firefox)
"""

import sys
import os
import subprocess
import shutil

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(SCRIPT_DIR)
HTML_PATH = os.path.join(ROOT_DIR, "ui", "desktop", "index.html")

def launch_gtk_webkit(html_uri):
    """Native: launch GTK3 + WebKit2 Window."""
    try:
        import gi
        gi.require_version("Gtk", "3.0")
        gi.require_version("WebKit2", "4.1")
        from gi.repository import Gtk, WebKit2, GLib
    except Exception as e:
        print(f"[!] GTK/WebKit2 initialization error: {e}")
        return False

    try:
        window = Gtk.Window(title="󰣇 SigmaOS Sovereign Desktop (Arch & Omarchy Edition)")
        window.set_default_size(1280, 820)
        window.set_position(Gtk.WindowPosition.CENTER)

        # WebKit WebView settings
        settings = WebKit2.Settings()
        settings.set_enable_javascript(True)
        settings.set_enable_developer_extras(True)
        settings.set_enable_webgl(True)
        settings.set_allow_file_access_from_file_urls(True)
        settings.set_allow_universal_access_from_file_urls(True)

        webview = WebKit2.WebView.new_with_settings(settings)
        webview.load_uri(html_uri)

        window.add(webview)
        window.connect("destroy", Gtk.main_quit)
        window.show_all()

        print("==> SigmaOS Sovereign Desktop Environment Window Active.")
        print("==> Running on native GTK3 + WebKit2 4.1 engine. Close window to exit.")
        Gtk.main()
        return True
    except Exception as e:
        print(f"[!] Native GTK window launch encountered: {e}")
        return False

def launch_native_gtk_app():
    """Fallback: launch native pure GTK3 desktop interface."""
    native_script = os.path.join(SCRIPT_DIR, "sigmaos_native_app.py")
    if os.path.isfile(native_script):
        print("==> Launching SigmaOS Native GTK3 Desktop Interface...")
        try:
            res = subprocess.run([sys.executable, native_script])
            return res.returncode == 0
        except Exception as e:
            print(f"[!] Native GTK3 app error: {e}")
            return False
    return False

def launch_chrome_fallback(html_uri):
    """Fallback: launch in dedicated standalone Chrome application window."""
    chrome = shutil.which("google-chrome") or shutil.which("chromium")
    if chrome:
        profile_dir = "/tmp/sigmaos-chrome-profile"
        print(f"==> Launching SigmaOS Desktop in dedicated app window via {chrome}...")
        try:
            proc = subprocess.Popen([
                chrome,
                f"--user-data-dir={profile_dir}",
                f"--app={html_uri}",
                "--window-size=1280,820",
                "--window-position=100,60",
                "--no-first-run",
                "--no-default-browser-check"
            ])
            proc.wait()
            return True
        except Exception as e:
            print(f"[!] Chrome app launch error: {e}")
            return False
    return False

def launch_firefox_fallback(html_uri):
    """Fallback: launch in Firefox."""
    firefox = shutil.which("firefox")
    if firefox:
        print(f"==> Launching SigmaOS Desktop via {firefox}...")
        try:
            proc = subprocess.Popen([firefox, "--new-window", html_uri])
            proc.wait()
            return True
        except Exception as e:
            print(f"[!] Firefox launch error: {e}")
            return False
    return False

def main():
    if not os.path.isfile(HTML_PATH):
        print(f"Error: UI desktop file not found at {HTML_PATH}", file=sys.stderr)
        sys.exit(1)

    html_uri = f"file://{HTML_PATH}"

    # Check for graphical DISPLAY
    if not os.environ.get("DISPLAY") and not os.environ.get("WAYLAND_DISPLAY"):
        print("[!] No DISPLAY or WAYLAND_DISPLAY detected. Falling back to terminal workstation...")
        os.execv(os.path.join(ROOT_DIR, "bin", "sigmaos"), ["sigmaos"])

    print("╔══════════════════════════════════════════════════════════╗")
    print("║        Launching SigmaOS Sovereign GUI Desktop           ║")
    print("║     Omarchy Aesthetic & Arch Linux Substrate Studio     ║")
    print("╚══════════════════════════════════════════════════════════╝")

    # 1. Native WebKit2 GTK3 window (FIRST PRIORITY - true native OS window)
    if launch_gtk_webkit(html_uri):
        sys.exit(0)

    # 2. Native Pure GTK3 desktop app
    if launch_native_gtk_app():
        sys.exit(0)

    # 3. Dedicated Chrome App Window
    if launch_chrome_fallback(html_uri):
        sys.exit(0)

    # 4. Firefox fallback
    if launch_firefox_fallback(html_uri):
        sys.exit(0)

    print("[!] Unable to open graphical window. Reverting to terminal mode...")
    os.execv(os.path.join(ROOT_DIR, "bin", "sigmaos"), ["sigmaos"])

if __name__ == "__main__":
    main()
