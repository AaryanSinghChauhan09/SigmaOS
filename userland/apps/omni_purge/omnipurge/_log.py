"""
Auto-split from userland\apps\omni_purge.py — OmniPurge._log
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniPurge:
    def _log(self, msg):
        self.term.config(state=tk.NORMAL)
        self.term.insert(tk.END, f'{msg}\n')
        self.term.see(tk.END)
        self.term.config(state=tk.DISABLED)
