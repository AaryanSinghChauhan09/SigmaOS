# Generated method: MorphicLayout.__init__
import math
from typing import List, Dict, Any, Tuple
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicLayout:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.screen_res = (1920, 1080)
        self.active_layout = 'FLOATING'
        self.padding = 10