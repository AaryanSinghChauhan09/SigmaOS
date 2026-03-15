# Generated method: SigmaDiagnostics.get_repair_history
import time
import uuid
import random
from dataclasses import dataclass
from enum import Enum, auto

class SigmaDiagnostics:
    def get_repair_history(self) -> list[dict]:
        return self._repairs[-10:]