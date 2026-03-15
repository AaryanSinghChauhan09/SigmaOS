# Generated method: SigmaStopwatch._pomo_toggle
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _pomo_toggle(self):
        if self._pomo_running:
            self._pomo_running = False
            self._pomo_btn.config(text='RESUME')
        else:
            self._pomo_running = True
            self._pomo_btn.config(text='PAUSE')
            if self._pomo_remaining <= 0:
                self._pomo_remaining = float(self._work_min.get() * 60)
            self._pomo_tick()