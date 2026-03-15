"""
Auto-split from userland\apps\theme_engine.py — SovereignThemeEngine._build_color_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random



class SovereignThemeEngine:
    def _build_color_tab(self):
        tk.Label(self.tab_colors, text='SYSTEM COLOR MATRIX', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        colors_fr = tk.Frame(self.tab_colors, bg=PAL['bg'])
        colors_fr.pack(fill='x', pady=(0, 20))
        color_slots = [('Accent Primary', '#BD00FF'), ('Accent Secondary', '#00FFCC'), ('Background Base', '#0B0C0E'), ('Surface Panel', '#1C1E24'), ('Text Foreground', '#F2F2F7'), ('Danger / Alert', '#FF3B30')]
        for i, (name, default_col) in enumerate(color_slots):
            row = i // 3
            col = i % 3
            f = tk.Frame(colors_fr, bg=PAL['panel'], padx=15, pady=15)
            f.grid(row=row, column=col, padx=8, pady=8, sticky='nsew')
            colors_fr.grid_columnconfigure(col, weight=1)
            tk.Label(f, text=name, font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w')
            swatch = tk.Label(f, bg=default_col, width=20, height=3, relief='flat', cursor='hand2')
            swatch.pack(fill='x', pady=8)
            swatch.bind('<Button-1>', lambda e, s=swatch, n=name: self._pick_color(s, n))
            tk.Label(f, text=default_col, font=('Consolas', 9), fg=PAL['dim'], bg=PAL['panel']).pack()
        tk.Label(self.tab_colors, text='PRESET PALETTES (Rice Presets)', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(20, 10))
        presets_fr = tk.Frame(self.tab_colors, bg=PAL['bg'])
        presets_fr.pack(fill='x')
        presets = [('Catppuccin Mocha', '#1e1e2e', '#cdd6f4', '#bd93f9'), ('Dracula Pro', '#282a36', '#f8f8f2', '#ff79c6'), ('TokyoNight', '#1a1b26', '#a9b1d6', '#7aa2f7'), ('Gruvbox Dark', '#282828', '#ebdbb2', '#fabd2f'), ('Sovereign Default', '#0B0C0E', '#F2F2F7', '#BD00FF')]
        for name, bg, fg_col, acc in presets:
            btn = tk.Button(presets_fr, text=name, font=('Inter', 8, 'bold'), bg=acc, fg='black', relief='flat', padx=12, pady=6, command=lambda n=name: self._apply_preset(n))
            btn.pack(side='left', padx=5)
