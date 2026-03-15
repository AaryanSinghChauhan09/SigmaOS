"""
Auto-split from userland\apps\stopwatch_timer.py — SigmaStopwatch._cd_tick
"""

import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any



class SigmaStopwatch:
    def _cd_tick(self):
        if not self._cd_running:
            return
        if self._cd_remaining <= 0:
            self._cd_running = False
            self._cd_disp.config(text='00:00:00', fg=PAL['danger'])
            self._cd_status.config(text="⏰ TIME'S UP!")
            return
        rem = int(self._cd_remaining)
        h = rem // 3600
        m = rem % 3600 // 60
        s = rem % 60
        self._cd_disp.config(text=f'{h:02}:{m:02}:{s:02}')
        pct = self._cd_remaining / self._cd_total if self._cd_total else 1.0
        self._cd_disp.config(fg=PAL['danger'] if pct < 0.2 else PAL['warn'] if pct < 0.5 else PAL['success'])
        self._cd_remaining -= 1
        self.after(1000, self._cd_tick)
