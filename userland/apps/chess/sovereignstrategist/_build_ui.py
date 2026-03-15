"""
Auto-split from userland\apps\chess.py — SovereignStrategist._build_ui
"""

import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys



class SovereignStrategist:
    def _build_ui(self):
        self.configure(bg=PAL['bg'])
        head = tk.Frame(self, bg=PAL['bg'], padx=40, pady=30)
        head.pack(fill='x')
        tk.Label(head, text='STRATEGIST PRO', font=('Inter', 24, 'bold'), fg=PAL['primary'], bg=PAL['bg']).pack(side='left')
        self.score_lbl = tk.Label(head, text='+0.00', font=('JetBrains Mono', 12, 'bold'), fg=PAL['accent'], bg=PAL['bg'])
        self.score_lbl.pack(side='right', padx=20)
        body = tk.Frame(self, bg=PAL['bg'], padx=40)
        body.pack(fill='both', expand=True)
        board_container = tk.Frame(body, bg=PAL['border'], padx=2, pady=2)
        board_container.pack(side='left')
        self.cells: List[List[tk.Button]] = []
        for r in range(8):
            row_btns = []
            for c in range(8):
                bg = PAL['sq_light'] if (r + c) % 2 == 0 else PAL['sq_dark']
                btn = tk.Button(board_container, text='', width=2, height=1, font=('Inter', 42), bg=bg, activebackground=PAL['accent'], relief='flat', borderwidth=0, highlightthickness=0, command=lambda r=r, c=c: self._handle_click(r, c))
                btn.grid(row=r, column=c)
                row_btns.append(btn)
            self.cells.append(row_btns)
        self.panel = tk.Frame(body, bg=PAL['panel'], width=300, padx=25, pady=25)
        self.panel.pack(side='right', fill='y', padx=(30, 0))
        self.panel.pack_propagate(False)
        tk.Label(self.panel, text='NEURAL ANALYSIS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.analysis_bar = ttk.Progressbar(self.panel, style='TProgressbar', length=250, mode='determinate')
        self.analysis_bar.pack(pady=15)
        self.analysis_bar['value'] = 50
        self.hist_txt = tk.Text(self.panel, bg='#000', fg=PAL['text'], font=('JetBrains Mono', 9), borderwidth=0, padx=10, pady=10)
        self.hist_txt.pack(fill='both', expand=True, pady=10)
        ctrl = tk.Frame(self.panel, bg=PAL['panel'])
        ctrl.pack(fill='x')
        tk.Button(ctrl, text='GET HINT', bg=PAL['accent'], fg='white', command=self._get_hint).pack(fill='x', pady=5)
        tk.Button(ctrl, text='RESET', bg=PAL['border'], fg='white', command=self._reset).pack(fill='x', pady=5)
        self.status = tk.Label(self, text='', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
        self._draw_board()
