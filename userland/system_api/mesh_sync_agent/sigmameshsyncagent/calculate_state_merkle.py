# Generated method: SigmaMeshSyncAgent.calculate_state_merkle
import os
import sys
import hashlib
import json
import time
import socket
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaMeshSyncAgent:
    def calculate_state_merkle(self, root_path: str) -> str:
        """Calculates a recursive hash of the system state."""
        hasher = hashlib.sha256()
        for root, dirs, files in os.walk(root_path):
            for file in sorted(files):
                if file.endswith(('.json', '.sigma', '.vault')):
                    fp = os.path.join(root, file)
                    with open(fp, 'rb') as f:
                        hasher.update(f.read())
        self.state_hash = str(hasher.hexdigest())
        return self.state_hash