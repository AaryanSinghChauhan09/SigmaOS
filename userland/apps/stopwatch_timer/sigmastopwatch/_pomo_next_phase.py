"""
Auto-split from userland\apps\stopwatch_timer.py — SigmaStopwatch._pomo_next_phase
"""

import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any



class SigmaStopwatch:
    def _pomo_next_phase(self):
        if self._pomo_state == 'work':
            self._pomo_count += 1
            self._pomo_lbl.config(text=f'Sessions: {self._pomo_count}')
            if self._pomo_count % 4 == 0:
                self._pomo_state = 'long'
                self._pomo_remaining = float(self._long_break.get() * 60)
                self._pomo_state_lbl.config(text='🌿 LONG BREAK', fg=PAL['success'])
            else:
                self._pomo_state = 'break'
                self._pomo_remaining = float(self._break_min.get() * 60)
                self._pomo_state_lbl.config(text='☕ SHORT BREAK', fg=PAL['success'])
        else:
            self._pomo_state = 'work'
            self._pomo_remaining = float(self._work_min.get() * 60)
            self._pomo_state_lbl.config(text='🎯 FOCUS SESSION', fg=PAL['danger'])
