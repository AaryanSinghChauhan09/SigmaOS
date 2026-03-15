"""
Auto-split from userland\apps\stopwatch_timer.py — SigmaStopwatch._cd_reset
"""

import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any



class SigmaStopwatch:
    def _cd_reset(self):
        self._cd_running = False
        self._cd_remaining = 0.0
        self._cd_disp.config(text='00:00:00', fg=PAL['warn'])
        self._cd_btn.config(text=f"{ICONS.get('perf', '▶')} START", bg=PAL['warn'])
        self._cd_status.config(text='Set timer and press START')
