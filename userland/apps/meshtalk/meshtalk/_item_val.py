"""
Auto-split from userland\apps\meshtalk.py — MeshTalk._item_val
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time



class MeshTalk:
    def _item_val(self, parent, key, val, color):
        f = tk.Frame(parent, bg=PAL['sidebar'], pady=10)
        f.pack(fill='x')
        tk.Label(f, text=key, font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        tk.Label(f, text=val, font=('Inter', 10, 'bold'), fg=color, bg=PAL['sidebar']).pack(anchor='w')
