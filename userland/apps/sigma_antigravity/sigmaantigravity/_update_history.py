"""
Auto-split from userland\apps\sigma_antigravity.py — SigmaAntigravity._update_history
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading, webbrowser, urllib.parse, json, os, time, sys
from typing import Dict, Any, List, Optional



class SigmaAntigravity:
    def _update_history(self):
        self.hist_tree.delete(*self.hist_tree.get_children())
        for rec in reversed(self.engine.history):
            self.hist_tree.insert('', 'end', values=(rec['time'], ', '.join(rec['platforms']), rec['prompt'][:100]))
