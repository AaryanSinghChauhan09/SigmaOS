# Generated method: GurukulEngine._save_cards
import time
import json
import os
from sigma_core.system.interfaces import SigmaModuleBase

class GurukulEngine:
    def _save_cards(self):
        if not os.path.exists(os.path.dirname(self.cards_path)):
            os.makedirs(os.path.dirname(self.cards_path), exist_ok=True)
        with open(self.cards_path, 'w') as f:
            json.dump(self.knowledge_base, f, indent=4)