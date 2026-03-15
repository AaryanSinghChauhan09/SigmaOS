# Generated method: SigmaStopwatch._build
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _build(self):
        hdr = tk.Frame(self, bg=PAL['panel'], height=50)
        hdr.pack(fill='x')
        hdr.pack_propagate(False)
        tk.Label(hdr, text=f"{ICONS.get('timer', '⏱')}  STOPWATCH & TIMER", fg=PAL['accent'], bg=PAL['panel'], font=('Segoe UI Bold', 13)).pack(side='left', padx=18, pady=10)
        nb = ttk.Notebook(self)
        nb.pack(fill='both', expand=True, padx=10, pady=10)
        self._build_stopwatch(nb)
        self._build_countdown(nb)
        self._build_pomodoro(nb)