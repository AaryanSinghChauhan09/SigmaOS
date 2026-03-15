"""
Auto-split from userland\apps\sigma_calculator.py — SigmaCalculator._build
"""

import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List



class SigmaCalculator:
    def _build(self):
        hdr = tk.Frame(self, bg=PAL['panel'], height=50)
        hdr.pack(fill='x')
        hdr.pack_propagate(False)
        tk.Label(hdr, text=f"{ICONS.get('calculator', '🧮')} SIGMA CALCULATOR", fg=PAL['accent'], bg=PAL['panel'], font=('Segoe UI Bold', 13)).pack(side='left', padx=18, pady=10)
        for m in ('DEC', 'HEX', 'BIN', 'OCT'):
            rb = tk.Radiobutton(hdr, text=m, variable=self._mode, value=m, fg=PAL['dim'], bg=PAL['panel'], selectcolor=PAL['card'], activebackground=PAL['panel'], font=('Segoe UI', 8), command=self._mode_changed)
            rb.pack(side='right', padx=4)
        disp_fr = tk.Frame(self, bg=PAL['card'], highlightthickness=1, highlightbackground=PAL['border'])
        disp_fr.pack(fill='x', padx=12, pady=8)
        self._hist_lbl = tk.Label(disp_fr, text='', fg=PAL['dim'], bg=PAL['card'], font=('Cascadia Code', 9), anchor='e')
        self._hist_lbl.pack(fill='x', padx=12, pady=(8, 0))
        self._disp = tk.Label(disp_fr, text='0', fg=PAL['text'], bg=PAL['card'], font=('Cascadia Code', 26), anchor='e')
        self._disp.pack(fill='x', padx=12, pady=(0, 8))
        self._mem_lbl = tk.Label(disp_fr, text='M: 0', fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 8), anchor='w')
        self._mem_lbl.pack(fill='x', padx=12, pady=(0, 6))
        grid = tk.Frame(self, bg=PAL['bg'])
        grid.pack(fill='both', expand=True, padx=12, pady=(0, 12))
        ROWS = [[('sin', 'fn'), ('cos', 'fn'), ('tan', 'fn'), ('log', 'fn'), ('ln', 'fn')], [('x²', 'fn'), ('√', 'fn'), ('π', 'fn'), ('e', 'fn'), ('C', 'clr')], [('MC', 'mem'), ('MR', 'mem'), ('M+', 'mem'), ('M-', 'mem'), ('MS', 'mem')], [('7', 'num'), ('8', 'num'), ('9', 'num'), ('÷', 'op'), ('(', 'op')], [('4', 'num'), ('5', 'num'), ('6', 'num'), ('×', 'op'), (')', 'op')], [('1', 'num'), ('2', 'num'), ('3', 'num'), ('−', 'op'), ('%', 'op')], [('0', 'num'), ('.', 'num'), ('±', 'fn'), ('+', 'op'), ('=', 'eq')]]
        COLOR_MAP = {'num': PAL['btn'], 'op': '#252848', 'fn': '#1C2040', 'clr': PAL['danger'], 'eq': PAL['accent'], 'mem': '#1A2535'}
        for r, row in enumerate(ROWS):
            for c, (label, typ) in enumerate(row):
                bg = COLOR_MAP.get(typ, PAL['btn'])
                btn = tk.Button(grid, text=label, bg=bg, fg=PAL['text'], font=('Segoe UI', 12), relief='flat', command=lambda l=label: self._press(l))
                btn.grid(row=r, column=c, padx=3, pady=3, sticky='nsew', ipady=10)
                btn.bind('<Enter>', lambda e, b=btn: b.config(bg=PAL['btnH']))
                btn.bind('<Leave>', lambda e, b=btn, c=bg: b.config(bg=c))
        for i in range(5):
            grid.columnconfigure(i, weight=1)
        for i in range(7):
            grid.rowconfigure(i, weight=1)
        hpanel = tk.Frame(self, bg=PAL['panel'], height=90)
        hpanel.pack(fill='x', padx=12, pady=(0, 8))
        hpanel.pack_propagate(False)
        tk.Label(hpanel, text='HISTORY', fg=PAL['dim'], bg=PAL['panel'], font=('Segoe UI', 8, 'bold')).pack(anchor='w', padx=10, pady=(6, 2))
        self._hist_box = tk.Text(hpanel, bg=PAL['panel'], fg=PAL['dim'], font=('Cascadia Code', 8), borderwidth=0, height=3)
        self._hist_box.pack(fill='x', padx=10)
        self._refresh_history()
