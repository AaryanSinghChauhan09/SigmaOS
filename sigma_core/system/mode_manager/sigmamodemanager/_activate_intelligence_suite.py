"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager._activate_intelligence_suite
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def _activate_intelligence_suite(self, phase: str='') -> str:
        """USP: Hydrates professional intelligence engines for Data/AI roles."""
        engines = []
        if self.kernel:
            if hasattr(self.kernel, 'viz_engine') and self.kernel.viz_engine:
                engines.append('DataViz')
            if hasattr(self.kernel, 'ml_engine') and self.kernel.ml_engine:
                engines.append('MLEngine')
            if hasattr(self.kernel, 'genai_lab') and self.kernel.genai_lab:
                engines.append('GenAILab')
            if hasattr(self.kernel, 'insights_engine') and self.kernel.insights_engine:
                engines.append('InsightsEngine')
            if hasattr(self.kernel, 'sql_forge') and self.kernel.sql_forge:
                engines.append('SQLForge')
            if hasattr(self.kernel, 'hypertune') and self.kernel.hypertune:
                engines.append('HyperTune')
        if engines:
            return f"Intelligence Suite Active: {', '.join(engines)} hydrated."
        return 'Intelligence Suite: Engines offline or not found in registry.'
