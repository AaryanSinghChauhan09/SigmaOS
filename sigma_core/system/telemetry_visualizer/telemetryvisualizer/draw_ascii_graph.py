# Generated method: TelemetryVisualizer.draw_ascii_graph
from typing import List
import math
import random

class TelemetryVisualizer:
    def draw_ascii_graph(self, height: int=5) -> str:
        """Generates a sparkline-style ASCII graph from history."""
        if not self.history:
            return '[ No Data ]'
        mx = float(max(self.history))
        mn = float(min(self.history))
        span = mx - mn if mx != mn else 1.0
        graph_lines = []
        for h in range(height, 0, -1):
            line_chars = []
            rel_h = float(h) / float(height)
            threshold = mn + span * rel_h
            for val in self.history:
                if float(val) >= threshold:
                    line_chars.append('█')
                elif float(val) >= threshold - span / float(height) * 0.5:
                    line_chars.append('▄')
                else:
                    line_chars.append(' ')
            graph_lines.append(''.join(line_chars))
        return '\n'.join(graph_lines) + '\n'