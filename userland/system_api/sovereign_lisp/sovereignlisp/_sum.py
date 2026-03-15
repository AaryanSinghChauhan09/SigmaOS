# Generated method: SovereignLisp._sum
from dataclasses import dataclass
from typing import List, Any

class SovereignLisp:
    def _sum(self, args):
        return sum((a.value for a in args))