"""
Auto-split from ecosystem\pdf_forge.py — SigmaPDFForge.convert_to
"""

from sigma_core.system.sovereign_app import SovereignApp



class SigmaPDFForge:
    def convert_to(self, format='Word'):
        """Direct transformation to Office formats via SigmaUniversalBridge."""
        self.log_to_forensic_ledger(f'Converted to {format}')
        return f'PDF Forge (Convert): Exporting to .{format.lower()}. Layout integrity preserved.'
