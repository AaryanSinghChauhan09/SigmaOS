# Generated method: PulsePlayer._prev
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _prev(self):
        self.current_idx = max(0, self.current_idx - 1)
        self._progress = 0
        self._update_track_display()
        self._populate_queue()