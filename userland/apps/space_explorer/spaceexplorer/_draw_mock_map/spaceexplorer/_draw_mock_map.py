# Generated method: SpaceExplorer._draw_mock_map
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random

class SpaceExplorer:
    def _draw_mock_map(self):
        self.canvas.delete('all')
        colors = [PAL['accent'], PAL['success'], '#FFA500', PAL['danger'], '#8A2BE2', '#FF69B4']
        for _ in range(40):
            x1 = random.randint(10, 500)
            y1 = random.randint(10, 400)
            x2 = x1 + random.randint(20, 150)
            y2 = y1 + random.randint(20, 150)
            c = random.choice(colors)
            self.canvas.create_rectangle(x1, y1, x2, y2, fill=c, outline=PAL['bg'], width=2)