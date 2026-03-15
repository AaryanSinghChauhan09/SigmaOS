"""
SigmaOS Sovereign Chat - User Actions Shard
===========================================
Handles high-level user actions like channel management and identity shifts.
"""
from ..protocol import SecurePacket

class ChatActions:
    @staticmethod
    def join_channel(engine, channel_id: str):
        if channel_id not in engine.identity.joined_channels:
            engine.identity.joined_channels.append(channel_id)
            engine.log_event("CHANNEL_JOIN", {"chan": channel_id})
            return f"Joined Stealth Channel: #{channel_id}"
        return "Already joined."

    @staticmethod
    def send_channel_msg(engine, channel_id: str, text: str):
        if channel_id not in engine.identity.joined_channels:
            return "ERR: Not in channel."
            
        packet = SecurePacket.construct(
            "CHANNEL_MSG",
            engine.identity.sid,
            f"CHAN:{channel_id}|{text}".encode(),
            engine.identity.keys
        )
        
        # Fixed: using peer_dir
        for peer_sid, info in engine.peer_dir.all_peers().items():
            engine._socket_send(info["ip"], packet)
        
        engine.stats["channel_broadcasts"] += 1
        return "Broadcasting to mesh..."

    @staticmethod
    def switch_alias(engine, new_alias: str):
        engine.identity.alias = new_alias
        engine._purge_volatile_memory()
        return f"Identity shifted to '{new_alias}'. Session keys rotated."

    @staticmethod
    def send_broadcast(engine, text: str):
        packet = {
            "type": "HANDSHAKE",
            "sid": engine.identity.sid,
            "alias": engine.identity.alias,
            "proto": engine.network_hash
        }
        engine._socket_send("255.255.255.255", packet)
        return "Discovery Handshake Dispatched"
