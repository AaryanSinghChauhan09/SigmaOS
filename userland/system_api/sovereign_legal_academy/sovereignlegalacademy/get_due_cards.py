# Generated method: SovereignLegalAcademy.get_due_cards
import time
import json
import random
from typing import List, Dict, Any, Optional

class SovereignLegalAcademy:
    def get_due_cards(self) -> List[Dict[str, Any]]:
        return [c for c in self.study_deck if c['due'] <= time.time()]