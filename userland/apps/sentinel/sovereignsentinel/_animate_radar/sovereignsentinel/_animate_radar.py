# Generated method: SovereignSentinel._animate_radar
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

class SovereignSentinel:
    def _animate_radar(self, step):
        import math
        self.viz_canvas.delete('all')
        cx, cy, r = (210, 110, 90)
        for ri in [30, 60, 90]:
            self.viz_canvas.create_oval(cx - ri, cy - ri, cx + ri, cy + ri, outline='#1a1a2a')
        sweep_end_x = cx + r * math.cos(math.radians(step))
        sweep_end_y = cy + r * math.sin(math.radians(step))
        self.viz_canvas.create_line(cx, cy, sweep_end_x, sweep_end_y, fill=PAL['safe'], width=2)
        for i in range(12):
            angle = (step + i * 30) % 360
            rad = math.radians(angle)
            dist = random.uniform(0.4, 0.9) * r
            x = cx + dist * math.cos(rad)
            y = cy + dist * math.sin(rad)
            col = PAL['danger'] if i == 3 else PAL['safe']
            alpha = max(0, 1 - i / 12)
            self.viz_canvas.create_oval(x - 4, y - 4, x + 4, y + 4, fill=col, outline='')
        self.after(40, lambda: self._animate_radar((step + 3) % 360))