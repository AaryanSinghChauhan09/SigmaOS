# Generated method: PeriodicTable._build_ui
import tkinter as tk
from tkinter import messagebox

class PeriodicTable:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], pady=20)
        head.pack(fill='x')
        tk.Label(head, text='NCERT PERIODIC TABLE OF ELEMENTS', font=('Segoe UI Bold', 20), fg=PAL['accent'], bg=PAL['bg']).pack()
        tk.Label(head, text='Click any element for forensic chemical data', font=('Segoe UI', 10), fg=PAL['text'], bg=PAL['bg']).pack()
        container = tk.Frame(self, bg=PAL['bg'])
        container.pack(padx=20, pady=20, expand=True)
        for z, sym, name, mass, group, period, cat in ELEMENTS:
            color = PAL.get(cat, PAL['element'])
            cell = tk.Frame(container, bg=color, width=58, height=68, highlightthickness=1, highlightbackground='#30363D')
            cell.grid(row=period, column=group, padx=2, pady=2)
            cell.pack_propagate(False)
            l_click = lambda e, zid=z: self._show_details(zid)
            cell.bind('<Button-1>', l_click)
            tk.Label(cell, text=str(z), font=('Consolas', 7), fg='white', bg=color).pack(anchor='nw', padx=2)
            tk.Label(cell, text=sym, font=('Segoe UI Bold', 12), fg='white', bg=color).pack()
            tk.Label(cell, text=name[:6], font=('Segoe UI', 7), fg='white', bg=color).pack()