# Generated method: AerynSearch.health_check
import os
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class AerynSearch:
    def health_check(self) -> str:
        return f"OK — Index: {self.stats['indexed_documents']} docs | {self.stats['queries_served']} queries"