"""
Auto-split from userland\apps\stopwatch_timer.py — SigmaStopwatch._spin
"""

import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any



class SigmaStopwatch:
    def _spin(self, parent, label, fr, to):
        v = tk.IntVar(value=0)
        tk.Label(parent, text=label, fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 7)).pack(side='left', padx=(4, 0))
        sb = tk.Spinbox(parent, from_=fr, to=to, textvariable=v, width=4, bg=PAL['card'], fg='white', font=('Cascadia Code', 18), buttonbackground=PAL['card'], relief='flat')
        sb.pack(side='left')
        return v
