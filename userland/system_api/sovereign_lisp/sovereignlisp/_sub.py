# Generated method: SovereignLisp._sub
from dataclasses import dataclass
from typing import List, Any

class SovereignLisp:
    def _sub(self, args):
        return args[0].value - sum((a.value for a in args[1:]))