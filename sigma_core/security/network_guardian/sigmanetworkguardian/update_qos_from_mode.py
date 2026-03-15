# Generated method: SigmaNetworkGuardian.update_qos_from_mode
import time
import threading
from typing import Dict, List, Any

class SigmaNetworkGuardian:
    def update_qos_from_mode(self, mode_name: str, mode_config: Dict):
        """Called by ModeManager to adjust network Quality of Service."""
        flags = mode_config.get('Kernel_Flags', [])
        if 'network-latency-low' in flags or 'game-mode-boost' in flags:
            self._current_qos = 'Ultra-Low-Latency Mode (BBRv2 + DSCP EF)'
        elif 'network-qos-high' in flags:
            self._current_qos = 'High-Throughput (Max Window Size)'
        elif 'network-vpn-forced' in flags:
            self._current_qos = 'Strict VPN / Tor Only'
        elif 'airplane-mode' in flags:
            self._current_qos = 'Air-Gapped (All Drops)'
            self._active = False
        else:
            self._current_qos = 'Balanced Sovereign'
            self._active = True
        self.kernel.bus.emit('net.qos_updated', {'qos': self._current_qos})