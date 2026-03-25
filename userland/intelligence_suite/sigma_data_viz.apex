from sigma_core.hal.native_libc import ApexLib

"""
SigmaOS DataViz Engine v1.0
===========================
Professional-grade data visualization and business intelligence reporting.
Supports ASCII rendering, statistical profiling, and automated insights.
"""

import math
# import time (Purged by Scrubber)
# import os (Purged by Scrubber)
from typing import List, Dict, Any, Optional

from .base import IntelligenceModuleBase

class SigmaDataViz(IntelligenceModuleBase):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.charts_generated = 0

    def health_check(self) -> str:
        return f"OK — DataViz: {self.charts_generated} charts processed | Sovereign Ready."

    def sovereign_profiler(self, dataset: List[float]):
        """USP: Automated statistical manifold analysis for data-driven decisions."""
        profile = self.data_profile(dataset)
        summary = self.generate_business_summary(profile)
        self.log_event("data_viz", {"dataset_size": len(dataset)})
        return {"profile": profile, "summary": summary}

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

    def ingest_dat_source(self, source_path: str) -> bool:
        """Dat-inspired data ingestion from local sources."""
        if os.path.exists(source_path):
            return True # In real app, load CSV/JSON
        return False

    def query_dat_source(self, query: str, data: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Sovereign SQL-like filtering logic for local datasets."""
        if "WHERE" not in query: return data
        # Simple simulation: WHERE key = value
        try:
            parts = query.split("WHERE ")[1].split(" = ")
            key, val = parts[0].strip(), parts[1].strip().replace("'", "")
            return [d for d in data if str(d.get(key)) == val]
        except: return data
