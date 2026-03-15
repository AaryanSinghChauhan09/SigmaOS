# Generated method: NetworkVanguard.__init__
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from .traffic_policer import TrafficPolicer
from .anonymity_shield import AnonymityShield

class NetworkVanguard:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.policer = TrafficPolicer(kernel)
        self.anonymity = AnonymityShield(kernel)
        self._running = False