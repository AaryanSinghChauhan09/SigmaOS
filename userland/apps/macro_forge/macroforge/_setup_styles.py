"""
Auto-split from userland\apps\macro_forge.py — MacroForge._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random



class MacroForge:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Forge.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 10), rowheight=40)
        style.configure('Forge.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Forge.Treeview', background=[('selected', PAL['panel'])])
