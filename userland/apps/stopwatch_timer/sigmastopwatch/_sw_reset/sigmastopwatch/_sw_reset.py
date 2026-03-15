# Generated method: SigmaStopwatch._sw_reset
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _sw_reset(self):
        self._sw_running = False
        self._sw_elapsed = 0.0
        self._sw_disp.config(text='00:00:00.00')
        self._sw_start_btn.config(text=f"{ICONS.get('perf', '▶')} START", bg=PAL['success'])
        self._laps = []
        self._lap_box.delete('1.0', 'end')