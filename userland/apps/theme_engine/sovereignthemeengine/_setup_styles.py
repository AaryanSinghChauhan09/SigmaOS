"""
Auto-split from userland\apps\theme_engine.py — SovereignThemeEngine._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random



class SovereignThemeEngine:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Theme.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Theme.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Theme.TNotebook.Tab', background=[('selected', PAL['accent'])])
        style.configure('Theme.TScale', background=PAL['panel'], troughcolor=PAL['sidebar'])
