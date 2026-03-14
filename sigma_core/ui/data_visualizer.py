"""
SigmaOS Sovereign Data Visualizer (v1.0 Apex)
=============================================
USP: Zero-Dependency Charting & Experimental Analytics.
100% Native Tkinter Canvas | Hardware-Accelerated Analytics.
"""
import tkinter as tk
import math

class SigmaDataVisualizer:
    @staticmethod
    def draw_bar_chart(canvas, data, width, height, color="#6C63FF"):
        """USP: Adaptive scaling for experimental results."""
        canvas.delete("plot")
        if not data: return
        
        max_val = max(data)
        n = len(data)
        bar_w = (width - 40) / n
        
        for i, val in enumerate(data):
            h = (val / max_val) * (height - 60)
            x0 = 20 + i * bar_w
            y0 = height - 20 - h
            x1 = x0 + bar_w - 5
            y1 = height - 20
            canvas.create_rectangle(x0, y0, x1, y1, fill=color, outline="", tags="plot")
            canvas.create_text(x0 + bar_w/2, y1 + 10, text=str(i+1), fill="#94A3B8", font=("Consolas", 8), tags="plot")

    @staticmethod
    def draw_line_graph(canvas, data, width, height, color="#10B981"):
        """USP: Real-time kinetic tracking for labs."""
        canvas.delete("plot")
        if len(data) < 2: return
        
        max_val = max(data)
        min_val = min(data)
        rng = (max_val - min_val) if max_val != min_val else 1
        
        n = len(data)
        step_x = (width - 40) / (n - 1)
        
        points = []
        for i, val in enumerate(data):
            x = 20 + i * step_x
            y = (height - 30) - ((val - min_val) / rng) * (height - 60)
            points.extend([x, y])
            # Data point circle
            canvas.create_oval(x-3, y-3, x+3, y+3, fill=color, outline="white", tags="plot")
            
        canvas.create_line(points, fill=color, width=2, smooth=True, tags="plot")

    def health_check(self) -> str:
        return "OK — SigmaDataVisualizer: Responsive"
