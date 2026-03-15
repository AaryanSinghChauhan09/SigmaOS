# Generated method: SigmaDataViz.generate_ascii_bar_chart
import math
import time
from typing import List, Dict, Any, Optional

class SigmaDataViz:
    def generate_ascii_bar_chart(self, data: Dict[str, float], title: str='Data Distribution') -> str:
        """Renders an industry-standard ASCII bar chart."""
        if not data:
            return 'No data to visualize.'
        max_val = max(data.values())
        max_label_len = max((len(str(k)) for k in data.keys()))
        width = 40
        chart = [f'\n--- {title} ---', '']
        for label, value in data.items():
            bar_len = int(value / max_val * width) if max_val > 0 else 0
            bar = '█' * bar_len
            chart.append(f'{str(label).rjust(max_label_len)} | {bar} ({value})')
        chart.append('\n' + '-' * (max_label_len + width + 10))
        self.charts_generated += 1
        return '\n'.join(chart)