"""
Auto-split from userland\apps\context_engine.py — ContextEngine._log
"""

import tkinter as tk
from tkinter import ttk, messagebox
import time
import random



class ContextEngine:
    def _log(self, msg, color=None):
        self.term.config(state=tk.NORMAL)
        self.term.insert(tk.END, f'{msg}\n')
        self.term.see(tk.END)
        self.term.config(state=tk.DISABLED)
