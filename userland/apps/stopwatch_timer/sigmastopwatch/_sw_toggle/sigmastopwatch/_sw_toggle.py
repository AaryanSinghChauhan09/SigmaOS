# Generated method: SigmaStopwatch._sw_toggle
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _sw_toggle(self):
        if self._sw_running:
            self._sw_running = False
            self._sw_elapsed += time.time() - self._sw_start
            self._sw_start_btn.config(text=f"{ICONS.get('perf', '▶')} START", bg=PAL['success'])
        else:
            self._sw_running = True
            self._sw_start = time.time()
            self._sw_start_btn.config(text='PAUSE', bg=PAL['warn'])
            self._sw_tick()