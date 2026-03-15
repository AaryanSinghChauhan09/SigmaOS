# Generated method: SpectralAnalyzer._draw_mock_treemap
import tkinter as tk
from tkinter import ttk, messagebox
import random

class SpectralAnalyzer:
    def _draw_mock_treemap(self):
        self.canvas.delete('all')
        w, h = (750, 500)
        colors = ['#1E90FF', '#00FA9A', '#FF6347', '#9370DB', PAL['sidebar'], PAL['panel']]
        for _ in range(30):
            x1 = random.randint(0, w - 50)
            y1 = random.randint(0, h - 50)
            x2 = min(x1 + random.randint(50, 200), w - 5)
            y2 = min(y1 + random.randint(50, 200), h - 5)
            c = random.choice(colors)
            self.canvas.create_rectangle(x1, y1, x2, y2, fill=c, outline=PAL['bg'], width=2)