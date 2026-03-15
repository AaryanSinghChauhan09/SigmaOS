"""
Auto-split from userland\apps\omni_etl_forge.py — OmniETLForge._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading



class OmniETLForge:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('ETL.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 9), rowheight=25)
        style.configure('ETL.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.configure('ETL.Horizontal.TProgressbar', background=PAL['accent'], troughcolor=PAL['sidebar'], borderwidth=0)
