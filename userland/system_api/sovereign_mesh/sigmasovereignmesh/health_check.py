# Generated method: SigmaSovereignMesh.health_check
from .sovereign_mesh_drive import SigmaSovereignMeshDrive

class SigmaSovereignMesh:
    def health_check(self) -> str:
        """Perform a health check of the mesh subsystem."""
        return self._drive.health_check()