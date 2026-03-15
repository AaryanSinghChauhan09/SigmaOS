"""
Auto-split from userland\apps\quantum_bi.py — QuantumBIEngine._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import math



class QuantumBIEngine:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('BI.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 9), rowheight=25)
        style.configure('BI.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('BI.Treeview', background=[('selected', PAL['accent_dim'])])
