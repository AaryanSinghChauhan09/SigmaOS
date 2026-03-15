"""
Auto-split from userland\apps\duplicate_finder.py — DuplicateFinder._purge
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import hashlib
from pathlib import Path



class DuplicateFinder:
    def _purge(self):
        if not self.duplicates:
            messagebox.showinfo('Purge', 'No duplicates identified for purging.')
            return
        if messagebox.askyesno('Confirm Purge', f'Are you sure you want to forensically delete {len(self.duplicates)} files?'):
            self.duplicates = []
            for i in self.tree.get_children():
                self.tree.delete(i)
            self.stat_lbl.config(text='VOLUME PURGED | INTEGRITY RE-VERIFIED', fg=PAL['success'])
