"""
Auto-split from userland\apps\board_hub.py — SovereignArcade._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional



class SovereignArcade:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=30, pady=20)
        head.pack(fill='x')
        tk.Label(head, text='ZENITH ARCADE', font=('Inter', 24, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        self.status_lbl = tk.Label(head, text='SYSTEM STATUS: PROTECTED', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg'])
        self.status_lbl.pack(side='right', pady=12)
        self.tabs = ttk.Notebook(self)
        self.tabs.pack(fill='both', expand=True, padx=25, pady=10)
        xo_fr = tk.Frame(self.tabs, bg=PAL['bg'], pady=30)
        self.tabs.add(xo_fr, text=f" {ICONS.get('board_hub', '⭕')} XO ")
        self._init_xo(xo_fr)
        nx_fr = tk.Frame(self.tabs, bg=PAL['bg'], pady=30)
        self.tabs.add(nx_fr, text=f" {ICONS.get('mesh', '🕸️')} DOTS ")
        self._init_nexus(nx_fr)
        bl_fr = tk.Frame(self.tabs, bg=PAL['bg'], pady=30)
        self.tabs.add(bl_fr, text=f" {ICONS.get('fabric', '🧱')} VOID ")
        self._init_blocks(bl_fr)
        if SovereignStrategist:
            ch_fr = tk.Frame(self.tabs, bg=PAL['bg'], pady=30)
            self.tabs.add(ch_fr, text=f" {ICONS.get('ncert', '♟️')} CHESS ")
            tk.Button(ch_fr, text='LAUNCH SOVEREIGN CHESS', bg=PAL['accent'], fg='white', command=lambda: SovereignStrategist().mainloop() if SovereignStrategist else None, font=('Inter Bold', 10), padx=20, pady=10).pack(expand=True)
        if LudoApp:
            ld_fr = tk.Frame(self.tabs, bg=PAL['bg'], pady=30)
            self.tabs.add(ld_fr, text=f" {ICONS.get('board_hub', '🎲')} LUDO ")
            tk.Button(ld_fr, text='LAUNCH DETERMINISTIC LUDO', bg=PAL['accent'], fg='white', command=lambda: LudoApp().mainloop() if LudoApp else None, font=('Inter Bold', 10), padx=20, pady=10).pack(expand=True)
