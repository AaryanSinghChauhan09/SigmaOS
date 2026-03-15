# Generated method: ChatLogic.process_handshake
import time
from typing import Dict
from userland.system_api.sigma_std import SigmaCrypto

class ChatLogic:
    @staticmethod
    def process_handshake(engine, packet: Dict, ip: str):
        peer_sid = packet.get('sid')
        peer_pub = packet.get('pub_key') or packet.get('alias')
        shared_secret = SigmaCrypto.generate_shared_secret(engine.identity.keys, peer_pub.encode() if isinstance(peer_pub, str) else peer_pub)
        engine.peer_dir.add_peer(peer_sid, ip, shared_secret)
        engine.stats['active_tunnels'] = engine.peer_dir.count_peers()
        engine.log_event('PEER_SYNC', {'sid': peer_sid, 'status': 'TUNNEL_ESTABLISHED'})