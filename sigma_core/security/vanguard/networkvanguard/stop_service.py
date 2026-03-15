# Generated method: NetworkVanguard.stop_service
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService
from .traffic_policer import TrafficPolicer
from .anonymity_shield import AnonymityShield

class NetworkVanguard:
    def stop_service(self):
        self._running = False