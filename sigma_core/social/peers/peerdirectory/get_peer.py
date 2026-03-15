# Generated method: PeerDirectory.get_peer
from typing import Dict, Any

class PeerDirectory:
    def get_peer(self, sid: str) -> Any:
        return self._peers.get(sid)