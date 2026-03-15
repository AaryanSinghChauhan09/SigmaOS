# Generated method: SovereignArcade._init_nexus
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

class SovereignArcade:
    def _init_nexus(self, parent):
        inf = tk.Frame(parent, bg=PAL['bg'])
        inf.pack(fill='x', padx=50)
        self.nx_lbl = tk.Label(inf, text="RED'S TURN", font=('Inter', 12, 'bold'), fg=PAL['p1'], bg=PAL['bg'])
        self.nx_lbl.pack(side='left')
        self.nx_score = tk.Label(inf, text='RED: 0 | BLUE: 0', font=('Inter', 10), fg=PAL['dim'], bg=PAL['bg'])
        self.nx_score.pack(side='right')
        self.nx_canv = tk.Canvas(parent, width=450, height=450, bg='#000')
        self.nx_canv.pack(pady=20)
        self.nx_canv.bind('<Button-1>', self._nexus_click)