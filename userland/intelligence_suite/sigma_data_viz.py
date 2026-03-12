
"""
SigmaOS DataViz Engine v1.0
===========================
Professional-grade data visualization and business intelligence reporting.
Supports ASCII rendering, statistical profiling, and automated insights.
"""

import math
import time
from typing import List, Dict, Any, Optional

class SigmaDataViz:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.charts_generated = 0

    def generate_ascii_bar_chart(self, data: Dict[str, float], title: str = "Data Distribution") -> str:
        """Renders an industry-standard ASCII bar chart."""
        if not data:
            return "No data to visualize."
        
        max_val = max(data.values())
        max_label_len = max(len(str(k)) for k in data.keys())
        width = 40 # Max bars width

        chart = [f"\n--- {title} ---", ""]
        for label, value in data.items():
            bar_len = int((value / max_val) * width) if max_val > 0 else 0
            bar = "█" * bar_len
            chart.append(f"{str(label).rjust(max_label_len)} | {bar} ({value})")
        
        chart.append("\n" + "-" * (max_label_len + width + 10))
        self.charts_generated += 1
        return "\n".join(chart)

    def data_profile(self, dataset: List[float], name: str = "Dataset") -> Dict[str, Any]:
        """Performs statistical profiling for data analysts."""
        if not dataset:
            return {"error": "Empty dataset"}

        dataset.sort()
        n = len(dataset)
        mean = sum(dataset) / n
        median = dataset[n // 2] if n % 2 != 0 else (dataset[n // 2 - 1] + dataset[n // 2]) / 2
        
        variance = sum((x - mean) ** 2 for x in dataset) / n
        std_dev = math.sqrt(variance)

        profile = {
            "name": name,
            "count": n,
            "min": float(dataset[0]),
            "max": float(dataset[-1]),
            "mean": float(int(mean * 100)) / 100.0,
            "median": float(int(median * 100)) / 100.0,
            "std_dev": float(int(std_dev * 100)) / 100.0,
            "range": float(int((dataset[-1] - dataset[0]) * 100)) / 100.0
        }
        return profile

    def generate_business_summary(self, profile: Dict[str, Any]) -> str:
        """Automated Business Analyst summary generation."""
        summary = [
            f"BI REPORT: {profile.get('name', 'General Data')}",
            "=" * 30,
            f"Observed Sample Size: {profile['count']}",
            f"Performance Range: {profile['min']} -> {profile['max']}",
            f"Central Tendency: Mean ({profile['mean']}), Median ({profile['median']})",
            f"Stability Analysis: {'Highly Volatile' if profile['std_dev'] > (profile['mean'] * 0.5) else 'Stable Performance'}",
        ]
        return "\n".join(summary)
