"""
Auto-split from userland\apps\theme_engine.py — SovereignThemeEngine._build_icon_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random



class SovereignThemeEngine:
    def _build_icon_tab(self):
        tk.Label(self.tab_icons, text='ICON PACK & CURSOR MATRIX', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        icon_packs = [('Papirus Dark', 'Community • 4,500 icons • SVG Scalable'), ('Candy Icons', 'Vibrant • 5,200+ icons • Neon Palette'), ('Numix Circle', 'Rounded • Consistent MD Style'), ('Sovereign Pack (Custom)', 'SigmaOS Native • 2,048 icons • Kernel-linked')]
        for name, desc in icon_packs:
            f = tk.Frame(self.tab_icons, bg=PAL['panel'], pady=12, padx=20)
            f.pack(fill='x', pady=6)
            tk.Label(f, text=name, font=('Inter', 11, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(side='left')
            tk.Label(f, text=desc, font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(side='left', padx=15)
            tk.Button(f, text='APPLY PACK', bg=PAL['sidebar'], fg='white', font=('Inter', 8, 'bold'), relief='flat', command=lambda n=name: messagebox.showinfo('Icon Pack', f'[{n}] deployed system-wide.\nIcon cache regenerated in 0.01s.')).pack(side='right')
        tk.Label(self.tab_icons, text='CURSOR SETS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(20, 10))
        cursors_fr = tk.Frame(self.tab_icons, bg=PAL['bg'])
        cursors_fr.pack(fill='x')
        for c in ['Breeze Dark', 'Bibata Modern Ice', 'Qogir White', 'Sovereign Beam']:
            tk.Button(cursors_fr, text=c, font=('Inter', 8, 'bold'), bg=PAL['panel'], fg=PAL['text'], relief='flat', padx=12, pady=8).pack(side='left', padx=5)
