# Generated method: SigmaDSStudio.health_check
from typing import Dict, List, Any
import time
import random

class SigmaDSStudio:
    def health_check(self) -> str:
        return f'OK — Active Lakes: {len(self._data_stores)} | Pipelines: {len(self._pipeline_history)}.'