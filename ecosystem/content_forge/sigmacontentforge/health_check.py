# Generated method: SigmaContentForge.health_check
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import time

class SigmaContentForge:
    def health_check(self) -> str:
        s = self._stats
        total = s['extractions'] + s['conversions'] + s['audits']
        return f"OK — {total} Jobs Processed. Audits: {s['audits']}, Shards: {s['conversions']}."