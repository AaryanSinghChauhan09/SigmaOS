"""
Auto-split from userland\apps\spot_it.py — SpotItGame._mini_stat
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _mini_stat(self, parent, label, value):
        row = tk.Frame(parent, bg=PAL['card'])
        row.pack(fill='x', pady=3)
        tk.Label(row, text=label, font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['card'], width=10, anchor='w').pack(side='left')
        lbl = tk.Label(row, text=value, font=('Segoe UI', 11, 'bold'), fg=PAL['accent'], bg=PAL['card'])
        lbl.pack(side='left')
        return lbl
