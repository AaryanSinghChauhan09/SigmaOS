# Generated method: UIMixin._build_page_header
import tkinter as tk
from tkinter import scrolledtext, messagebox
from .styles import PAL, FONT_MONO, FONT_SMALL, FONT_BOLD

class UIMixin:
    def _build_page_header(self, parent, title, subtitle):
        """Standardized Page Header."""
        header = tk.Frame(parent, bg=PAL['bg'], pady=30, padx=20)
        header.pack(fill='x')
        tk.Label(header, text=title.upper(), font=('Inter', 24, 'bold'), fg=PAL['text'], bg=PAL['bg']).pack(anchor='w')
        tk.Label(header, text=subtitle, font=('Inter', 10), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(5, 0))
        tk.Frame(parent, bg=PAL['border'], height=1).pack(fill='x', padx=20)