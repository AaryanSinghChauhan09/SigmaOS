# Generated method: SovereignArcade._init_xo
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

class SovereignArcade:
    def _init_xo(self, parent):
        self.xo_status = tk.Label(parent, text="X's STRATEGIC VECTOR", font=('Inter', 12, 'bold'), fg=PAL['text'], bg=PAL['bg'])
        self.xo_status.pack(pady=20)
        grid = tk.Frame(parent, bg=PAL['bg'])
        grid.pack()
        for i in range(9):
            btn = tk.Button(grid, text='', font=('Inter', 32, 'bold'), width=4, height=1, bg=PAL['sidebar'], fg='white', command=lambda idx=i: self._xo_move(idx))
            btn.grid(row=i // 3, column=i % 3, padx=5, pady=5)
            self.xo_btns.append(btn)