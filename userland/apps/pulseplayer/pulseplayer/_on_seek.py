"""
Auto-split from userland\apps\pulseplayer.py — PulsePlayer._on_seek
"""

import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict



class PulsePlayer:
    def _on_seek(self, val):
        t = self.tracks[self.current_idx]
        dur_s = float(t.get('dur_s', 300))
        self._progress = float(val) / 100.0 * dur_s
