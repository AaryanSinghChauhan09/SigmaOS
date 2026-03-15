# Generated method: MorphicLayout.switch_layout
import math
from typing import List, Dict, Any, Tuple
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicLayout:
    def switch_layout(self, layout_type: str):
        self.active_layout = layout_type.upper()
        self.log_event('layout_switch', {'type': self.active_layout})