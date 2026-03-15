# Generated method: ConfigHubPage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL

class ConfigHubPage:
    def _build_ui(self):
        is_child = self.controller._is_child_mode()
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        s_fr = tk.Frame(body, bg=PAL['bg2'], width=200)
        s_fr.pack(side='left', fill='both', padx=(0, 10))
        s_fr.pack_propagate(False)
        self.c_fr = tk.Frame(body, bg=PAL['bg'])
        self.c_fr.pack(side='left', fill='both', expand=True)
        if is_child:
            cats = ['Safety', 'Info']
        else:
            cats = ['System', 'Display', 'Network', 'Security', 'Safety', 'Sovereignty', 'About']
        for cat in cats:
            btn_text = cat
            tk.Button(s_fr, text=btn_text, font=FONT_MED, bg=PAL['bg2'], fg=PAL['text'], relief='flat', anchor='w', padx=15, command=lambda c=cat: self._show_cfg(c)).pack(fill='x', pady=2)
        start_cat = 'Safety' if is_child else 'About'
        self._show_cfg(start_cat)