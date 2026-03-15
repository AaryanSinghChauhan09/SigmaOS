# Generated method: SigmaGhostChat._peer_discovery
import socket
import threading
import time
import json
import uuid
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from userland.system_api.sigma_std import SigmaCrypto

class SigmaGhostChat:
    def _peer_discovery(self):
        """USP: Passive Peer Discovery via Local Broadcast."""
        while self._running:
            payload = {'type': 'HELLO', 'sender': self.username}
            packet = {'payload': payload, 'signature': SigmaCrypto.sign(json.dumps(payload))}
            time.sleep(10)