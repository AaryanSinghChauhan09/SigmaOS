# Generated method: SigmaCAAT.list_rules
from enum import Enum
import time
import random
from dataclasses import dataclass, field

class SigmaCAAT:
    def list_rules(self) -> list[dict]:
        return [{'name': r.name, 'condition': r.condition, 'action': r.action, 'enabled': r.enabled} for r in self._rules]