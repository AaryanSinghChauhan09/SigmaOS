"""
SigmaOS Cortex Engine (v2.0 Apex)
==================================
USP: Unified Neural Hub orchestrating Mapping and Orchestration.
Modular Architecture: Delegating focus to NeuralMapper and VibeOrchestrator.
"""
from typing import Dict, Any
from .neural_mapper import NeuralMapper
from .vibe_orchestrator import VibeOrchestrator

class CortexEngine:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.mapper = NeuralMapper(kernel)
        self.orchestrator = VibeOrchestrator(kernel)
        
    def process_telemetry(self, data: Dict[str, Any]):
        """Modularized processing pipeline."""
        self.mapper.update_map(data)
        profile = self.mapper.get_current_state()
        self.orchestrator.adjust_vibe(profile)
        
    def get_intelligence_state(self) -> Dict[str, Any]:
        """Provides a unified view of OS intelligence."""
        return {
            "profile": self.mapper.get_current_state(),
            "vibe": self.orchestrator.current_vibe,
            "modifiers": self.orchestrator.get_vibe_modifiers()
        }

    def health_check(self) -> str:
        return "OK — Cortex Modularized (Mapper + Orchestrator)"
