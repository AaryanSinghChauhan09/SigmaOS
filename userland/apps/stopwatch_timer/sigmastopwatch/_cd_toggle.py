"""
Auto-split from userland\apps\stopwatch_timer.py — SigmaStopwatch._cd_toggle
"""

import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any



class SigmaStopwatch:
    def _cd_toggle(self):
        if self._cd_running:
            self._cd_running = False
            self._cd_btn.config(text='RESUME', bg=PAL['success'])
        else:
            if not self._cd_running and self._cd_remaining == 0:
                self._cd_remaining = float(self._hh.get() * 3600 + self._mm.get() * 60 + self._ss.get())
                self._cd_total = self._cd_remaining
            self._cd_running = True
            self._cd_btn.config(text='PAUSE', bg=PAL['warn'])
            self._cd_tick()
