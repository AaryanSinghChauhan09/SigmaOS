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
        
        mx = float(max(self.history))
        mn = float(min(self.history))
        span = mx - mn if mx != mn else 1.0
        
        graph_lines = []
        for h in range(height, 0, -1):
            line_chars = []
            rel_h = float(h) / float(height)
            threshold = mn + (span * rel_h)
            for val in self.history:
                if float(val) >= threshold:
                    line_chars.append("█")
                elif float(val) >= threshold - (span / float(height) * 0.5):
                    line_chars.append("▄")
                else:
                    line_chars.append(" ")
            graph_lines.append("".join(line_chars))
        
        return "\n".join(graph_lines) + "\n"

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
