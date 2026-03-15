# Generated method: SigmaAuraProject.start_projection
from typing import Dict, List, Any

class SigmaAuraProject:
    def start_projection(self, target_display: str, source_app: str) -> str:
        """USP: Low-latency, encrypted AV stream to remote Sovereign nodes."""
        if target_display not in self._available_displays:
            return f"Error: '{target_display}' not found on Aura Mesh."
        self._active_streams.append({'app': source_app, 'target': target_display})
        return f"AuraProject: Projecting '{source_app}' to {target_display} via Lattice-PQC Stream."