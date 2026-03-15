# Generated method: SigmaStopwatch._pomo_tick
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _pomo_tick(self):
        if not self._pomo_running:
            return
        if self._pomo_remaining <= 0:
            self._pomo_next_phase()
            self.bell()
        rem = int(self._pomo_remaining)
        m = rem // 60
        s = rem % 60
        self._pomo_disp.config(text=f'{m:02}:{s:02}')
        self._pomo_remaining -= 1
        self.after(1000, self._pomo_tick)