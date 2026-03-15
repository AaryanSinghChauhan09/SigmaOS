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
    def start_service(self):
        """Initializes the P2P listener."""
        if not self._running:
            self._running = True
            self._server_thread = threading.Thread(target=self._listen_for_peers, daemon=True)
            self._server_thread.start()
            self._discovery_thread = threading.Thread(target=self._peer_discovery, daemon=True)
            self._discovery_thread.start()
            self.log_event('ghostchat_init', {'node_id': self.username})
            return f'GhostChat Sovereign Node [{self.username}] active on port {self.port}.'
