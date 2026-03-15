"""
Auto-split from userland\apps\nexus_ai.py — SovereignAINexus._setup_styles
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json



class SovereignAINexus:
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('Treeview', background=PAL['card'], foreground=PAL['text'], fieldbackground=PAL['card'], borderwidth=0, font=('Segoe UI', 9))
        s.configure('TNotebook', background=PAL['bg'], borderwidth=0)
        s.configure('TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['dim'], padding=[12, 6], font=('Segoe UI', 9))
        s.map('TNotebook.Tab', background=[('selected', PAL['card'])], foreground=[('selected', 'white')])
