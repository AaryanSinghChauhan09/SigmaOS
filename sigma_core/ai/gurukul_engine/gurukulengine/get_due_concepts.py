# Generated method: GurukulEngine.get_due_concepts
import time
import json
import os
from sigma_core.system.interfaces import SigmaModuleBase

class GurukulEngine:
    def get_due_concepts(self):
        now = time.time()
        return [cid for cid, c in self.knowledge_base.items() if c['next_review'] <= now]