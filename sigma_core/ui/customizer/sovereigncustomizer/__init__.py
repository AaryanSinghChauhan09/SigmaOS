# Generated method: SovereignCustomizer.__init__
from typing import Dict, Any, List
from .morphic_engine import MorphicEngine
from .sensory_assets import SensoryAssets
from .neural_themer import NeuralThemer

class SovereignCustomizer:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.morphic = MorphicEngine(kernel)
        self.sensory = SensoryAssets(kernel)
        self.themer = NeuralThemer(kernel)