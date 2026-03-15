"""
Auto-split from userland\apps\sigma_antigravity.py — SigmaAntigravity._setup_styles
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading, webbrowser, urllib.parse, json, os, time, sys
from typing import Dict, Any, List, Optional



class SigmaAntigravity:
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('Treeview', background=PAL['card'], foreground=PAL['text'], fieldbackground=PAL['card'])
        s.configure('TNotebook', background=PAL['bg'])
        s.configure('TNotebook.Tab', background=PAL['panel'], foreground=PAL['dim'], padding=[15, 5])
        s.map('TNotebook.Tab', background=[('selected', PAL['card'])], foreground=[('selected', 'white')])
