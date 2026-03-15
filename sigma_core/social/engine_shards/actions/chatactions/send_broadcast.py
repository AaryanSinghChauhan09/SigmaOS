# Generated method: ChatActions.send_broadcast
from ..protocol import SecurePacket

class ChatActions:
    @staticmethod
    def send_broadcast(engine, text: str):
        packet = {'type': 'HANDSHAKE', 'sid': engine.identity.sid, 'alias': engine.identity.alias, 'proto': engine.network_hash}
        engine._socket_send('255.255.255.255', packet)
        return 'Discovery Handshake Dispatched'