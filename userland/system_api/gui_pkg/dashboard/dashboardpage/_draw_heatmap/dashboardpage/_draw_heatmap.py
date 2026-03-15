# Generated method: DashboardPage._draw_heatmap
import tkinter as tk
from tkinter import ttk, scrolledtext
import random
import time
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO, FONT_MED, FONT_TITLE

class DashboardPage:
    def _draw_heatmap(self):
        if not self.heatmap_canvas.winfo_exists():
            return
        self.heatmap_canvas.delete('all')
        W = self.heatmap_canvas.winfo_width()
        if W < 10:
            W = 200
        for i in range(12):
            x1 = W / 12 * i
            x2 = x1 + W / 12 - 3
            intensity = random.randint(40, 120)
            color = f'#{intensity:02x}20{200 - intensity:02x}'
            if i % 4 == 0:
                color = PAL['accent'] if random.random() > 0.7 else PAL['bg3']
            self.heatmap_canvas.create_rectangle(x1, 5, x2, 35, fill=color, outline='')
        self.after(1000, self._draw_heatmap)