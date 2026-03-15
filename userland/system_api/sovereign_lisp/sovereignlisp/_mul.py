# Generated method: SovereignLisp._mul
from dataclasses import dataclass
from typing import List, Any

class SovereignLisp:
    def _mul(self, args):
        res = 1
        for a in args:
            res *= a.value
        return res