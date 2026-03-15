# Generated method: NCERTGrapher._build_ui
import tkinter as tk
from tkinter import ttk
import math

class NCERTGrapher:
    def _build_ui(self):
        hdr = tk.Frame(self, bg='#11142A', height=60)
        hdr.pack(fill='x')
        tk.Label(hdr, text='📈 SCIENTIFIC GRAPHING UTILITY', fg='#3B82F6', bg='#11142A', font=('Segoe UI Bold', 14)).pack(pady=15)
        ctrl = tk.Frame(self, bg='#11142A', pady=10)
        ctrl.pack(fill='x')
        tk.Label(ctrl, text='f(x) = ', fg='white', bg='#11142A', font=('Consolas', 12)).pack(side='left', padx=(20, 0))
        ent = tk.Entry(ctrl, textvariable=self.func_var, bg='#1A1E30', fg='white', font=('Consolas', 12), width=30, relief='flat', insertbackground='white')
        ent.pack(side='left', padx=10)
        ent.bind('<Return>', lambda e: self.plot())
        tk.Label(ctrl, text='Range ±', fg='white', bg='#11142A').pack(side='left', padx=10)
        tk.Scale(ctrl, from_=1, to=100, variable=self.range_var, orient='horizontal', command=lambda x: self.plot(), bg='#11142A', fg='white', highlightthickness=0).pack(side='left', padx=5)
        tk.Button(ctrl, text='PLOT GRAPH', command=self.plot, bg='#6C63FF', fg='white', relief='flat', padx=15).pack(side='left', padx=20)
        self.canvas.destroy()
        self.canvas = tk.Canvas(self, bg='#00050A', highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, padx=20, pady=20)