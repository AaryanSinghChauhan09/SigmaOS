"""
SigmaOS Telemetry Visualizer v1.0
==================================
USP: Radical Transparency & Visualization.
Provides ASCII-based or JSON-data for real-time system performance 
monitoring, making the invisible kernel visible.
"""
from typing import List
import math
import random

class TelemetryVisualizer:
    def __init__(self, kernel):
        self.kernel = kernel
        self.history: List[float] = []

    def log_metric(self, value: float):
        self.history.append(value)
        if len(self.history) > 50:
            self.history.pop(0)

    def draw_ascii_graph(self, height: int = 5) -> str:
        """Generates a sparkline-style ASCII graph from history."""
        if not self.history: return "[ No Data ]"
        
        mx = max(self.history) if self.history else 1
        mn = min(self.history) if self.history else 0
        span = mx - mn if mx != mn else 1
        
        graph = ""
        for h in range(height, 0, -1):
            line = ""
            threshold = mn + (span * (h / height))
            for val in self.history:
                if val >= threshold:
                    line += "█"
                elif val >= threshold - (span / height * 0.5):
                    line += "▄"
                else:
                    line += " "
            graph += line + "\n"
        
        return graph

    def get_realtime_stream(self):
        """Mock stream of CPU/Memory usage."""
        # Simulated volatility
        val = 10 + 20 * math.sin(random.random()) + random.uniform(0, 40)
        self.log_metric(val)
        return val

if __name__ == "__main__":
    tv = TelemetryVisualizer(None)
    for _ in range(30):
        tv.get_realtime_stream()
    print("--- SIGMA KERNEL LOAD VISUALIZATION ---")
    print(tv.draw_ascii_graph())
