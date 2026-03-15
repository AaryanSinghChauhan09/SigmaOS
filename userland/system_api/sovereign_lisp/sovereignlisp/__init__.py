# Generated method: SovereignLisp.__init__
from dataclasses import dataclass
from typing import List, Any

class SovereignLisp:
    def __init__(self, kernel):
        self.kernel = kernel
        self.heap: List[LispObject] = []
        self.max_heap = 1000
        self.globals = {'+': self._sum, '-': self._sub, '*': self._mul, 'draw-box': self._draw_box, 'gc': self._trigger_gc}