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
        self.sid = f"SID-{uuid.uuid4().hex[:8].upper()}"
        self.keys = os.urandom(32) # Ephemeral Root Key
        self.joined_channels: list[str] = []
