# Generated method: ChatActions.join_channel
from ..protocol import SecurePacket

class ChatActions:
    @staticmethod
    def join_channel(engine, channel_id: str):
        if channel_id not in engine.identity.joined_channels:
            engine.identity.joined_channels.append(channel_id)
            engine.log_event('CHANNEL_JOIN', {'chan': channel_id})
            return f'Joined Stealth Channel: #{channel_id}'
        return 'Already joined.'