# Generated method: ChatIdentity.__init__
import uuid
import os

class ChatIdentity:
    def __init__(self, alias: str='Sigma_User'):
        self.alias = alias
        u_hex = str(uuid.uuid4().hex)
        self.sid = f'SID-{u_hex[:8].upper()}'
        self.keys = os.urandom(32)
        self.joined_channels: list[str] = []