"""
Auto-split from userland\apps\theme_engine.py — SovereignThemeEngine._build_fx_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random



class SovereignThemeEngine:
    def _build_fx_tab(self):
        tk.Label(self.tab_fx, text='COMPOSITOR EFFECTS ENGINE (picom/kwin usurp)', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        effects = [('Blur Radius', 0, 30, 12), ('Window Opacity (Inactive)', 50, 100, 90), ('Animation Speed (ms)', 50, 500, 200), ('Shadow Spread', 0, 30, 8)]
        for label, frm, to, default in effects:
            f = tk.Frame(self.tab_fx, bg=PAL['panel'], padx=20, pady=15)
            f.pack(fill='x', pady=6)
            tk.Label(f, text=label, font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel'], width=35, anchor='w').pack(side='left')
            scale = ttk.Scale(f, from_=frm, to=to, orient='horizontal', style='Theme.TScale')
            scale.set(default)
            scale.pack(side='left', fill='x', expand=True, padx=15)
            val_lbl = tk.Label(f, text=str(default), font=('Consolas', 10, 'bold'), fg=PAL['accent'], bg=PAL['panel'], width=5)
            val_lbl.pack(side='right')
            scale.config(command=lambda v, l=val_lbl: l.config(text=f'{float(v):.0f}'))
        tk.Label(self.tab_fx, text='SPECIAL EFFECTS TOGGLES', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(20, 10))
        toggles_fr = tk.Frame(self.tab_fx, bg=PAL['bg'])
        toggles_fr.pack(fill='x')
        for txt in ['Background Blur', 'Window Fade-In', 'Dual-Kawase Blur', 'Rounded Corners (12px)']:
            var = tk.BooleanVar(value=True)
            cb = tk.Checkbutton(toggles_fr, text=txt, variable=var, bg=PAL['bg'], fg=PAL['text'], selectcolor=PAL['panel'], font=('Inter', 9), activebackground=PAL['bg'], activeforeground=PAL['accent'])
            cb.pack(side='left', padx=15)
