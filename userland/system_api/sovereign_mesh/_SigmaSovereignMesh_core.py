# Generated class core: SigmaSovereignMesh
from .sovereign_mesh_drive import SigmaSovereignMeshDrive

class SigmaSovereignMesh:
    """High-level interface for the Sovereign Mesh subsystem.

    The kernel expects a class named ``SigmaSovereignMesh`` with a constructor
    accepting the kernel instance. Internally we delegate to ``SigmaSovereignMeshDrive``
    which contains the actual implementation.
    """