# Generated method: CortexEngine.get_intelligence_state
from typing import Dict, Any
from .neural_mapper import NeuralMapper
from .vibe_orchestrator import VibeOrchestrator

class CortexEngine:
    def get_intelligence_state(self) -> Dict[str, Any]:
        """Provides a unified view of OS intelligence."""
        return {'profile': self.mapper.get_current_state(), 'vibe': self.orchestrator.current_vibe, 'modifiers': self.orchestrator.get_vibe_modifiers()}