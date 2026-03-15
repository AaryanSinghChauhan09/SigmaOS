# Generated method: SigmaGhostChat.purge_chat_history
import os
import sys
import hashlib
import json
import time
import socket
import threading
from typing import Dict, List, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaGhostChat:
    def purge_chat_history(self) -> int:
        """Forensic wipe of all message buffers."""
        count = len(self.messages)
        self.messages = []
        return count