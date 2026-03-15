"""
Auto-split from userland\apps\omni_savant.py — OmniSavant._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniSavant:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Savant.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 10), rowheight=35)
        style.configure('Savant.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Savant.Treeview', background=[('selected', PAL['accent_dim'])])
