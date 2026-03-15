"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager._build_cognitive_dag
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def _build_cognitive_dag(self, phase: str='') -> str:
        if self.kernel and hasattr(self.kernel, 'registry'):
            ar = self.kernel.registry.get('agentic_runtime')
            if ar and hasattr(ar, 'build_sovereign_graph'):
                ar.build_sovereign_graph('OS-Orchestrator', ['Listen', 'Decide', 'Act'], {'Listen': ['Decide'], 'Decide': ['Act']})
                return 'Sovereign Cognitive DAG built (LangGraph Alternative).'
        return 'Agentic Runtime offline.'
