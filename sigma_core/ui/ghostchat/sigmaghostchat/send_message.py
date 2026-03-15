# auto-split module

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
    def send_message(self, text: str, peer_id: Optional[str]=None):
        """USP: Blind Routing. If peer_id is none, it broadcasts to all known peers."""
        payload = {'type': 'MSG', 'sender': self.username, 'text': text, 'timestamp': time.time(), 'ghost': True}
        packet = {'payload': payload, 'signature': SigmaCrypto.sign(json.dumps(payload))}
        targets = [peer_id] if peer_id else list(self.peers.keys())
        for tid in targets:
            ip = self.peers.get(tid)
            if ip:
                self._dispatch_packet(ip, packet)
        self.stats['messages_sent'] += 1
        return f'Message dispatched to {len(targets)} peers.'
