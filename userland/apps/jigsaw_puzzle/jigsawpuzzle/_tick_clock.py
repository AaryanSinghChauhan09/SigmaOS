"""
Auto-split from userland\apps\jigsaw_puzzle.py — JigsawPuzzle._tick_clock
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os



class JigsawPuzzle:
    def _tick_clock(self):
        if self.start_time and (not self.solved):
            elapsed = int(time.time() - self.start_time)
            m, s = divmod(elapsed, 60)
            self.lbl_time.config(text=f'{m:02}:{s:02}')
        self.after(1000, self._tick_clock)
