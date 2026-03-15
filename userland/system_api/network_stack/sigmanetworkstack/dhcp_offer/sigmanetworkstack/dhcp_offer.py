# Generated method: SigmaNetworkStack.dhcp_offer
import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaNetworkStack:
    def dhcp_offer(self) -> dict:
        proposed_ip = '10.0.2.15'
        return {'status': 'OFFER', 'yiaddr': proposed_ip, 'siaddr': '10.0.2.2', 'message': f'DHCP: Server offered IP {proposed_ip}. Sending REQUEST...'}