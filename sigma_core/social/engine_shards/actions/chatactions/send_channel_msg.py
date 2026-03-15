# Generated method: ChatActions.send_channel_msg
from ..protocol import SecurePacket

class ChatActions:
    @staticmethod
    def send_channel_msg(engine, channel_id: str, text: str):
        if channel_id not in engine.identity.joined_channels:
            return 'ERR: Not in channel.'
        packet = SecurePacket.construct('CHANNEL_MSG', engine.identity.sid, f'CHAN:{channel_id}|{text}'.encode(), engine.identity.keys)
        for peer_sid, info in engine.peer_dir.all_peers().items():
            engine._socket_send(info['ip'], packet)
        engine.stats['channel_broadcasts'] += 1
        return 'Broadcasting to mesh...'