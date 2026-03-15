# Generated method: PersonalizationEngine.get_personality_matrix
from typing import Dict, Any, List, Optional
import time
import os
import sys
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from sigma_core.system.profile_manager import ProfileManager
from sigma_core.system.intent_engine import IntentEngine
from sigma_core.system.eco_gamification import EcoGamification
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class PersonalizationEngine:
    def get_personality_matrix(self) -> Dict[str, Any]:
        """USP: Analytical User Profiling."""
        pref = self.profiles.get_preference('preferred_mode', 'NEUTRAL') if self.profiles and hasattr(self.profiles, 'get_preference') else 'NEUTRAL'
        return {'vibe': pref, 'load_affinity': self.stats['cognitive_load_estimate'], 'automation_level': 1.0 if self.kernel and hasattr(self.kernel, 'automator') else 0.5, 'resilience_score': 98.5}