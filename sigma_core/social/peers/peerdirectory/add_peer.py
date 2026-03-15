# Generated method: PeerDirectory.add_peer
from typing import Dict, Any

class PeerDirectory:
    def add_peer(self, sid: str, ip: str, shared_secret: bytes, alias: str='Unknown'):
        self._peers[sid] = {'ip': ip, 'shared_secret': shared_secret, 'alias': alias, 'last_seen': 0, 'status': 'ONLINE'}