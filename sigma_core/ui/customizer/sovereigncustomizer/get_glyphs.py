# Generated method: SovereignCustomizer.get_glyphs
from typing import Dict, Any, List
from .morphic_engine import MorphicEngine
from .sensory_assets import SensoryAssets
from .neural_themer import NeuralThemer

class SovereignCustomizer:
    def get_glyphs(self, set_name: str='Sovereign') -> Dict[str, str]:
        return self.sensory.get_glyphs(set_name)