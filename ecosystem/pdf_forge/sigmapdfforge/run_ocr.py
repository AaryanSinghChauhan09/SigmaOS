"""
Auto-split from ecosystem\pdf_forge.py — SigmaPDFForge.run_ocr
"""

from sigma_core.system.sovereign_app import SovereignApp



class SigmaPDFForge:
    def run_ocr(self, language='English'):
        """High-accuracy OCR using LOCAL neural models. 100% Offline."""
        self.is_ocr_active = True
        return self._call_service('AI_Engine', 'OCR_Scan', lang=language)
