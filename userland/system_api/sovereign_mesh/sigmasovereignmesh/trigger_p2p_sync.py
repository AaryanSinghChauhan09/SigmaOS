# Generated method: SigmaSovereignMesh.trigger_p2p_sync
from .sovereign_mesh_drive import SigmaSovereignMeshDrive

class SigmaSovereignMesh:
    def trigger_p2p_sync(self) -> dict:
        """Start a decentralized sync between trusted Sigma nodes."""
        return self._drive.trigger_p2p_sync()