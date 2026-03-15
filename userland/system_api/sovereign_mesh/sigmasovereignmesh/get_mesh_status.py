# Generated method: SigmaSovereignMesh.get_mesh_status
from .sovereign_mesh_drive import SigmaSovereignMeshDrive

class SigmaSovereignMesh:
    def get_mesh_status(self) -> dict:
        """Return current mesh status information."""
        return self._drive.get_mesh_status()