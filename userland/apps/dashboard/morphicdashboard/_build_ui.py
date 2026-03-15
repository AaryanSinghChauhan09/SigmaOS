"""
Auto-split from userland\apps\dashboard.py — MorphicDashboard._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import sys, os, time, random
from typing import Dict, Any, List, Optional



class MorphicDashboard:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['background'], height=80, padx=40)
        head.pack(side='top', fill='x', pady=20)
        tk.Label(head, text='MORPHIC DASHBOARD', font=FONT['h1'], fg=PAL['primary'], bg=PAL['background']).pack(side='left')
        self.time_lbl = tk.Label(head, text='00:00:00', font=('JetBrains Mono', 14), fg=PAL['primary'], bg=PAL['background'])
        self.time_lbl.pack(side='right')
        self.main_grid = tk.Frame(self, bg=PAL['background'], padx=40)
        self.main_grid.pack(fill='both', expand=True)
        self.cpu_icon = self._create_card(self.main_grid, 'HARDWARE CPU', 'cpu_val', 0, 0, icon_key='hal')
        self.ram_icon = self._create_card(self.main_grid, 'HARDWARE RAM', 'ram_val', 0, 1, icon_key='memory')
        self.xp_icon = self._create_card(self.main_grid, 'ACADEMIC XP', 'xp_val', 1, 0, icon_key='ncert')
        self.ai_icon = self._create_card(self.main_grid, 'AI FLEET STATUS', 'ai_val', 1, 1, icon_key='intelligence')
        self.tabs = ttk.Notebook(self.main_grid)
        self.tabs.grid(row=0, column=2, rowspan=2, sticky='nsew', padx=10, pady=10)
        self.main_grid.grid_columnconfigure(2, weight=1)
        if SovereignStrategist:
            ch_fr = tk.Frame(self.tabs, bg=PAL['background'], pady=30)
            self.tabs.add(ch_fr, text=' ♟️ CHESS ')
            tk.Button(ch_fr, text='LAUNCH SOVEREIGN CHESS', bg=PAL['accent'], fg='white', command=lambda: SovereignStrategist().mainloop()).pack(expand=True)
        if LudoApp:
            ld_fr = tk.Frame(self.tabs, bg=PAL['background'], pady=30)
            self.tabs.add(ld_fr, text=f" {ICONS.get('board_hub', '🎲')} LUDO ")
            tk.Button(ld_fr, text='LAUNCH DETERMINISTIC LUDO', bg=PAL['accent'], fg='white', command=lambda: LudoApp().mainloop()).pack(expand=True)
        nexus_fr = tk.Frame(self.tabs, bg=PAL['background'], padx=20, pady=20)
        self.tabs.add(nexus_fr, text=f" {ICONS.get('nexus', '📡')} TOOL NEXUS ")
        tools = [('fs', 'SigmaFS'), ('perf', 'Boost'), ('shield', 'AuraShield'), ('crusher', 'Crusher'), ('automator', 'Automation'), ('portal', 'Transparency'), ('ghostchat', 'GhostChat'), ('ncert', 'Virtual Lab'), ('studio', 'SigmaStudio')]
        for i, (key, name) in enumerate(tools):
            r, c = divmod(i, 3)
            btn = tk.Button(nexus_fr, text=f"{ICONS.get(key, '🔹')}\n{name}", font=('Inter', 9, 'bold'), bg=PAL['surface'], fg='white', relief='flat', width=12, height=4, highlightthickness=1, highlightbackground=PAL['border'])
            btn.grid(row=r, column=c, padx=5, pady=5)
        ctrl = tk.Frame(self, bg=PAL['surface'], height=100, padx=40)
        ctrl.pack(side='bottom', fill='x')
        tk.Label(ctrl, text='SYSTEM VIBE', font=('Inter Bold', 10), fg=PAL['primary'], bg=PAL['surface']).pack(side='left')
        for v in ['DEEP_SPACE', 'APEX_GOLD', 'ZEN_FOCUS', 'GAMING_NEON']:
            tk.Button(ctrl, text=v, bg='#1A1D23', fg='white', relief='flat', padx=15, pady=8, command=lambda v=v: self._switch_vibe(v)).pack(side='left', padx=10)
