# Generated method: PhysiologyHub._setup_circ
import tkinter as tk
from tkinter import ttk, messagebox
import math, random

class PhysiologyHub:
    def _setup_circ(self, master):
        tk.Label(master, text='CARDIAC CYCLE SIMULATOR', font=('Segoe UI Bold', 12), fg=PAL['heart'], bg=PAL['bg']).pack(pady=10)
        self.ecg_canvas = tk.Canvas(master, bg='#000', height=200, highlightthickness=1, highlightbackground=PAL['heart'])
        self.ecg_canvas.pack(fill='x', padx=40, pady=20)
        tk.Button(master, text='GENERATE ECG SCAN', command=self._draw_ecg, bg=PAL['heart'], fg='white', relief='flat', padx=20).pack()
        info = tk.Label(master, text='P-Wave: Atrial Depolarization\nQRS: Ventricular Depolarization\nT-Wave: Ventricular Repolarization', fg=PAL['text'], bg=PAL['bg'], justify='left', pady=20)
        info.pack()