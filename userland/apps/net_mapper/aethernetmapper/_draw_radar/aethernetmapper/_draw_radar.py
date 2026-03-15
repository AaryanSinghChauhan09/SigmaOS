# Generated method: AetherNetMapper._draw_radar
import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random

class AetherNetMapper:
    def _draw_radar(self, angle):
        self.canvas.delete('all')
        w, h = (370, 400)
        cx, cy = (w / 2, h / 2)
        r = min(cx, cy) - 20
        self.canvas.create_oval(cx - r, cy - r, cx + r, cy + r, outline=PAL['dim'])
        self.canvas.create_oval(cx - r / 2, cy - r / 2, cx + r / 2, cy + r / 2, outline=PAL['dim'])
        self.canvas.create_line(cx - r, cy, cx + r, cy, fill=PAL['dim'])
        self.canvas.create_line(cx, cy - r, cx, cy + r, fill=PAL['dim'])
        for _ in range(8):
            nx = cx + random.randint(int(-r / 1.5), int(r / 1.5))
            ny = cy + random.randint(int(-r / 1.5), int(r / 1.5))
            col = random.choice([PAL['accent'], PAL['warning'], PAL['danger']])
            self.canvas.create_oval(nx - 4, ny - 4, nx + 4, ny + 4, fill=col, outline='')
        if self.scanning:
            self.after(500, lambda: self._draw_radar((angle + 45) % 360))
        else:
            self.after(2000, lambda: self._draw_radar(0))