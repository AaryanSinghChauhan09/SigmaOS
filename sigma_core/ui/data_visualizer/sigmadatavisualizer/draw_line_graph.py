# Generated method: SigmaDataVisualizer.draw_line_graph
import tkinter as tk
import math

class SigmaDataVisualizer:
    @staticmethod
    def draw_line_graph(canvas, data, width, height, color='#10B981'):
        """USP: Real-time kinetic tracking for labs."""
        canvas.delete('plot')
        if len(data) < 2:
            return
        max_val = max(data)
        min_val = min(data)
        rng = max_val - min_val if max_val != min_val else 1
        n = len(data)
        step_x = (width - 40) / (n - 1)
        points = []
        for i, val in enumerate(data):
            x = 20 + i * step_x
            y = height - 30 - (val - min_val) / rng * (height - 60)
            points.extend([x, y])
            canvas.create_oval(x - 3, y - 3, x + 3, y + 3, fill=color, outline='white', tags='plot')
        canvas.create_line(points, fill=color, width=2, smooth=True, tags='plot')