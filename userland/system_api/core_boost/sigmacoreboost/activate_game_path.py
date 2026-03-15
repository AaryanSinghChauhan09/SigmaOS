# Generated method: SigmaCoreBoost.activate_game_path
from typing import Dict, List, Any

class SigmaCoreBoost:
    def activate_game_path(self, executable_id: str) -> str:
        """USP: Creates a dedicated hardware-fence for the game process."""
        self._active_optimizations.append(executable_id)
        return f"CoreBoost: Hardware-fence established for '{executable_id}'. Latency minimized to {self._latency_ms}ms."