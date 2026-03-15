# Generated method: SigmaOfflineGuard.__init__
import socket
import hashlib
import time

class SigmaOfflineGuard:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self._independence_score = 100.0
        self._blocked_outbound = 0
        self._stats = {'local_models_active': True, 'external_telemetry_disabled': True, 'p2p_discovery_only': True, 'app_sovereignty_enforced': True}
        self._sovereign_userland_apps = ['pdf_forge', 'titan_capture', 'sigma_browser', 'sigma_studio', 'sigma_lab', 'sigma_data_pro', 'omni_converter', 'aether_orchestrator']