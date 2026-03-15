"""
Auto-split from userland\apps\space_explorer.py — SpaceExplorer._quick_scan
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random



class SpaceExplorer:
    def _quick_scan(self):
        self.status.config(text='QUICK HEURISTIC SCAN IN PROGRESS...', bg=PAL['accent'])
        self.after(1000, lambda: self._complete_scan('QUICK'))
