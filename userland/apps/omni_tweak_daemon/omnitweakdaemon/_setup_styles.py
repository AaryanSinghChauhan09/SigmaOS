"""
Auto-split from userland\apps\omni_tweak_daemon.py — OmniTweakDaemon._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniTweakDaemon:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Tweak.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Tweak.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Tweak.TNotebook.Tab', background=[('selected', PAL['accent'])])
        style.configure('Tweak.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Consolas', 9), rowheight=25)
        style.configure('Tweak.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Tweak.Treeview', background=[('selected', PAL['accent_dim'])])
