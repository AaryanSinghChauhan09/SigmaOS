# Generated method: SigmaStopwatch._build_pomodoro
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _build_pomodoro(self, nb):
        tab = tk.Frame(nb, bg=PAL['bg'])
        nb.add(tab, text='  🍅 Pomodoro  ')
        self._pomo_disp = tk.Label(tab, text='25:00', fg=PAL['danger'], bg=PAL['bg'], font=('Cascadia Code', 56))
        self._pomo_disp.pack(pady=24)
        self._pomo_state_lbl = tk.Label(tab, text='FOCUS SESSION', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 11))
        self._pomo_state_lbl.pack()
        btn_fr = tk.Frame(tab, bg=PAL['bg'])
        btn_fr.pack(pady=16)
        self._pomo_btn = tk.Button(btn_fr, text=f"{ICONS.get('perf', '▶')} START", bg=PAL['danger'], fg='white', font=('Segoe UI Bold', 11), relief='flat', padx=22, pady=10, command=self._pomo_toggle)
        self._pomo_btn.pack(side='left', padx=6)
        tk.Button(btn_fr, text=f"{ICONS.get('code', '⏭')} SKIP", bg=PAL['card'], fg=PAL['text'], font=('Segoe UI Bold', 11), relief='flat', padx=22, pady=10, command=self._pomo_skip).pack(side='left', padx=6)
        cfg_fr = tk.Frame(tab, bg=PAL['card'], padx=16, pady=12)
        cfg_fr.pack(fill='x', padx=20, pady=16)
        tk.Label(cfg_fr, text=f"{ICONS.get('hal', '⚙️')} Settings", fg=PAL['accent'], bg=PAL['card'], font=('Segoe UI Bold', 10)).pack(anchor='w')
        self._work_min = self._mini_spin(cfg_fr, f"{ICONS.get('zero_trust', '🎯')} Work (min)", 25)
        self._break_min = self._mini_spin(cfg_fr, '☕ Break (min)', 5)
        self._long_break = self._mini_spin(cfg_fr, '🌿 Long break (min)', 15)
        self._pomo_lbl = tk.Label(tab, text='Sessions: 0', fg=PAL['success'], bg=PAL['bg'], font=('Segoe UI', 9))
        self._pomo_lbl.pack()