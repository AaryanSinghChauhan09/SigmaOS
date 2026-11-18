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
import gi
gi.require_version("Gtk", "3.0")
gi.require_version("Gdk", "3.0")
from gi.repository import Gtk, Gdk, GLib

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

        btn_status = Gtk.Button(label="󰒋 Verify System Status")
        btn_status.connect("clicked", self.on_run_cmd, "sigmaos --status")
        btn_box.pack_start(btn_status, False, False, 0)

        main_box.pack_start(btn_box, False, False, 0)

        # ── Terminal Output View ──────────────────────────────────────
        scroll = Gtk.ScrolledWindow()
        scroll.set_hexpand(True)
        scroll.set_vexpand(True)
        scroll.set_margin_start(16)
        scroll.set_margin_end(16)
        scroll.set_margin_bottom(16)

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
            "Click any of the action buttons above or run commands from your host shell:\n"
            "  • sigmaos --omarchy   : Summon the Omarchy Wayland aesthetic layer\n"
            "  • sigmaos --arch      : Switch to minimalist Arch Linux rolling substrate\n"
            "  • sigpkg aur <query>  : Query Arch User Repository (AUR v5)\n"
            "  • omarchy theme list  : Explore dynamic Wallust color harmonies\n"
            "────────────────────────────────────────────────────────────────────────────────\n"
        )
        self.buffer.set_text(welcome_text)
        scroll.add(self.text_view)
        main_box.pack_start(scroll, True, True, 0)

    def on_run_cmd(self, widget, cmd):
        self.append_text(f"\n$ {cmd}\n")
        try:
            output = subprocess.check_output(cmd, shell=True, stderr=subprocess.STDOUT, text=True)
            self.append_text(output)
        except subprocess.CalledProcessError as e:
            self.append_text(e.output)

    def append_text(self, text):
        end_iter = self.buffer.get_end_iter()
        self.buffer.insert(end_iter, text)
        self.text_view.scroll_to_iter(self.buffer.get_end_iter(), 0.0, False, 0.0, 0.0)

def main():
    app = SigmaOSWindow()
    app.connect("destroy", Gtk.main_quit)
    app.show_all()
    Gtk.main()

if __name__ == "__main__":
    main()
