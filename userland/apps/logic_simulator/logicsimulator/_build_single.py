"""
Auto-split from userland\apps\logic_simulator.py — LogicSimulator._build_single
"""

import tkinter as tk
from tkinter import ttk



class LogicSimulator:
    def _build_single(self, nb):
        tab = tk.Frame(nb, bg=PAL['bg'])
        nb.add(tab, text='  🔌 Single Gate  ')
        inp = tk.Frame(tab, bg=PAL['bg'])
        inp.pack(pady=20)
        for lbl, var in (('Input A', self._A), ('Input B', self._B)):
            fr = tk.Frame(inp, bg=PAL['bg'])
            fr.pack(side='left', padx=20)
            tk.Label(fr, text=lbl, fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 10)).pack()
            tk.Button(fr, textvariable=tk.StringVar(value='1' if var.get() else '0'), bg=PAL['on'] if var.get() else PAL['off'], fg='white', font=('Segoe UI Bold', 14), relief='flat', width=4, command=lambda v=var: self._toggle(v)).pack(pady=4)
        gf = tk.Frame(tab, bg=PAL['bg'])
        gf.pack(pady=10)
        tk.Label(gf, text='Select Gate:', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 10)).pack(side='left', padx=8)
        self._gate_var = tk.StringVar(value='AND')
        for g in GATES:
            rb = tk.Radiobutton(gf, text=g, variable=self._gate_var, value=g, fg=PAL['text'], bg=PAL['bg'], selectcolor=PAL['card'], activebackground=PAL['bg'], font=('Segoe UI', 9), command=self._evaluate)
            rb.pack(side='left', padx=4)
        out_fr = tk.Frame(tab, bg=PAL['card'], padx=30, pady=20, highlightthickness=1, highlightbackground=PAL['accent'])
        out_fr.pack(pady=20, padx=40, fill='x')
        tk.Label(out_fr, text='OUTPUT', fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 9, 'bold')).pack()
        self._out_lbl = tk.Label(out_fr, text='0', fg=PAL['off'], bg=PAL['card'], font=('Cascadia Code', 52))
        self._out_lbl.pack()
        self._out_txt = tk.Label(out_fr, text='LOW', fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 12))
        self._out_txt.pack()
        ctrl = tk.Frame(tab, bg=PAL['bg'])
        ctrl.pack(pady=8)
        self._a_btn = tk.Button(ctrl, text=f'A = {self._A.get()}', bg=PAL['on'] if self._A.get() else PAL['off'], fg='white', font=('Segoe UI Bold', 10), relief='flat', padx=16, pady=6, command=lambda: self._toggle_btn('A'))
        self._a_btn.pack(side='left', padx=8)
        self._b_btn = tk.Button(ctrl, text=f'B = {self._B.get()}', bg=PAL['on'] if self._B.get() else PAL['off'], fg='white', font=('Segoe UI Bold', 10), relief='flat', padx=16, pady=6, command=lambda: self._toggle_btn('B'))
        self._b_btn.pack(side='left', padx=8)
        self._evaluate()
