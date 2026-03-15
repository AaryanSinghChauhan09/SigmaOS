"""
Auto-split from userland\apps\energy_core.py — EnergyCore._create_metric_card
"""

import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
import time
import random
from userland.system_api.sigma_std import SigmaSys



class EnergyCore:
    def _create_metric_card(self, parent, title, value, color):
        f = tk.Frame(parent, bg=PAL['panel'], padx=15, pady=15)
        tk.Label(f, text=title, font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        lbl = tk.Label(f, text=value, font=('Inter', 16, 'bold'), fg=color, bg=PAL['panel'])
        lbl.pack(anchor='w', pady=(5, 0))
        f.val_lbl = lbl
        return f
