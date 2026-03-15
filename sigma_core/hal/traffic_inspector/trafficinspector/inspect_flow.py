# Generated method: TrafficInspector.inspect_flow
from typing import Dict, Any

class TrafficInspector:
    def inspect_flow(self, packet_meta: Dict[str, Any]) -> bool:
        """Analyzes packet headers for anomalies or data leaks."""
        origin = packet_meta.get('origin', 'unknown')
        size = packet_meta.get('size', 0)
        if size > 1024 * 1024:
            self.stats['dropped'] += 1
            return False
        self.stats['bytes_in'] += size
        return True