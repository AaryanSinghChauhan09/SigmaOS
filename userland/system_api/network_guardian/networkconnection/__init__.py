# Generated method: NetworkConnection.__init__
import time
import threading
from typing import Dict, List, Any

class NetworkConnection:
    def __init__(self, pid: str, dest_ip: str, dest_port: int, domain: str=''):
        self.pid = pid
        self.dest_ip = dest_ip
        self.dest_port = dest_port
        self.domain = domain
        self.bytes_sent = 0
        self.bytes_recv = 0
        self.status = 'ESTABLISHED'
        self.blocked = False