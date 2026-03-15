# Generated method: SigmaStopwatch._build_countdown
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _build_countdown(self, nb):
        tab = tk.Frame(nb, bg=PAL['bg'])
        nb.add(tab, text=f"  {ICONS.get('snapshots', '⏳')} Countdown  ")
        self._cd_disp = tk.Label(tab, text='00:00:00', fg=PAL['warn'], bg=PAL['bg'], font=('Cascadia Code', 42))
        self._cd_disp.pack(pady=20)
        inp_fr = tk.Frame(tab, bg=PAL['bg'])
        inp_fr.pack(pady=10)
        self._hh = self._spin(inp_fr, 'Hours', 0, 23)
        tk.Label(inp_fr, text=':', fg=PAL['dim'], bg=PAL['bg'], font=('Cascadia Code', 24)).pack(side='left')
        self._mm = self._spin(inp_fr, 'Min', 0, 59)
        tk.Label(inp_fr, text=':', fg=PAL['dim'], bg=PAL['bg'], font=('Cascadia Code', 24)).pack(side='left')
        self._ss = self._spin(inp_fr, 'Sec', 0, 59)
        btn_fr = tk.Frame(tab, bg=PAL['bg'])
        btn_fr.pack(pady=14)
        self._cd_btn = tk.Button(btn_fr, text=f"{ICONS.get('perf', '▶')} START", bg=PAL['warn'], fg='white', font=('Segoe UI Bold', 11), relief='flat', padx=22, pady=10, command=self._cd_toggle)
        self._cd_btn.pack(side='left', padx=6)
        tk.Button(btn_fr, text=f"{ICONS.get('minimalist', '↺')} RESET", bg=PAL['danger'], fg='white', font=('Segoe UI Bold', 11), relief='flat', padx=22, pady=10, command=self._cd_reset).pack(side='left', padx=6)
        self._cd_status = tk.Label(tab, text='Set timer and press START', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 10))
        self._cd_status.pack(pady=8)