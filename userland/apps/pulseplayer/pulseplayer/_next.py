"""
Auto-split from userland\apps\pulseplayer.py — PulsePlayer._next
"""

import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict



class PulsePlayer:
    def _next(self):
        self.current_idx = (self.current_idx + 1) % len(self.tracks)
        self._progress = 0
        self._update_track_display()
        self._populate_queue()
