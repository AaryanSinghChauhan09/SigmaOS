"""
Auto-split from userland\system_api\sovereign_sync.py — SigmaSovereignSync.get_total_mesh_power
"""

import socket
import json
import uuid
import random
import time
from dataclasses import dataclass, field



class SigmaSovereignSync:
    def get_total_mesh_power(self) -> str:
        total_ram = sum((p.available_ram_gb for p in self.peers.values()))
        total_cores = sum((p.cpu_cores for p in self.peers.values()))
        return f'{total_ram:.1f} GB RAM / {total_cores} Cores'
