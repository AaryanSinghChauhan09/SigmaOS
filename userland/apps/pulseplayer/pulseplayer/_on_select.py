"""
Auto-split from userland\apps\pulseplayer.py — PulsePlayer._on_select
"""

import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict



class PulsePlayer:
    def _on_select(self, event):
        sel = self.queue_tree.selection()
        if not sel:
            return
        idx = self.queue_tree.index(sel[0])
        self.current_idx = idx
        self._progress = 0
        self._update_track_display()
        if not self.playing:
            self.toggle()
