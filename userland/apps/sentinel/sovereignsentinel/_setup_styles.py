"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._setup_styles
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('Treeview', background=PAL['panel'], foreground=PAL['text'], fieldbackground=PAL['panel'], borderwidth=0, font=('Segoe UI', 9))
        s.configure('Treeview.Heading', background=PAL['sidebar'], foreground=PAL['dim'], font=('Segoe UI', 8, 'bold'))
        s.map('Treeview', background=[('selected', PAL['accent2'])])
