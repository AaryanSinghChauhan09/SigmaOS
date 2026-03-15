"""
Auto-split from userland\apps\quantum_bi.py — QuantumBIEngine._draw_chart
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import math



class QuantumBIEngine:
    def _draw_chart(self):
        self.canvas.delete('all')
        w, h = (650, 450)
        self.canvas.create_line(50, h - 50, w - 50, h - 50, fill=PAL['dim'], width=2)
        self.canvas.create_line(50, 50, 50, h - 50, fill=PAL['dim'], width=2)
        bars = 6
        spacing = (w - 120) / bars
        for i in range(bars):
            val = random.randint(50, 300)
            x1 = 70 + i * spacing
            y1 = h - 50
            x2 = x1 + (spacing - 20)
            y2 = h - 50 - val
            c = random.choice([PAL['chart1'], PAL['chart2'], PAL['chart3'], PAL['accent']])
            self.canvas.create_rectangle(x1, y1, x2, y2, fill=c, outline=PAL['bg'], width=2)
            self.canvas.create_text((x1 + x2) / 2, y2 - 15, text=f'${val}M', fill=PAL['text'], font=('Inter', 8, 'bold'))
            self.canvas.create_text((x1 + x2) / 2, y1 + 15, text=f'Q{i + 1}', fill=PAL['dim'], font=('Inter', 8, 'bold'))
        self.canvas.create_text(w / 2, 30, text='SOVEREIGN REVENUE MATRIX (QUARTERLY)', fill=PAL['text'], font=('Inter', 12, 'bold'))
