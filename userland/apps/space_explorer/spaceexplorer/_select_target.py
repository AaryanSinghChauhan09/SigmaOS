"""
Auto-split from userland\apps\space_explorer.py — SpaceExplorer._select_target
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random



class SpaceExplorer:
    def _select_target(self):
        d = filedialog.askdirectory()
        if d:
            self.target_drive = d
            self.drive_lbl.config(text=f'TARGET: {d}')
            self.status.config(text=f'TARGET ACQUIRED: {d}')
