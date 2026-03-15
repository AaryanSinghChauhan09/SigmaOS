# Generated method: VibeOrchestrator.get_vibe_modifiers
from typing import Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class VibeOrchestrator:
    def get_vibe_modifiers(self) -> Dict[str, Any]:
        """Provides modifiers for UI and Resource shards."""
        if self.current_vibe == 'MINIMALIST':
            return {'transparency': 0.1, 'animation_speed': 0.5, 'resource_priority': 'STABILITY'}
        return {'transparency': 0.8, 'animation_speed': 1.0, 'resource_priority': 'PERFORMANCE'}