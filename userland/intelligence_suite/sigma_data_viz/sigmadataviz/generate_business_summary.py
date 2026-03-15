# Generated method: SigmaDataViz.generate_business_summary
import math
import time
from typing import List, Dict, Any, Optional

class SigmaDataViz:
    def generate_business_summary(self, profile: Dict[str, Any]) -> str:
        """Automated Business Analyst summary generation."""
        summary = [f"BI REPORT: {profile.get('name', 'General Data')}", '=' * 30, f"Observed Sample Size: {profile['count']}", f"Performance Range: {profile['min']} -> {profile['max']}", f"Central Tendency: Mean ({profile['mean']}), Median ({profile['median']})", f"Stability Analysis: {('Highly Volatile' if profile['std_dev'] > profile['mean'] * 0.5 else 'Stable Performance')}"]
        return '\n'.join(summary)