# Generated method: BuyhatkePage._show_sub
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def _show_sub(self, name):
        for s in self.sub_pages.values():
            s.pack_forget()
        if name not in self.sub_pages:
            p = tk.Frame(self.container, bg=PAL['bg'])
            self.sub_pages[name] = p
            getattr(self, f'_build_{name}')(p)
        self.sub_pages[name].pack(fill='both', expand=True)