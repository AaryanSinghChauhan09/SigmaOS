"""
Auto-split from userland\apps\space_explorer.py — SpaceExplorer._complete_scan
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random



class SpaceExplorer:
    def _complete_scan(self, stype):
        self._draw_mock_map()
        self.status.config(text=f'{stype} SCAN COMPLETE | 100% VERIFIED', bg=PAL['success'])
        messagebox.showinfo('Scanner', f'{stype} Analysis Completed.\nNo anomalies detected.')
