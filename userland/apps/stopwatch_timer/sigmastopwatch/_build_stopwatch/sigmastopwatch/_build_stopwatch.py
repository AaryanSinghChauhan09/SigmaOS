# Generated method: SigmaStopwatch._build_stopwatch
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _build_stopwatch(self, nb):
        tab = tk.Frame(nb, bg=PAL['bg'])
        nb.add(tab, text=f"  {ICONS.get('timer', '⏱')} Stopwatch  ")
        self._sw_disp = tk.Label(tab, text='00:00:00.00', fg=PAL['accent'], bg=PAL['bg'], font=('Cascadia Code', 42))
        self._sw_disp.pack(pady=30)
        btn_fr = tk.Frame(tab, bg=PAL['bg'])
        btn_fr.pack()
        self._sw_start_btn = tk.Button(btn_fr, text=f"{ICONS.get('perf', '▶')} START", bg=PAL['success'], fg='white', font=('Segoe UI Bold', 11), relief='flat', padx=22, pady=10, command=self._sw_toggle)
        self._sw_start_btn.pack(side='left', padx=6)
        tk.Button(btn_fr, text='LAP', bg=PAL['card'], fg=PAL['text'], font=('Segoe UI Bold', 11), relief='flat', padx=22, pady=10, command=self._sw_lap).pack(side='left', padx=6)
        tk.Button(btn_fr, text=f"{ICONS.get('minimalist', '↺')} RESET", bg=PAL['danger'], fg='white', font=('Segoe UI Bold', 11), relief='flat', padx=22, pady=10, command=self._sw_reset).pack(side='left', padx=6)
        tk.Label(tab, text='LAP TIMES', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 8, 'bold')).pack(pady=(20, 4))
        self._lap_box = tk.Text(tab, bg=PAL['card'], fg=PAL['text'], font=('Cascadia Code', 9), height=8, borderwidth=0, padx=10, pady=10)
        self._lap_box.pack(fill='x', padx=16)