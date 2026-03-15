"""
Auto-split from ecosystem\bharat_law_bridge.py — SigmaBharatLawBridge.add_client
"""

from typing import Dict, List, Any, Optional
import datetime



class SigmaBharatLawBridge:
    def add_client(self, client_id: str, name: str):
        self._clients[client_id] = {'name': name, 'matters': []}
        return f"Client '{name}' added to SigmaLegalPro database."
