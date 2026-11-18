#!/usr/bin/env python3
"""
SigmaOS Sovereign Desktop Native Experience
===========================================
Pure GTK3 native desktop interface matching Omarchy's aesthetic
and Arch Linux's pacman/system management controls.
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

CSS_DATA = b"""
window {
    background-color: #1e1e2e;
    color: #cdd6f4;
    font-family: 'Inter', 'Noto Sans', sans-serif;
}
#header_bar {
    background-color: #181825;
    border-bottom: 2px solid #cba6f7;
    padding: 10px 16px;
}
#brand_title {
    font-size: 18px;
    font-weight: bold;
    color: #cba6f7;
}
#status_badge {
    color: #a6e3a1;
    font-weight: bold;
    font-size: 12px;
}
button {
    background-color: #313244;
    color: #cdd6f4;
    border: 1px solid #45475a;
    border-radius: 8px;
    padding: 8px 16px;
    font-weight: 500;
}
button:hover {
    background-color: #45475a;
    border-color: #cba6f7;
    color: #f5e0dc;
}
#term_view {
    background-color: #11111b;
    color: #a6adc8;
    font-family: 'JetBrainsMono Nerd Font', 'Monospace', monospace;
    font-size: 13px;
    padding: 12px;
}
"""

class SigmaOSWindow(Gtk.Window):
    def __init__(self):
        super().__init__(title="SigmaOS Sovereign Desktop (Arch & Omarchy Edition)")
        self.set_default_size(1050, 680)
        self.set_position(Gtk.WindowPosition.CENTER)

        # Style provider
        style_provider = Gtk.CssProvider()
        style_provider.load_from_data(CSS_DATA)
        Gtk.StyleContext.add_provider_for_screen(
            Gdk.Screen.get_default(),
            style_provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )

        main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
        self.add(main_box)

        # ── Header / Topbar ───────────────────────────────────────────
        header = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=16)
        header.set_name("header_bar")

        title = Gtk.Label(label="󰣇 SigmaOS Sovereign Workstation")
        title.set_name("brand_title")
        header.pack_start(title, False, False, 0)

        badge = Gtk.Label(label="● Arch ALPM Engine: Online | Omarchy Hyprland: Active")
        badge.set_name("status_badge")
        header.pack_start(badge, False, False, 0)

        main_box.pack_start(header, False, False, 0)

        # ── Action Toolbar ────────────────────────────────────────────
        btn_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        btn_box.set_margin_top(12)
        btn_box.set_margin_bottom(12)
        btn_box.set_margin_start(16)
        btn_box.set_margin_end(16)

        btn_omarchy = Gtk.Button(label="󰄛 Omarchy Persona")
        btn_omarchy.connect("clicked", self.on_run_cmd, "sigmaos --omarchy")
        btn_box.pack_start(btn_omarchy, False, False, 0)

        btn_arch = Gtk.Button(label="󰣇 Arch Persona")
        btn_arch.connect("clicked", self.on_run_cmd, "sigmaos --arch")
        btn_box.pack_start(btn_arch, False, False, 0)

        btn_aur = Gtk.Button(label="󰚰 AUR Hyprland Query")
        btn_aur.connect("clicked", self.on_run_cmd, "sigpkg aur hyprland")
        btn_box.pack_start(btn_aur, False, False, 0)

        btn_theme = Gtk.Button(label="󰏘 Wallust Themes")
        btn_theme.connect("clicked", self.on_run_cmd, "omarchy theme list")
        btn_box.pack_start(btn_theme, False, False, 0)

        btn_pacman = Gtk.Button(label="󰮯 Pacman Update")
        btn_pacman.connect("clicked", self.on_run_cmd, "sigpkg update")
        btn_box.pack_start(btn_pacman, False, False, 0)

        btn_doctor = Gtk.Button(label="󰀻 Omarchy Doctor")
        btn_doctor.connect("clicked", self.on_run_cmd, "omarchy doctor")
        btn_box.pack_start(btn_doctor, False, False, 0)

        btn_status = Gtk.Button(label="󰒋 System Status")
        btn_status.connect("clicked", self.on_run_cmd, "sigmaos --status")
        btn_box.pack_start(btn_status, False, False, 0)

        main_box.pack_start(btn_box, False, False, 0)

        # ── Terminal Output View ──────────────────────────────────────
        scroll = Gtk.ScrolledWindow()
        scroll.set_hexpand(True)
        scroll.set_vexpand(True)
        scroll.set_margin_start(16)
        scroll.set_margin_end(16)
        scroll.set_margin_bottom(8)

        self.text_view = Gtk.TextView()
        self.text_view.set_name("term_view")
        self.text_view.set_editable(False)
        self.text_view.set_cursor_visible(False)
        self.buffer = self.text_view.get_buffer()

        welcome_text = (
            "╔══════════════════════════════════════════════════════════════════════════════╗\n"
            "║                 SIGMAOS SOVEREIGN DESKTOP RUNTIME                            ║\n"
            "║          Powered by Arch Linux ALPM Bridge & Omarchy Desktop Engine          ║\n"
            "╚══════════════════════════════════════════════════════════════════════════════╝\n\n"
            "Welcome! SigmaOS combines the rolling-release freedom of Arch Linux with the\n"
            "meticulous aesthetic and ergonomic polish of Omarchy Linux.\n\n"
            "Click any of the action buttons above or type commands below:\n"
            "  • pacman -Syu         : Simulate full system upgrade via ALPM bridge\n"
            "  • pacman -Ss <pkg>    : Search sovereign & Arch repositories\n"
            "  • sigpkg aur <query>  : Query Arch User Repository (AUR v5)\n"
            "  • omarchy theme list  : Explore dynamic Wallust color harmonies\n"
            "  • omarchy doctor      : Check all Wayland & substrate health metrics\n"
            "────────────────────────────────────────────────────────────────────────────────\n"
        )
        self.buffer.set_text(welcome_text)
        scroll.add(self.text_view)
        main_box.pack_start(scroll, True, True, 0)

        # ── Interactive Command Entry Bar ─────────────────────────────
        cmd_bar = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=8)
        cmd_bar.set_margin_start(16)
        cmd_bar.set_margin_end(16)
        cmd_bar.set_margin_bottom(16)

        prompt_lbl = Gtk.Label(label="󰣇 sigmaos>")
        prompt_lbl.set_name("status_badge")
        cmd_bar.pack_start(prompt_lbl, False, False, 0)

        self.cmd_entry = Gtk.Entry()
        self.cmd_entry.set_placeholder_text("Enter command (e.g., pacman -Ss neovim, uname -a, omarchy doctor)...")
        self.cmd_entry.set_hexpand(True)
        self.cmd_entry.connect("activate", self.on_entry_activate)
        cmd_bar.pack_start(self.cmd_entry, True, True, 0)

        btn_run = Gtk.Button(label="Execute")
        btn_run.connect("clicked", lambda w: self.on_entry_activate(self.cmd_entry))
        cmd_bar.pack_start(btn_run, False, False, 0)

        main_box.pack_start(cmd_bar, False, False, 0)

    def on_entry_activate(self, entry):
        text = entry.get_text().strip()
        if text:
            entry.set_text("")
            self.on_run_cmd(None, text)

    def on_run_cmd(self, widget, cmd):
        self.append_text(f"\n$ {cmd}\n")
        self.append_text("(running...)\n")
        def _run():
            try:
                output = subprocess.check_output(cmd, shell=True, stderr=subprocess.STDOUT, text=True, timeout=30)
            except subprocess.CalledProcessError as e:
                output = e.output or "(exit code non-zero, no output)\n"
            except Exception as e:
                output = f"Error: {e}\n"
            GLib.idle_add(self._finish_cmd_output, output)
        threading.Thread(target=_run, daemon=True).start()

    def _finish_cmd_output(self, output):
        # Remove last "(running...)\n" placeholder then append real output
        end = self.buffer.get_end_iter()
        # Walk back one line
        start = end.copy()
        start.backward_line()
        self.buffer.delete(start, end)
        self.append_text(output)
        return False  # GLib.idle_add one-shot

    def run_diagnostics_async(self):
        """Run startup diagnostics in background and append to terminal view."""
        diag_script = os.path.join(SCRIPT_DIR, "sigmaos_startup_diagnostics.py")
        if os.path.isfile(diag_script):
            self.on_run_cmd(None, f"python3 {diag_script}")

    def append_text(self, text):
        end_iter = self.buffer.get_end_iter()
        self.buffer.insert(end_iter, text)
        self.text_view.scroll_to_iter(self.buffer.get_end_iter(), 0.0, False, 0.0, 0.0)

def main():
    app = SigmaOSWindow()
    app.connect("destroy", Gtk.main_quit)
    app.show_all()
    # Run startup diagnostics after a brief delay (non-blocking)
    GLib.timeout_add(600, lambda: (app.run_diagnostics_async(), False)[1])
    Gtk.main()

if __name__ == "__main__":
    main()
