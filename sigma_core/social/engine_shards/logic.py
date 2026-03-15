"""
SigmaOS Sovereign Chat - Logic Shard
====================================
Handles packet processing, decryption, and peer synchronization.
"""
import time
from typing import Dict
from userland.system_api.sigma_std import SigmaCrypto

class ChatLogic:
    @staticmethod
    def process_handshake(engine, packet: Dict, ip: str):
        peer_sid = packet.get("sid")
        peer_pub = packet.get("pub_key") or packet.get("alias") # Fallback to alias if pub_key missing
        
        shared_secret = SigmaCrypto.generate_shared_secret(engine.identity.keys, peer_pub.encode() if isinstance(peer_pub, str) else peer_pub)
        
        # Fixed: using peer_dir
        engine.peer_dir.add_peer(peer_sid, ip, shared_secret)
        engine.stats["active_tunnels"] = engine.peer_dir.count_peers()
        engine.log_event("PEER_SYNC", {"sid": peer_sid, "status": "TUNNEL_ESTABLISHED"})

    @staticmethod
    def process_message(engine, packet: Dict):
        sender_sid = packet.get("from")
        # Fixed: using peer_dir
        peer_info = engine.peer_dir.get_peer(sender_sid)
        if not peer_info: return
            
        enc_payload = bytes.fromhex(str(packet.get("payload", "")))
        decrypted_text = SigmaCrypto.decrypt_payload(enc_payload, peer_info["shared_secret"])
        
        if not decrypted_text.startswith("DEC_ERR"):
            msg_obj = {
                "sid": sender_sid,
                "text": decrypted_text,
                "time": time.time(),
                "verified": True
            }
            engine.inbox.append(msg_obj)
            engine.stats["packets_decrypted"] += 1
            if engine.kernel:
                engine.kernel.bus.publish("social.chat.msg", msg_obj)
