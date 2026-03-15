# Generated method: PeerDirectory.remove_peer
from typing import Dict, Any

class PeerDirectory:
    def remove_peer(self, sid: str):
        if sid in self._peers:
            del self._peers[sid]