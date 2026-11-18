#!/usr/bin/env python3
"""
SigmaOS System Tray / Status Indicator
========================================
Provides a GTK3 StatusIcon that sits in the system tray (or AppIndicator),
showing SigmaOS status and offering a right-click menu for quick actions.

Inspired by Omarchy's minimal panel design and Arch Linux's efficient tooling.
"""
import sys
import os
import subprocess
import threading
import gi
gi.require_version("Gtk", "3.0")
gi.require_version("Gdk", "3.0")
from gi.repository import Gtk, Gdk, GLib

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(SCRIPT_DIR)

def run_cmd_async(cmd, callback=None):
    """Run a shell command asynchronously, optionally calling callback(output)."""
    def _run():
        try:
            out = subprocess.check_output(cmd, shell=True, stderr=subprocess.STDOUT, text=True, timeout=15)
        except subprocess.CalledProcessError as e:
            out = e.output or "(no output)"
        except Exception as e:
            out = f"Error: {e}"
        if callback:
            GLib.idle_add(callback, out)
    threading.Thread(target=_run, daemon=True).start()

class SigmaOSTray:
    """GTK3 system tray icon with SigmaOS quick-launch menu."""

    def __init__(self):
        # Use a standard GTK stock icon (network-transmit-receive) as placeholder
        self.icon = Gtk.StatusIcon()
        self.icon.set_from_stock(Gtk.STOCK_EXECUTE)
        self.icon.set_tooltip_text("SigmaOS Sovereign OS — Click to open")
        self.icon.set_visible(True)
        self.icon.connect("activate", self.on_activate)
        self.icon.connect("popup-menu", self.on_popup)
        self._build_menu()
        self._output_window = None

    def _build_menu(self):
        self.menu = Gtk.Menu()

        def _add_item(label, callback, separator_before=False):
            if separator_before:
                self.menu.append(Gtk.SeparatorMenuItem())
            item = Gtk.MenuItem(label=label)
            item.connect("activate", callback)
            self.menu.append(item)

        _add_item("󰣇 Open SigmaOS Desktop",  lambda w: self._run_gui())
        _add_item("⬛ Open Terminal REPL",   lambda w: self._run_repl())
        _add_item("󰄛 Omarchy Persona",       lambda w: self._show_output("sigmaos --omarchy --non-interactive || echo 'done'"), separator_before=True)
        _add_item("󰣇 Arch Persona",          lambda w: self._show_output("sigmaos --arch --non-interactive || echo 'done'"))
        _add_item("󰮯 Pacman Update (dry)",   lambda w: self._show_output("sigpkg update 2>&1 | head -40"), separator_before=True)
        _add_item("󰚰 AUR Search: hyprland",  lambda w: self._show_output("sigpkg aur hyprland 2>&1 | head -20"))
        _add_item("󰀻 Doctor / Health Check", lambda w: self._show_output("sigmaos --doctor 2>&1"), separator_before=True)
        _add_item("󰒋 System Status",         lambda w: self._show_output("sigmaos --status 2>&1"))
        _add_item("⚙  About SigmaOS",         lambda w: self._show_about(), separator_before=True)
        _add_item("✕  Quit Tray",             lambda w: Gtk.main_quit(), separator_before=True)
        self.menu.show_all()

    def on_activate(self, icon):
        self._run_gui()

    def on_popup(self, icon, button, time):
        self.menu.popup(None, None, Gtk.StatusIcon.position_menu, icon, button, time)

    def _run_gui(self):
        subprocess.Popen([sys.executable, os.path.join(SCRIPT_DIR, "sigmaos_gui.py")])

    def _run_repl(self):
        for term in ["gnome-terminal", "xterm", "x-terminal-emulator"]:
            if os.path.exists(f"/usr/bin/{term}"):
                subprocess.Popen([term, "--", "sigmaos"])
                return

    def _show_output(self, cmd):
        win = Gtk.Window(title=f"SigmaOS: {cmd[:40]}")
        win.set_default_size(700, 420)
        win.set_position(Gtk.WindowPosition.CENTER)

        box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
        win.add(box)

        sw = Gtk.ScrolledWindow()
        sw.set_hexpand(True)
        sw.set_vexpand(True)
        tv = Gtk.TextView()
        tv.set_editable(False)
        tv.override_background_color(Gtk.StateFlags.NORMAL, Gdk.RGBA(0.07, 0.07, 0.11, 1))
        tv.override_color(Gtk.StateFlags.NORMAL, Gdk.RGBA(0.80, 0.84, 0.95, 1))
        buf = tv.get_buffer()
        buf.set_text(f"$ {cmd}\n\n(running...)\n")
        sw.add(tv)
        box.pack_start(sw, True, True, 0)

        btn_close = Gtk.Button(label="Close")
        btn_close.connect("clicked", lambda w: win.destroy())
        box.pack_start(btn_close, False, False, 4)

        win.show_all()

        def _on_done(output):
            buf.set_text(f"$ {cmd}\n\n{output}")
        run_cmd_async(cmd, _on_done)

    def _show_about(self):
        dialog = Gtk.AboutDialog()
        dialog.set_program_name("SigmaOS")
        dialog.set_version("v3.6-sovereign")
        dialog.set_comments(
            "Sovereign Operating System\n"
            "Arch Linux ALPM substrate · Omarchy desktop aesthetic\n"
            "Built with Rust, Python 3, GTK3 & WebKit2"
        )
        dialog.set_website("https://github.com/AaryanSinghChauhan09/SigmaOS")
        dialog.run()
        dialog.destroy()


def main():
    tray = SigmaOSTray()
    # Show a brief notification on start
    GLib.timeout_add(500, lambda: (
        tray.icon.set_tooltip_text("SigmaOS Sovereign OS — Running ✓"),
        False
    )[-1])
    Gtk.main()

if __name__ == "__main__":
    main()
