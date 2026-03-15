"""
SigmaOS Peer Management Module
==============================
Handles peer discovery and directory services for the Sovereign Mesh.
"""
from typing import Dict, Any

class PeerDirectory:
    def __init__(self):
        self._peers: Dict[str, Dict[str, Any]] = {}

    def add_peer(self, sid: str, ip: str, shared_secret: bytes, alias: str = "Unknown"):
        self._peers[sid] = {
            "ip": ip,
            "shared_secret": shared_secret,
            "alias": alias,
            "last_seen": 0, # Should be timestamp
            "status": "ONLINE"
        }

    def get_peer(self, sid: str) -> Any:
        return self._peers.get(sid)

    def count_peers(self) -> int:
        return len(self._peers)

    def remove_peer(self, sid: str):
        if sid in self._peers:
            del self._peers[sid]

    def all_peers(self):
        return self._peers
