# Generated method: SigmaSovereignZenith.search_context_snippet
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import random
import json
import os
from pathlib import Path

class SigmaSovereignZenith:
    def search_context_snippet(self, query: str) -> List[str]:
        """Fast-text search across the project index."""
        q = query.lower()
        return [p for p in self.project_index if q in p.lower()][:20]