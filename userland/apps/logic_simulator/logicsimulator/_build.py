"""
Auto-split from userland\apps\logic_simulator.py — LogicSimulator._build
"""

import tkinter as tk
from tkinter import ttk



class LogicSimulator:
    def _build(self):
        hdr = tk.Frame(self, bg=PAL['panel'], height=50)
        hdr.pack(fill='x')
        hdr.pack_propagate(False)
        tk.Label(hdr, text='⚡ LOGIC GATE SIMULATOR', fg=PAL['accent'], bg=PAL['panel'], font=('Segoe UI Bold', 14)).pack(side='left', padx=18, pady=10)
        nb = ttk.Notebook(self)
        nb.pack(fill='both', expand=True, padx=10, pady=10)
        self._build_single(nb)
        self._build_truth(nb)
        self._build_adder(nb)
        self._build_combinational(nb)
