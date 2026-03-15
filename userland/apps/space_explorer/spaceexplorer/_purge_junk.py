"""
Auto-split from userland\apps\space_explorer.py — SpaceExplorer._purge_junk
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random



class SpaceExplorer:
    def _purge_junk(self):
        self.status.config(text='PURGING ORPHANED CLUSTERS...', bg=PAL['danger'])
        self.after(800, lambda: messagebox.showinfo('Purge', 'Orphaned files, caches, and telemetry data purged securely.'))
