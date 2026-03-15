"""
Auto-split from userland\apps\event_matrix.py — EventMatrix._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class EventMatrix:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Event.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Consolas', 9), rowheight=25)
        style.configure('Event.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Event.Treeview', background=[('selected', PAL['panel'])])
