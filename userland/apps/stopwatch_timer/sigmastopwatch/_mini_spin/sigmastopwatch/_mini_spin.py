# Generated method: SigmaStopwatch._mini_spin
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _mini_spin(self, parent, label, default):
        fr = tk.Frame(parent, bg=PAL['card'])
        fr.pack(fill='x', pady=2)
        tk.Label(fr, text=label, fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 9), width=16, anchor='w').pack(side='left')
        v = tk.IntVar(value=default)
        tk.Spinbox(fr, from_=1, to=120, textvariable=v, width=5, bg=PAL['bg'], fg='white', font=('Cascadia Code', 10), buttonbackground=PAL['bg'], relief='flat').pack(side='left')
        return v