# Generated method: SovereignLisp._draw_box
from dataclasses import dataclass
from typing import List, Any

class SovereignLisp:
    def _draw_box(self, args):
        if hasattr(self.kernel, 'registry'):
            return 'BOX_DRAW_DISPATCHED'