"""
Auto-split from ecosystem\pdf_forge.py — SigmaPDFForge.load_document
"""

from sigma_core.system.sovereign_app import SovereignApp



class SigmaPDFForge:
    def load_document(self, path):
        self.active_document = path
        return f"PDF Forge: Loaded '{path}' into high-speed rendering buffer."
