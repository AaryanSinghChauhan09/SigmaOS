"""
Auto-split from userland\apps\jigsaw_puzzle.py — JigsawPuzzle._stat_row
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os



class JigsawPuzzle:
    def _stat_row(self, parent, label, value):
        row = tk.Frame(parent, bg=PAL['card'])
        row.pack(fill='x', pady=4)
        tk.Label(row, text=label, font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['card'], width=8, anchor='w').pack(side='left')
        lbl = tk.Label(row, text=value, font=('Segoe UI', 11, 'bold'), fg=PAL['accent'], bg=PAL['card'])
        lbl.pack(side='left')
        return lbl
