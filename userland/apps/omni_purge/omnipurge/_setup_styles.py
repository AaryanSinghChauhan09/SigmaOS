"""
Auto-split from userland\apps\omni_purge.py — OmniPurge._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniPurge:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Purge.Horizontal.TProgressbar', background=PAL['success'], troughcolor=PAL['sidebar'], borderwidth=0)
