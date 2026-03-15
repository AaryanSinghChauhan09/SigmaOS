"""
Auto-split from userland\apps\theme_engine.py — SovereignThemeEngine._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random



class SovereignThemeEngine:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='RICE HQ - SOVEREIGN THEME ENGINE', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='✨ APPLY ALL & RELOAD', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=15, pady=8, command=self._apply_all).pack(side='left', padx=5)
        tk.Button(btn_fr, text='💾 EXPORT RICE PROFILE', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._export_profile).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.tabs = ttk.Notebook(self.workspace, style='Theme.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self.tab_colors = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_colors, text='🎨 PALETTE & GTK')
        self._build_color_tab()
        self.tab_icons = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_icons, text='🖱 ICONS & CURSORS')
        self._build_icon_tab()
        self.tab_fx = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_fx, text='💫 COMPOSITOR FX')
        self._build_fx_tab()
        self.tab_fonts = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_fonts, text='🔤 FONTS & TYPOGRAPHY')
        self._build_font_tab()
        self.status = tk.Label(self, text='RICE ENGINE IDLE | GTK3/4 & QT5/6 UNIFIED THEMING ACTIVE', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
