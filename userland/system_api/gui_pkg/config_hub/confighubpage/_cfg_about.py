"""
Auto-split from userland\system_api\gui_pkg\config_hub.py — ConfigHubPage._cfg_about
"""

import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL



class ConfigHubPage:
    def _cfg_about(self, parent):
        is_child = self.controller._is_child_mode()
        head = 'SigmaOS for Kids' if is_child else 'SigmaOS Sovereign'
        tk.Label(parent, text=head, font=FONT_LOGO, fg=PAL['cyan'], bg=PAL['bg']).pack(pady=20)
        tk.Label(parent, text=f'Version {self.cfg.VERSION}', font=FONT_MED, fg=PAL['dim'], bg=PAL['bg']).pack()
        info_title = 'Fun Details' if is_child else 'OS Status & Parity Dashboard'
        info = self._card(parent, info_title)
        info.master.pack(fill='x', pady=20)
        grid = tk.Frame(info, bg=PAL['card'])
        grid.pack(fill='x')
        if is_child:
            metrics = [('OS Heart', 'Happy Beats'), ('Fun Level', 'Maximum!'), ('Safety Shield', '🟢 100% SECURE'), ('Learning Points', 'Ready to Grow')]
        else:
            metrics = [('Kernel Type', 'Neural-Predictive'), ('Subsystem', 'Sovereign-Core-v3'), ('Parity Status', '🟢 TITAN LEVEL REACHED'), ('Bridges Active', '4 (Win32, Cocoa, APK, WASM)')]
        for i, (k, v) in enumerate(metrics):
            tk.Label(grid, text=k + ':', font=FONT_BOLD, fg=PAL['dim'], bg=PAL['card']).grid(row=i, column=0, sticky='w', pady=5)
            tk.Label(grid, text=v, font=FONT_BOLD, fg='white', bg=PAL['card']).grid(row=i, column=1, sticky='w', padx=20)
