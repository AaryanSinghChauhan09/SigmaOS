# Generated method: SovereignThemeEngine._build_font_tab
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random

class SovereignThemeEngine:
    def _build_font_tab(self):
        tk.Label(self.tab_fonts, text='SYSTEM TYPOGRAPHY MATRIX', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        font_slots = [('Interface Font', 'Inter', 'System UI, App labels, menus'), ('Monospace / Terminal', 'JetBrains Mono', 'Code editors, terminal emulators'), ('Document Font', 'Noto Serif', 'Reading, PDFs, long-form text'), ('Icon Font (Ligatures)', 'Nerd Font Symbols', 'Status bars, WM decorations')]
        for role, font, desc in font_slots:
            f = tk.Frame(self.tab_fonts, bg=PAL['panel'], padx=20, pady=15)
            f.pack(fill='x', pady=6)
            lf = tk.Frame(f, bg=PAL['panel'])
            lf.pack(side='left', fill='x', expand=True)
            tk.Label(lf, text=role, font=('Inter', 9, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(lf, text=font, font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(lf, text=desc, font=('Inter', 8), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
            scale_f = tk.Frame(f, bg=PAL['panel'])
            scale_f.pack(side='right')
            tk.Label(scale_f, text='Size:', font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(side='left')
            s = ttk.Scale(scale_f, from_=8, to=24, orient='horizontal', style='Theme.TScale')
            s.set(12)
            s.pack(side='left', padx=5)