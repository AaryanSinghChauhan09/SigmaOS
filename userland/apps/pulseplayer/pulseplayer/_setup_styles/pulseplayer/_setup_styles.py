# Generated method: PulsePlayer._setup_styles
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('Treeview', background=PAL['player_bg'], foreground=PAL['text'], fieldbackground=PAL['player_bg'], borderwidth=0, font=('Segoe UI', 9))
        s.map('Treeview', background=[('selected', PAL['secondary'])])
        s.configure('Horizontal.TScale', background=PAL['header'], troughcolor=PAL['border'])