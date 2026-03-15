"""
SigmaOS Sovereign Chat Engine (v2.0 Apex)
=========================================
USP: Zero-Knowledge P2P Messaging | E2EE | Metadata Shredding.
Modular architecture fulfilling: Abstraction, High Cohesion, Security-First.
"""

import socket
import threading
import time
import json
import uuid
import os
import hashlib
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from userland.system_api.sigma_std import SigmaCrypto
from .identity import ChatIdentity
from .protocol import SecurePacket
from .peers import PeerDirectory
from .network import MeshSocket
from .engine_shards.ops import ChatOps
from .engine_shards.networking import ChatNet
from .engine_shards.logic import ChatLogic
from .engine_shards.actions import ChatActions

class SovereignChatEngine(SigmaModuleBase, ISigmaService):
    """
    Sovereign Comm-Layer. The most secured P2P chat engine in existence.
    """
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.port = 20910
        self.identity = ChatIdentity()
        self.peer_dir = PeerDirectory()
        self.net = MeshSocket(self.port)
        self.inbox: List[Dict] = []
        self._running = False
        
        # Fixing slicing lint by explicit casting
        raw_hash = str(hashlib.sha256(b"SIGMA_PROTO_V3").hexdigest())
        self.network_hash = raw_hash[:8]
        
        self.stats = {
            "packets_encrypted": 0,
            "packets_decrypted": 0,
            "shredded_metadata_kb": 0.0,
            "pow_verifications": 0,
            "channel_broadcasts": 0,
            "active_tunnels": 0
        }

    def start_service(self):
        return ChatOps.start_engine(self)

    def stop_service(self):
        ChatOps.stop_engine(self)

    def _secure_listener(self):
        ChatNet.secure_listener(self)

    def _handle_secure_conn(self, conn, addr):
        ChatNet.handle_conn(self, conn, addr)

    def _process_handshake(self, packet: Dict, ip: str):
        ChatLogic.process_handshake(self, packet, ip)

    def _process_encrypted_message(self, packet: Dict):
        ChatLogic.process_message(self, packet)

    def _socket_send(self, ip: str, packet: Dict):
        ChatNet.dispatch_packet(self, ip, packet)

    def _purge_volatile_memory(self):
        ChatOps.purge_memory(self)
        
    def join_channel(self, channel_id: str):
        return ChatActions.join_channel(self, channel_id)

    def send_channel_msg(self, channel_id: str, text: str):
        return ChatActions.send_channel_msg(self, channel_id, text)

    def switch_alias(self, new_alias: str):
        return ChatActions.switch_alias(self, new_alias)

    def send_broadcast(self, text: str):
        return ChatActions.send_broadcast(self, text)

    def health_check(self) -> dict:
        return {
            "status": "ONLINE" if self._running else "OFFLINE",
            "sid": self.identity.sid,
            "alias": self.identity.alias,
            "peers": self.peer_dir.count_peers(),
            "stats": self.stats
        }
