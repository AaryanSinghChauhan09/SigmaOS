# Generated method: SigmaDataVisualizer.draw_bar_chart
import tkinter as tk
import math

class SigmaDataVisualizer:
    @staticmethod
    def draw_bar_chart(canvas, data, width, height, color='#6C63FF'):
        """USP: Adaptive scaling for experimental results."""
        canvas.delete('plot')
        if not data:
            return
        max_val = max(data)
        n = len(data)
        bar_w = (width - 40) / n
        for i, val in enumerate(data):
            h = val / max_val * (height - 60)
            x0 = 20 + i * bar_w
            y0 = height - 20 - h
            x1 = x0 + bar_w - 5
            y1 = height - 20
            canvas.create_rectangle(x0, y0, x1, y1, fill=color, outline='', tags='plot')
            canvas.create_text(x0 + bar_w / 2, y1 + 10, text=str(i + 1), fill='#94A3B8', font=('Consolas', 8), tags='plot')