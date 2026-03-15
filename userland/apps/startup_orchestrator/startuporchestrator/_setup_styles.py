"""
Auto-split from userland\apps\startup_orchestrator.py — StartupOrchestrator._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random



class StartupOrchestrator:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Boot.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Boot.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Boot.TNotebook.Tab', background=[('selected', PAL['accent'])])
        style.configure('Boot.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Consolas', 9), rowheight=28)
        style.configure('Boot.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Boot.Treeview', background=[('selected', PAL['accent_dim'])])
