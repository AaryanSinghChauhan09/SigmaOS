# Generated method: SovereignChatEngine.join_channel
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

class SovereignChatEngine:
    def join_channel(self, channel_id: str):
        return ChatActions.join_channel(self, channel_id)