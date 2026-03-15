"""
Auto-split from userland\apps\logic_simulator.py — LogicSimulator._build_truth
"""

import tkinter as tk
from tkinter import ttk



class LogicSimulator:
    def _build_truth(self, nb):
        tab = tk.Frame(nb, bg=PAL['bg'])
        nb.add(tab, text='  📊 Truth Tables  ')
        tk.Label(tab, text='Full Truth Tables — All Standard Gates', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 10)).pack(pady=10)
        canvas = tk.Canvas(tab, bg=PAL['bg'], highlightthickness=0)
        canvas.pack(fill='both', expand=True, padx=16)
        sb = ttk.Scrollbar(tab, orient='vertical', command=canvas.yview)
        sb.pack(side='right', fill='y')
        canvas.configure(yscrollcommand=sb.set)
        frame = tk.Frame(canvas, bg=PAL['bg'])
        canvas.create_window((0, 0), window=frame, anchor='nw')
        col = 0
        for gate, fn in GATES.items():
            gf = tk.Frame(frame, bg=PAL['card'], padx=12, pady=10, highlightthickness=1, highlightbackground=PAL['accent'])
            gf.grid(row=0, column=col, padx=8, pady=8, sticky='n')
            col += 1
            tk.Label(gf, text=gate, fg=PAL['accent'], bg=PAL['card'], font=('Segoe UI Bold', 12)).grid(row=0, column=0, columnspan=4)
            heads = ['A', 'B', 'OUT'] if gate != 'NOT' else ['A', 'OUT']
            for ci, h in enumerate(heads):
                tk.Label(gf, text=h, fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI Bold', 9), width=4).grid(row=1, column=ci)
            row = 2
            for a in (0, 1):
                for b in (0, 1):
                    out = int(fn(bool(a), bool(b)))
                    vals = [str(a), str(b), str(out)] if gate != 'NOT' else [str(a), str(out)]
                    for ci, v in enumerate(vals):
                        c2 = PAL['on'] if v == '1' else PAL['off']
                        tk.Label(gf, text=v, fg=c2, bg=PAL['card'], font=('Cascadia Code', 11), width=4).grid(row=row, column=ci)
                    row += 1
                    if gate == 'NOT':
                        break
        frame.update_idletasks()
        canvas.configure(scrollregion=canvas.bbox('all'))
