# Generated method: UIMixin._card
import tkinter as tk
from tkinter import scrolledtext, messagebox
from .styles import PAL, FONT_MONO, FONT_SMALL, FONT_BOLD

class UIMixin:
    def _card(self, parent, title='', padx=16, pady=12, glass=False) -> tk.Frame:
        bg_col = PAL['bg2'] if glass else PAL['card']
        bord = PAL['accent'] if glass else PAL['border']
        outer = tk.Frame(parent, bg=bord, padx=1, pady=1)
        container = tk.Frame(outer, bg=bg_col, padx=padx, pady=pady)
        container.pack(fill='both', expand=True)
        container.master = outer
        if title:
            hdr = tk.Frame(container, bg=bg_col)
            hdr.pack(fill='x', pady=(0, 10))
            tk.Label(hdr, text=title.upper(), font=('Inter Bold', 8), fg=PAL['dim'] if not glass else PAL['cyan'], bg=bg_col).pack(side='left')
            tk.Frame(container, bg=PAL['bg3'], height=1).pack(fill='x', pady=(0, 15))
        return container