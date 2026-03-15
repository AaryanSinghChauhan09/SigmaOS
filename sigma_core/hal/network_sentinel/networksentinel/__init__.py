# Generated method: NetworkSentinel.__init__
from typing import Dict, Any
from .traffic_inspector import TrafficInspector
from .encryption_shield import EncryptionShield

class NetworkSentinel:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.inspector = TrafficInspector(kernel)
        self.shield = EncryptionShield(kernel)