"""
Auto-split from sigma_core\system\adaptive_governor.py — AdaptiveGovernor._apply_profile
"""

from typing import Dict, Any, List



class AdaptiveGovernor:
    def _apply_profile(self, perf: float, eco: bool, scheduler: str):
        """Orchestrates across specialized system modules."""
        self.state['performance_level'] = perf
        self.state['eco_priority'] = eco
        if hasattr(self.kernel, 'pbs'):
            self.kernel.pbs.set_policy(scheduler)
        if hasattr(self.kernel, 'perf'):
            intensity = 'High' if perf > 1.2 else 'Medium'
            self.kernel.perf.apply_tuning(intensity)
