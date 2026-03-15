# Generated method: SigmaMediaStudio.add_layer
import time
import os
import uuid

class SigmaMediaStudio:
    def add_layer(self, layer_name: str) -> str:
        """Photoshop-style non-destructive layer editing using open-source ImageMagick analogs."""
        if not self.active_project:
            return 'Error: No active project.'
        self.layers.append({'name': layer_name, 'visible': True, 'opacity': 100})
        self._record_state(f"Added Layer '{layer_name}'")
        return f"Added new non-destructive layer: '{layer_name}'."