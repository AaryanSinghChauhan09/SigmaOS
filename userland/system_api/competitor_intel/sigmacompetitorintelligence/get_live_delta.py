"""
Auto-split from userland\system_api\competitor_intel.py — SigmaCompetitorIntelligence.get_live_delta
"""

import time
import random
from typing import Dict, List, Any



class SigmaCompetitorIntelligence:
    def get_live_delta(self, competitor: str='Windows 11') -> List[Dict]:
        """Returns per-metric live delta (Sigma advantage in %) vs a competitor."""
        deltas = []
        baseline = _COMPETITOR_BASELINES.get(competitor, {})
        for metric, sigma_val in _SIGMA_TARGETS.items():
            comp_val = baseline.get(metric, 1)
            lower = _LOWER_BETTER[metric]
            adv = (comp_val - sigma_val) / comp_val * 100 if lower else (sigma_val - comp_val) / comp_val * 100
            deltas.append({'metric': metric, 'sigma': sigma_val, 'competitor': comp_val, 'advantage': f'+{adv:.0f}%', 'wins': adv > 0})
        return deltas
