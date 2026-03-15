"""
Auto-split from userland\apps\stopwatch_timer.py — SigmaStopwatch._sw_tick
"""

import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any



class SigmaStopwatch:
    def _sw_tick(self):
        if not self._sw_running:
            return
        elapsed = self._sw_elapsed + (time.time() - self._sw_start)
        h = int(elapsed // 3600)
        m = int(elapsed % 3600 // 60)
        s = int(elapsed % 60)
        cs = int(elapsed % 1 * 100)
        self._sw_disp.config(text=f'{h:02}:{m:02}:{s:02}.{cs:02}')
        self.after(10, self._sw_tick)
