"""
SigmaOS Peer Management Module
==============================
Handles peer discovery and directory services for the Sovereign Mesh.
"""
from typing import Dict

class PeerDirectory:
    def __init__(self):
        self._peers: Dict[str, Dict] = {}

    def register_peer(self, sid: str, ip: str, port: int, alias: str = "Unknown"):
        self._peers[sid] = {
            "ip": ip,
            "port": port,
            "alias": alias,
            "last_seen": 0, # Should be timestamp
            "status": "ONLINE"
        }

    def get_peer(self, sid: str):
        return self._peers.get(sid)

    def remove_peer(self, sid: str):
        if sid in self._peers:
            del self._peers[sid]

    def all_peers(self):
        return self._peers
