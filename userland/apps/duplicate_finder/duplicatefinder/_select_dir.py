"""
Auto-split from userland\apps\duplicate_finder.py — DuplicateFinder._select_dir
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import hashlib
from pathlib import Path



class DuplicateFinder:
    def _select_dir(self):
        d = filedialog.askdirectory()
        if d:
            self.target_dir = d
            self.drop_lbl.config(text=f'TARGET: {d}', fg=PAL['accent'])
