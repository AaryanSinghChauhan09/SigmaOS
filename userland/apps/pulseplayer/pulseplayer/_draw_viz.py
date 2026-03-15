"""
Auto-split from userland\apps\pulseplayer.py — PulsePlayer._draw_viz
"""

import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict



class PulsePlayer:
    def _draw_viz(self):
        self.viz.delete('all')
        w = int(self.viz.winfo_width()) or 700
        h = int(self.viz.winfo_height()) or 100
        n = 60
        bw = float(w) / n
        for i in range(n):
            if self.playing:
                v_data = int(self._viz_data[i])
                self._viz_data[i] = max(4, min(h - 4, v_data + random.randint(-15, 15)))
            bh = int(self._viz_data[i])
            x = float(i) * bw
            r_val = int(90 + 165 * (i / n))
            g_val = int(140 + 88 * (1 - i / n))
            color = f'#{r_val:02x}{g_val:02x}fa'
            self.viz.create_rectangle(x + 1, h - bh, x + bw - 2, h, fill=color, outline='')
