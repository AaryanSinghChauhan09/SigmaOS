"""
SigmaOS Sovereign Identity Module
=================================
Manages SID generation and context-switching aliases.
"""
import uuid
import os

class ChatIdentity:
    """Manages Sovereign SID and ephemeral aliases."""
    def __init__(self, alias: str = "Sigma_User"):
        self.alias = alias
        u_hex = str(uuid.uuid4().hex)
        self.sid = f"SID-{u_hex[:8].upper()}"
        self.keys = os.urandom(32) # Ephemeral Root Key
        self.joined_channels: list[str] = []
