# Generated method: SigmaSpotlight.health_check
from typing import List, Dict, Any
import time

class SigmaSpotlight:
    def health_check(self) -> str:
        return f'OK — Index: {sum((len(v) for v in self._index.values()))} nodes.'