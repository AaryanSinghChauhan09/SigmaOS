"""
Sigma Sovereign Mesh module providing P2P mesh functionality.
This module defines the SigmaSovereignMesh class, which wraps the existing
SigmaSovereignMeshDrive implementation to expose a clean API for the kernel.
"""

from .sovereign_mesh_drive import SigmaSovereignMeshDrive


class SigmaSovereignMesh:
    """High-level interface for the Sovereign Mesh subsystem.

    The kernel expects a class named ``SigmaSovereignMesh`` with a constructor
    accepting the kernel instance. Internally we delegate to ``SigmaSovereignMeshDrive``
    which contains the actual implementation.
    """

    def __init__(self, kernel):
        # Initialise the underlying drive implementation
        self._drive = SigmaSovereignMeshDrive(kernel)

    # Proxy methods – these simply forward calls to the drive instance.
    def trigger_p2p_sync(self) -> dict:
        """Start a decentralized sync between trusted Sigma nodes."""
        return self._drive.trigger_p2p_sync()

    def encrypt_and_vault(self, file_path: str) -> dict:
        """Encrypt a file and store it in the sovereign mesh vault."""
        return self._drive.encrypt_and_vault(file_path)

    def get_mesh_status(self) -> dict:
        """Return current mesh status information."""
        return self._drive.get_mesh_status()

    def health_check(self) -> str:
        """Perform a health check of the mesh subsystem."""
        return self._drive.health_check()
