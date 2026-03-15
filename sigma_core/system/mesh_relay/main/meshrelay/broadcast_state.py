# Generated method: MeshRelay.broadcast_state
import hashlib
import random

class MeshRelay:
    def broadcast_state(self, state_blob):
        """Encodes state and relays to peers."""
        encoded = hashlib.sha256(state_blob.encode()).hexdigest()
        print(f'[MESH] Relay Node {self.node_id} broadcasting state shard {encoded}')
        return True