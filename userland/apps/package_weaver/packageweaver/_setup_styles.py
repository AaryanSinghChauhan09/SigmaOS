"""
Auto-split from userland\apps\package_weaver.py — PackageWeaver._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class PackageWeaver:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Weaver.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 10), rowheight=35)
        style.configure('Weaver.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Weaver.Treeview', background=[('selected', PAL['accent_dim'])])
