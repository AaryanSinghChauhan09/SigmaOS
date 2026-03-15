"""
Auto-split from userland\apps\energy_core.py — EnergyCore._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
import time
import random
from userland.system_api.sigma_std import SigmaSys



class EnergyCore:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='ENERGY CORE APEX', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        nav_btns = [('⚡ OPTIMIZE', self._optimize_power), ('💤 DEEP SLEEP', self._hibernate)]
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.viz_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=300, padx=20, pady=20)
        self.viz_fr.pack(side='left', fill='y', padx=(0, 20))
        self.viz_fr.pack_propagate(False)
        tk.Label(self.viz_fr, text='CAPACITY MATRIX', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 20))
        self.bat_percent_lbl = tk.Label(self.viz_fr, text='--%', font=('Inter', 48, 'bold'), fg=PAL['accent'], bg=PAL['panel'])
        self.bat_percent_lbl.pack(expand=True)
        self.bat_status_lbl = tk.Label(self.viz_fr, text='ANALYZING CELL...', font=('Inter', 10), fg=PAL['text'], bg=PAL['panel'])
        self.bat_status_lbl.pack(pady=10)
        self.time_rem_lbl = tk.Label(self.viz_fr, text='--:-- REMAINING', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel'])
        self.time_rem_lbl.pack(pady=5)
        self.data_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.data_fr.pack(side='left', fill='both', expand=True)
        grid = tk.Frame(self.data_fr, bg=PAL['bg'])
        grid.pack(fill='x', pady=(0, 20))
        self.health_card = self._create_metric_card(grid, 'CELL HEALTH', 'PEAK', PAL['accent'])
        self.health_card.grid(row=0, column=0, sticky='nsew', padx=5, pady=5)
        self.cycle_card = self._create_metric_card(grid, 'CHARGE CYCLES', '142', PAL['text'])
        self.cycle_card.grid(row=0, column=1, sticky='nsew', padx=5, pady=5)
        self.temp_card = self._create_metric_card(grid, 'THERMALS', '32°C', PAL['text'])
        self.temp_card.grid(row=0, column=2, sticky='nsew', padx=5, pady=5)
        grid.columnconfigure((0, 1, 2), weight=1)
        graph_fr = tk.Frame(self.data_fr, bg=PAL['panel'], padx=20, pady=20)
        graph_fr.pack(fill='both', expand=True, padx=5, pady=5)
        tk.Label(graph_fr, text='NEURAL DISCHARGE PREDICTION', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.canvas = tk.Canvas(graph_fr, bg=PAL['panel'], highlightthickness=0, height=120)
        self.canvas.pack(fill='both', expand=True, pady=15)
        self._draw_graph()
        self.status = tk.Label(self, text='SOVEREIGN ENERGY CORE | MONITORING POWER GRID', bg=PAL['sidebar'], fg=PAL['text'], font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
